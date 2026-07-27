//! Dynamic action command handler.
//! Looks up a YAML-defined action by name and runs it with the given arguments.

use clap::ArgMatches;
use inquire::Confirm;

use crate::configs::app::AppConfig;
use crate::engine::engine::Engine;
use crate::output::format::FormatOutput;
use crate::output::output::{OutputKind, OutputType};
use crate::{print_template, print_text, utils};

/// Execute a dynamic action command.
pub async fn execute(name: &str, action_matches: &ArgMatches, config: &AppConfig) {
    let start_time = std::time::Instant::now();

    let flow = config
        .find_flow(name)
        .unwrap_or_else(|e| {
            print_text!(OutputKind::Error, "{}", e);
            std::process::exit(1);
        })
        .apply_system_tags()
        .unwrap_or_else(|e| {
            print_text!(OutputKind::Error, "{}", e);
            std::process::exit(1);
        })
        .apply_args(action_matches)
        .unwrap_or_else(|e| {
            print_text!(OutputKind::Error, "{}", e);
            std::process::exit(1);
        });

    let mut engine = Engine::new(&config.action.system, config.action.retries, &flow)
        .unwrap_or_else(|e| {
            print_text!(OutputKind::Error, "{}", e);
            std::process::exit(1);
        });

    let actions = engine.actions().to_vec();
    let total = actions.len();

    print_template!(
        OutputKind::Info,
        "Found action '{name}' ({steps} steps), starting...",
        "name" => flow.name,
        "steps" => total
    );

    // Warn if flow uses roles not available in cluster
    if config.check_role_mismatch(&flow) {
        let ans = Confirm::new("Continue with available nodes?")
        .with_placeholder("\nFlow has actions with roles not found in cluster. All available nodes will be used.")
        .with_default(false)
        .prompt();
        match ans {
            Ok(true) => {}
            Ok(false) => return,
            Err(_) => return,
        }
    }

    print_text!(OutputKind::Debug, "Flow: {} ({} steps)", flow.name, total);

    // Execute all actions.
    for (i, action) in actions.iter().enumerate() {
        print_template!(
            OutputKind::Debug,
            "[{current}/{total}] Running: {tag}",
            "current" => (i + 1).to_string(),
            "total" => total.to_string(),
            "tag" => action.tag
        );

        print_template!(
            OutputKind::Progress,
            "{tag} ({run})... {percent}% ({current}/{total})",
            "tag" => action.tag,
            "run" => action.run.to_string(),
            "percent" => format!("{:.0}", ((i + 1) as f32 / total as f32) * 100.0),
            "current" => (i + 1).to_string(),
            "total" => total.to_string()
        );

        if action.confirm && AppConfig::output().output_type() == OutputType::Cli {
            print_template!(
                OutputKind::Info,
                "completed in {duration}",
                "duration" => utils::format::format_duration(start_time.elapsed())
            );
            let query = format!("Execute '{}'?", FormatOutput::format_msg(&action.tag));
            let resolve = engine.action_display(&action).unwrap_or_else(|e| {
                print_text!(OutputKind::Error, "{}", e);
                std::process::exit(1);
            });

            let ans = Confirm::new(&query)
                .with_default(false)
                .with_placeholder(&format!("\n{}", resolve))
                .prompt();
            match ans {
                Ok(true) => {
                    engine.exec_action(action).await.unwrap_or_else(|e| {
                        print_text!(OutputKind::Error, "{}", e);
                        std::process::exit(1);
                    });
                }
                Ok(false) => return,
                Err(_) => return,
            }
        } else {
            engine.exec_action(action).await.unwrap_or_else(|e| {
                print_text!(OutputKind::Error, "{}", e);
                std::process::exit(1);
            });
        }
    }

    print_template!(
        OutputKind::Debug,
        "Flow completed: {name}",
        "name" => flow.name
    );

    let result = engine.result().unwrap_or_else(|e| {
        print_text!(OutputKind::Error, "{}", e);
        std::process::exit(1);
    });

    print_template!(
        OutputKind::Info,
        "completed in {duration}",
        "duration" => utils::format::format_duration(start_time.elapsed())
    );

    print_template!(
        ExportContext::Success,
        OutputKind::Success,
        "{message}",
        "message" => if result.is_empty() { "No matches found." } else { &result },
    );

    if flow.notify && AppConfig::output().output_type() == OutputType::Cli {
        #[cfg(target_os = "macos")]
        {
            match std::process::Command::new("terminal-notifier")
                .args(&[
                    "-title",
                    utils::app::app_name_pretty(),
                    "-message",
                    &format!(
                        "{} completed in {}",
                        flow.name,
                        utils::format::format_duration(start_time.elapsed())
                    ),
                ])
                .spawn()
            {
                Ok(_) => {}
                Err(_) => {
                    print_text!(
                        OutputKind::Warning,
                        "terminal-notifier not found. Install: brew install terminal-notifier"
                    );
                }
            }
        }
        #[cfg(target_os = "linux")]
        {
            let _ = notify_rust::Notification::new()
                .summary(utils::app::app_name_pretty())
                .body(&format!(
                    "{} completed in {}",
                    flow.name,
                    utils::format::format_duration(start_time.elapsed())
                ))
                .show();
        }
    }
}
