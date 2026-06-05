//! Srv command — validation and configuration management.

use std::path::PathBuf;

use clap::Subcommand;

/// Srv subcommands.
#[derive(Subcommand)]
pub enum SrvAction {
    /// Validate an action file or directory.
    Validate {
        /// Path to .yaml file or directory with actions.
        #[arg(long, short = 'p')]
        path: Option<PathBuf>,
    },
}

/// Execute srv commands.
pub async fn execute(action: SrvAction) {
    match action {
        SrvAction::Validate { path } => {
            // @todo
            println!("validate: {:?}", path);
        }
    }
}
