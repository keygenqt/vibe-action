//! Dynamic action command handler.
//! Looks up a YAML-defined action by name and runs it with the given arguments.

use arboard::Clipboard;
use clap::ArgMatches;
use inquire::Confirm;

use crate::{
    configs::app::AppConfig,
    engine::engine::Engine,
    exit_error,
    output::{macros::format_msg, output::OutputLevel},
    print_debug, print_info, print_progress, print_success, print_warning, utils,
};

/// Execute a dynamic action command.
pub async fn execute(name: &str, action_matches: &ArgMatches, config: &AppConfig) {
    let start_time = std::time::Instant::now();

    let flow = config
        .find_flow(name)
        .unwrap_or_else(|e| exit_error!("{}", e))
        .apply_args(action_matches);

    let mut engine = Engine::new(&config.action.system, config.action.retries, &flow)
        .unwrap_or_else(|e| exit_error!("{}", e));
    let actions = engine.actions().to_vec();
    let total = actions.len();

    print_debug!("Flow: {} ({} steps)", flow.name, total);

    // Execute all actions.
    for (i, action) in actions.iter().enumerate() {
        print_debug!("[{}/{}] Running: {}", i + 1, total, action.tag);

        print_progress!(
            "{} ({})... {:.0}% ({}/{})",
            action.tag,
            action.run.to_string(),
            ((i + 1) as f32 / total as f32) * 100.0,
            i + 1,
            total
        );

        if action.confirm {
            print_info!("completed in {:.2?}", start_time.elapsed());
            let query = format!("Execute '{}'?", format_msg(&action.tag));
            let resolve = engine
                .action_display(&action)
                .unwrap_or_else(|e| exit_error!("{}", e));
            let ans = Confirm::new(&query)
                .with_default(false)
                .with_placeholder(&format!("\n{}", resolve))
                .prompt();
            match ans {
                Ok(true) => {
                    engine
                        .exec_action(action)
                        .await
                        .unwrap_or_else(|e| exit_error!("{}", e));
                }
                Ok(false) => return,
                Err(_) => return,
            }
        } else {
            engine
                .exec_action(action)
                .await
                .unwrap_or_else(|e| exit_error!("{}", e));
        }
    }

    print_debug!("Flow completed: {}", flow.name);
    let result = engine.result().unwrap_or_else(|e| exit_error!("{}", e));

    if flow.clipboard {
        if let Ok(mut clipboard) = Clipboard::new() {
            clipboard
                .set_text(&result)
                .unwrap_or_else(|e| print_warning!("{}", e));
            print_info!("Text copied to clipboard");
        }
    }

    print_info!("completed in {:.2?}", start_time.elapsed());

    if result.is_empty() {
        print_info!("No matches found.")
    } else {
        print_success!("{}", &result)
    }
    if flow.notify && AppConfig::output().level() == OutputLevel::Cli {
        #[cfg(target_os = "macos")]
        {
            match std::process::Command::new("terminal-notifier")
                .args(&[
                    "-title",
                    utils::app::app_name_pretty(),
                    "-message",
                    &format!("{} completed in {:.2?}", flow.name, start_time.elapsed()),
                ])
                .spawn()
            {
                Ok(_) => {}
                Err(_) => {
                    print_warning!(
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
                    "{} completed in {:.2?}",
                    flow.name,
                    start_time.elapsed()
                ))
                .show();
        }
    }
}
