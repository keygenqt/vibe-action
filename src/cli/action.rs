//! Dynamic action command handler.
//! Looks up a YAML-defined action by name and runs it with the given arguments.

use clap::ArgMatches;
use inquire::Confirm;

use crate::{
    configs::app::AppConfig, engine::engine::Engine, exit_error, models::flow::FlowMode,
    print_info, print_newline, print_progress, print_success, print_success_block,
};

/// Execute a dynamic action command.
pub async fn execute(name: &str, action_matches: &ArgMatches, config: &AppConfig) {
    let flow = config
        .find_flow(name)
        .unwrap_or_else(|e| exit_error!("{}", e))
        .apply_args(action_matches);

    let mut engine = Engine::new(&flow).unwrap_or_else(|e| exit_error!("{}", e));
    let actions = engine.actions().to_vec();
    let total = actions.len() + 1;

    tracing::info!("Flow: {} ({} steps)", flow.name, actions.len());

    // Execute all intermediate actions.
    for (i, action) in actions.iter().enumerate() {
        tracing::debug!("[{}/{}] Running: {}", i + 1, actions.len(), action.tag);
        print_progress!(
            "{} ({})... {:.0}% ({}/{})",
            action.tag,
            action.r#type.to_string(),
            ((i + 1) as f32 / total as f32) * 100.0,
            i + 1,
            total
        );
        engine.exec_action(action).await.unwrap_or_else(|e| {
            print_newline!();
            exit_error!("{}", e)
        });
    }

    tracing::info!("Flow completed: {}", engine.trigger().tag);

    // Trigger progress.
    print_progress!(
        "{} (trigger)... 100% ({}/{})",
        engine.trigger().tag,
        total,
        total
    );
    print_newline!();

    match flow.mode {
        FlowMode::Output => {
            print_success_block!("{}", engine.trigger_fill());
        }
        FlowMode::Exec => {
            let trigger = engine.trigger().clone();
            let result = engine
                .exec_action(&trigger)
                .await
                .unwrap_or_else(|e| exit_error!("{}", e));
            print_success!("{}", result);
        }
        FlowMode::Ask => {
            let text = engine.trigger_fill();
            let ans = Confirm::new("Execute this command?")
                .with_default(false)
                .with_placeholder(&format!("\n{}", text))
                .prompt();
            match ans {
                Ok(true) => {
                    let trigger = engine.trigger().clone();
                    let result = engine
                        .exec_action(&trigger)
                        .await
                        .unwrap_or_else(|e| exit_error!("{}", e));
                    print_success!("{}", result);
                }
                Ok(false) => print_info!("{}", text),
                Err(_) => {}
            }
        }
    }
}
