//! Dynamic action command handler.
//! Looks up a YAML-defined action by name and runs it with the given arguments.

use arboard::Clipboard;
use clap::ArgMatches;
use inquire::Confirm;

use crate::{
    configs::app::AppConfig, engine::engine::Engine, exit_error, models::flow::FlowFormat,
    print_info, print_newline, print_progress, print_rich_block, print_success, print_warning,
};

/// Execute a dynamic action command.
pub async fn execute(name: &str, action_matches: &ArgMatches, config: &AppConfig) {
    let flow = config
        .find_flow(name)
        .unwrap_or_else(|e| exit_error!("{}", e))
        .apply_args(action_matches);

    let mut engine = Engine::new(&flow).unwrap_or_else(|e| exit_error!("{}", e));
    let actions = engine.actions().to_vec();
    let total = actions.len();

    tracing::info!("Flow: {} ({} steps)", flow.name, total);

    // Execute all actions.
    for (i, action) in actions.iter().enumerate() {
        tracing::debug!("[{}/{}] Running: {}", i + 1, total, action.tag);

        print_progress!(
            "{} ({})... {:.0}% ({}/{})",
            action.tag,
            action.r#type.to_string(),
            ((i + 1) as f32 / total as f32) * 100.0,
            i + 1,
            total
        );

        if action.confirm {
            print_newline!();
            let ans = Confirm::new(&format!("Execute '{}'?", action.tag))
                .with_default(false)
                .with_placeholder(&format!("\n{}", engine.action_display(&action)))
                .prompt();
            match ans {
                Ok(true) => {
                    print_newline!();
                    engine
                        .exec_action(action)
                        .await
                        .unwrap_or_else(|e| exit_error!("{}", e));
                }
                Ok(false) => return,
                Err(_) => return,
            }
        } else {
            engine.exec_action(action).await.unwrap_or_else(|e| {
                print_newline!();
                exit_error!("{}", e)
            });
            if i + 1 == total {
                print_newline!();
            }
        }
    }

    tracing::info!("Flow completed: {}", flow.name);

    let result = engine.result().unwrap_or_else(|e| exit_error!("{}", e));

    if flow.clipboard {
        if let Ok(mut clipboard) = Clipboard::new() {
            clipboard
                .set_text(&result)
                .unwrap_or_else(|e| print_warning!("{}", e));
            print_info!("Text copied to clipboard");
        }
    }

    match flow.format {
        FlowFormat::Compact => print_success!("{}", &result),
        FlowFormat::Rich => print_rich_block!("{}", &result),
    }
}
