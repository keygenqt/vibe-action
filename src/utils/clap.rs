//! CLI command dispatch and action execution.
//! Routes parsed matches to the appropriate handler: built-in commands or dynamic YAML actions.

use std::path::PathBuf;

use clap::{Arg, ColorChoice, Command};

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
            if !actions_model.flows.is_empty() {
                app = app.mut_subcommand("action", |mut action_subcommand| {
                    for flow in &actions_model.flows {
                        let mut dynamic_cmd = Command::new(flow.name.as_str()).about(&flow.about);
                        for arg_def in &flow.args {
                            let mut clap_arg = Arg::new(arg_def.name.as_str())
                                .long(arg_def.name.as_str())
                                .help(arg_def.help.as_deref().unwrap_or(""))
                                .value_parser(clap::value_parser!(String))
                                .required(arg_def.required.unwrap_or(false));

                            if let Some(short) = &arg_def.short {
                                if let Some(c) = short.chars().next() {
                                    clap_arg = clap_arg.short(c);
                                }
                            }
                            dynamic_cmd = dynamic_cmd.arg(clap_arg);
                        }

                        action_subcommand = action_subcommand.subcommand(dynamic_cmd);
                    }
                    action_subcommand
                });
            }
        }
        app
    }};
}

/// Print help for a specific subcommand and printing.
pub fn print_subcommand_help(app: &mut Command, subcommand_name: &str) {
    if let Some(sub) = app.find_subcommand_mut(subcommand_name) {
        let mut owned = sub
            .clone()
            .styles(crate::utils::app::app_styles())
            .color(ColorChoice::Always);
        let _ = owned.print_help();
    }
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
