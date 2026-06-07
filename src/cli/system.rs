//! System command handler.
//! Manages build, validate, and other maintenance subcommands.

use clap::{ArgMatches, FromArgMatches, Subcommand};
use std::path::PathBuf;

use crate::exit_error;

#[derive(Subcommand, Debug)]
pub enum SystemSubcommands {
    /// Build the CLI from actions — compile actions into SQLite cache
    Build,

    /// Validate configuration files
    Validate {
        /// Path to config file
        #[arg(long, short)]
        path: Option<PathBuf>,
    },
}

/// Execute the `system` command with the given parsed arguments.
pub async fn execute(matches: &ArgMatches) {
    let subcommand = match SystemSubcommands::from_arg_matches(matches) {
        Ok(cmd) => cmd,
        _ => exit_error!("Failed to parse system subcommand"),
    };
    match subcommand {
        SystemSubcommands::Build => {
            println!("TODO: system build — compile actions into SQLite cache");
        }
        SystemSubcommands::Validate { path } => {
            let target_path = path.unwrap_or_else(|| crate::utils::path::actions_dir());
            println!(
                "TODO: system validate — re‑validate config at {:?}",
                target_path
            );
        }
    }
}
