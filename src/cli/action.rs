//! Dynamic action command handler.
//! Looks up a YAML-defined action by name and runs it with the given arguments.

use std::collections::HashMap;

use clap::ArgMatches;

use crate::{
    configs::app::AppConfig, engine::engine::Engine, exit_error, print_newline, print_progress,
    print_success, utils,
};

/// Execute a dynamic action command.
pub async fn execute(cmd_name: &str, action_matches: &ArgMatches, config: &AppConfig) {
    let actions_model = match &config.actions_model {
        Some(m) => m,
        None => {
            exit_error!("No actions loaded.");
        }
    };

    let flow_ref = match actions_model.find(cmd_name) {
        Some(f) => f,
        None => {
            exit_error!("Unknown action: {}", cmd_name);
        }
    };

    // Collect CLI arguments and resolve path-like values.
    let mut args_map = HashMap::new();
    for arg_def in &flow_ref.args {
        if let Some(val) = action_matches.get_one::<String>(&arg_def.name) {
            let path_keys = ["path", "file", "dir"];
            let resolved = if path_keys.contains(&arg_def.name.as_str()) {
                utils::path::resolve(val)
                    .map(|p| p.display().to_string())
                    .unwrap_or_else(|_| val.clone())
            } else {
                val.clone()
            };
            args_map.insert(arg_def.name.clone(), resolved);
        }
    }

    // @todo
    let mut flow = flow_ref.clone();
    flow.apply_args(&args_map);

    // @todo
    match Engine::run(
        &flow,
        Some(|p| {
            print_progress!(
                "{} ({})... {:.0}% ({}/{})",
                p.tag,
                p.step_type,
                p.percent,
                p.current,
                p.total
            );
            if p.current == p.total {
                print_newline!();
            }
        }),
    )
    .await
    {
        Ok(result) => {
            if result.is_empty() {
                print_success!("Done.");
            } else {
                print_success!("{}", result);
            }
        }
        Err(e) => {
            print_newline!();
            exit_error!("{}", e)
        }
    };
}
