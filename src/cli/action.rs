//! Dynamic action command handler.
//! See [`crate::cli`] module-level docs for the execution flow.

use clap::ArgMatches;
use inquire::Confirm;
use std::io::BufRead;
use tokio::sync::mpsc;

use crate::configs::app::AppConfig;
use crate::engine::engine::Engine;
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
pub async fn execute(
    group: Option<&str>,
    name: &str,
    action_matches: &ArgMatches,
    config: &AppConfig,
) {
    let is_output_cli = AppConfig::output().output_type() == OutputType::Cli;
    let is_output_json = AppConfig::output().output_type() == OutputType::Json;
    let is_output_test = AppConfig::output().output_type() == OutputType::Test;
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

    let mut pipeline = config
        .find_pipeline(group, name)
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
        })
        .apply_query_tags()
        .unwrap_or_else(|e| {
            print_text!(OutputKind::Error, "{}", e);
            std::process::exit(1);
        });

    // If pipeline uses query_prompt and no query was provided via CLI args, ask for input
    if !is_output_json && pipeline.needs_prompt() {
        let is_query_empty = pipeline
            .input_tags
            .get("query")
            .map(|s| s.is_empty())
            .unwrap_or(true);

        if is_query_empty {
            let ans = inquire::Text::new("Query").prompt();
            match ans {
                Ok(text) => {
                    pipeline.input_tags.insert("query".to_string(), text);
                }
                Err(_) => return,
            }
        }
    }

    pipeline.validate_query_tags().unwrap_or_else(|e| {
        print_text!(OutputKind::Error, "{}", e);
        std::process::exit(1);
    });

    let mut engine = Engine::new(&config.action.system, config.action.retries, &pipeline)
        .unwrap_or_else(|e| {
            print_text!(OutputKind::Error, "{}", e);
            std::process::exit(1);
        });

    print_template!(
        OutputKind::Info,
        "Found action '{name}', starting...",
        "name" => pipeline.name
    );

    // Warn if pipeline uses roles not available in cluster
    if config.check_role_mismatch(&pipeline) {
        if is_output_test {
            // Non-interactive run (CI): no confirm, proceed with the fallback.
            print_text!(
                OutputKind::Warning,
                "pipeline has actions with roles not found in cluster. All available nodes will be used."
            );
        } else if is_output_json {
            print_template!(
                ExportContext::Confirm,
                OutputKind::Warning,
                "{tag}",
                "tag" => "role_mismatch",
                "display" => "pipeline has actions with roles not found in cluster. All available nodes will be used.",
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
                .with_placeholder("\npipeline has actions with roles not found in cluster. All available nodes will be used.")
                .with_default(false)
                .prompt()
            {
                Ok(true) => {}
                Ok(false) => return,
                Err(e) => {
                    print_text!(OutputKind::Error, "Confirm failed: {}", e);
                    std::process::exit(1);
                }
            }
        }
    }

    print_text!(OutputKind::Debug, "pipeline: {}", pipeline.name);

    let mut result = String::new();
    while let Some(action) = engine.next().unwrap_or_else(|e| {
        print_text!(OutputKind::Error, "{}", e);
        std::process::exit(1);
    }) {
        print_template!(
            OutputKind::Progress,
            "{tag} ({run})...",
            "tag" => action.tag,
            "run" => action.run.to_string(),
        );

        let (items, merge_sep) = engine.expand(&action).unwrap_or_else(|e| {
            print_text!(OutputKind::Error, "{}", e);
            std::process::exit(1);
        });
        let mut results = Vec::with_capacity(items.len());
        for item in &items {
            if action.ask {
                let confirmed = if is_output_json {
                    print_template!(
                        ExportContext::Confirm,
                        OutputKind::Info,
                        "{tag}",
                        "tag" => &action.tag,
                        "display" => item,
                    );
                    match &mut stdin_rx {
                        Some(rx) => match tokio::time::timeout(CONFIRM_TIMEOUT, rx.recv()).await {
                            Ok(Some(line)) => line.trim() == "true",
                            _ => false,
                        },
                        None => false,
                    }
                } else {
                    let query = format!("Execute '{}'?", FormatOutput::format_msg(&action.tag));
                    match Confirm::new(&query)
                        .with_default(false)
                        .with_placeholder(&format!("\n{}", item))
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
                if !confirmed {
                    return;
                }
            }
            results.push(engine.exec_item(&action, item).await.unwrap_or_else(|e| {
                print_text!(OutputKind::Error, "{}", e);
                std::process::exit(1);
            }));
        }
        result = engine.store_result(&action.tag, results, &merge_sep);
    }

    print_template!(
        OutputKind::Debug,
        "pipeline completed: {name}",
        "name" => pipeline.name
    );

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

    if pipeline.notify && is_output_cli {
        #[cfg(target_os = "macos")]
        {
            match std::process::Command::new("terminal-notifier")
                .args(&[
                    "-title",
                    utils::app::app_name_pretty(),
                    "-message",
                    &format!(
                        "{} completed in {}",
                        pipeline.name,
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
                    pipeline.name,
                    utils::format::format_duration(start_time.elapsed())
                ))
                .show();
        }
    }
}
