//! CLI command dispatch and action execution.
//! Routes parsed matches to the appropriate handler: built-in commands or dynamic YAML actions.

use std::path::PathBuf;

use clap::{Arg, Command};

/// Builds the full hierarchical CLI command tree including dynamic YAML actions.
///
/// This macro dynamically extends the `action` subcommand with commands
/// loaded from the user's YAML action files at runtime.
#[macro_export]
macro_rules! build_app {
    ($config:expr) => {{
        use clap::{Arg, Command, CommandFactory};
        let mut app = $crate::App::command();
        if let Some(actions_model) = &$config.actions_model {
            for flow in &actions_model.flows {
                let mut dynamic_cmd = Command::new(flow.name.as_str()).about(&flow.about);
                for arg_def in &flow.args {
                    let clap_arg: Arg = arg_def.into();
                    dynamic_cmd = dynamic_cmd.arg(clap_arg);
                }
                app = app.subcommand(dynamic_cmd);
            }
        }
        app
    }};
}

/// Parse global flags (--config, --debug) before full CLI init.
pub fn parse_global_flags() -> (Option<PathBuf>, bool) {
    let raw_args: Vec<String> = std::env::args().collect();
    let global_matches = Command::new("vibe-action")
        .ignore_errors(true)
        .arg(
            Arg::new("config")
                .long("config")
                .value_parser(clap::value_parser!(PathBuf)),
        )
        .arg(
            Arg::new("debug")
                .long("debug")
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches_from(&raw_args);

    let config_path = global_matches.get_one::<PathBuf>("config").cloned();
    let debug = global_matches.get_flag("debug");
    (config_path, debug)
}
