//! Dynamic action command handler.
//! Looks up a YAML-defined action by name and runs it with the given arguments.

use clap::ArgMatches;
use inquire::Confirm;
use std::io::BufRead;
use tokio::sync::mpsc;

use crate::configs::app::AppConfig;
use crate::engine::engine::Engine;
use crate::models::context::ContextModel;
use crate::output::format::FormatOutput;
use crate::output::output::OutputKind;
use crate::output::output::OutputType;
use crate::print_template;
use crate::print_text;
use crate::utils;

/// Timeout for confirm responses from the IDE plugin (JSON mode).
/// Fail-closed: if the plugin doesn't respond, the step is declined.
const CONFIRM_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(30);

/// Execute a dynamic action command.
pub async fn execute(name: &str, action_matches: &ArgMatches, config: &AppConfig) {
    let is_output_cli = AppConfig::output().output_type() == OutputType::Cli;
    let is_output_json = AppConfig::output().output_type() == OutputType::Json;
    let start_time = std::time::Instant::now();

    // Start a dedicated stdin reader thread for the confirm protocol.
    // The thread reads lines and sends them to an mpsc channel; each
    // confirm call awaits the next line with a timeout (fail-closed).
    let mut stdin_rx = if is_output_json {
        let (tx, rx) = mpsc::channel::<String>(8);
        std::thread::spawn(move || {
            let stdin = std::io::stdin();
            loop {
                let mut line = String::new();
                match stdin.lock().read_line(&mut line) {
                    Ok(0) | Err(_) => break,
                    Ok(_) => {
                        if tx.blocking_send(line).is_err() {
                            break;
                        }
                    }
                }
            }
        });
        Some(rx)
    } else {
        None
    };

    let mut flow = config
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

    // If flow uses {query|prompt} and no query was provided via CLI args, ask for input
    if !is_output_json && flow.needs_prompt() {
        let is_query_empty = match flow.input_tags.get("query") {
            Some(ContextModel::String(s)) if !s.is_empty() => false,
            _ => true,
        };

        if is_query_empty {
            let ans = inquire::Text::new("Query").prompt();
            match ans {
                Ok(text) => {
                    flow.input_tags
                        .insert("query".to_string(), ContextModel::String(text));
                }
                Err(_) => return,
            }
        }
    }

    let flow = flow.apply_query_tags().unwrap_or_else(|e| {
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
        if is_output_json {
            print_template!(
                ExportContext::Confirm,
                OutputKind::Warning,
                "{tag}",
                "tag" => "role_mismatch",
                "display" => "Flow has actions with roles not found in cluster. All available nodes will be used.",
            );
            let confirmed = match &mut stdin_rx {
                Some(rx) => match tokio::time::timeout(CONFIRM_TIMEOUT, rx.recv()).await {
                    Ok(Some(line)) => line.trim() == "true",
                    _ => false,
                },
                None => false,
            };
            if !confirmed {
                return;
            }
        } else {
            match Confirm::new("Continue with available nodes?")
                .with_placeholder("\nFlow has actions with roles not found in cluster. All available nodes will be used.")
                .with_default(false)
                .prompt()
            {
                Ok(true) => {}
                Ok(false) => return,
                Err(e) => {
                    print_text!(OutputKind::Error, "Confirm failed: {}", e);
                    return;
                }
            }
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

        // Evaluate step-level "when" condition
        if !engine.check_when(action).unwrap_or_else(|e| {
            print_text!(OutputKind::Error, "{}", e);
            std::process::exit(1);
        }) {
            print_template!(
                OutputKind::Progress,
                "[{current}/{total}] Skipped: {tag} (condition false)",
                "current" => (i + 1).to_string(),
                "total" => total.to_string(),
                "tag" => action.tag
            );
            continue;
        }

        print_template!(
            OutputKind::Progress,
            "{tag} ({run})... {percent}% ({current}/{total})",
            "tag" => action.tag,
            "run" => action.run.to_string(),
            "percent" => format!("{:.0}", ((i + 1) as f32 / total as f32) * 100.0),
            "current" => (i + 1).to_string(),
            "total" => total.to_string()
        );

        if action.confirm {
            let resolve = engine.action_display(&action).unwrap_or_else(|e| {
                print_text!(OutputKind::Error, "{}", e);
                std::process::exit(1);
            });

            let confirmed = if is_output_json {
                print_template!(
                    ExportContext::Confirm,
                    OutputKind::Info,
                    "{tag}",
                    "tag" => &action.tag,
                    "display" => &resolve,
                );
                match &mut stdin_rx {
                    Some(rx) => match tokio::time::timeout(CONFIRM_TIMEOUT, rx.recv()).await {
                        Ok(Some(line)) => line.trim() == "true",
                        _ => false,
                    },
                    None => false,
                }
            } else {
                print_template!(
                    OutputKind::Info,
                    "completed in {duration}",
                    "duration" => utils::format::format_duration(start_time.elapsed())
                );
                let query = format!("Execute '{}'?", FormatOutput::format_msg(&action.tag));
                match Confirm::new(&query)
                    .with_default(false)
                    .with_placeholder(&format!("\n{}", resolve))
                    .prompt()
                {
                    Ok(true) => true,
                    Ok(false) => false,
                    Err(e) => {
                        print_text!(OutputKind::Error, "Confirm failed: {}", e);
                        false
                    }
                }
            };

            if confirmed {
                engine.exec_action(action).await.unwrap_or_else(|e| {
                    print_text!(OutputKind::Error, "{}", e);
                    std::process::exit(1);
                });
            } else {
                return;
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
        "message" => if result.is_empty() && is_output_cli { "No matches found." } else { &result },
    );

    if flow.notify && is_output_cli {
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
