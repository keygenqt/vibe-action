//! Srv command — validation and configuration management.

use std::path::PathBuf;

use clap::Subcommand;

use crate::{
    configs::source::ActionSourceConfig, models::actions::ActionsModel, print_error, print_success,
};

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
            let source = match path {
                Some(p) => ActionSourceConfig::from(p.to_string_lossy().to_string()),
                None => {
                    print_error!("Path is required. Use --path <file or directory>");
                    return;
                }
            };
            match source.validate() {
                Ok(_) => {
                    let sources = vec![source];
                    match ActionsModel::load(&sources) {
                        Ok(_) => print_success!("Actions are valid."),
                        Err(e) => print_error!("Validation failed: {}", e),
                    }
                }
                Err(e) => print_error!("{}", e),
            }
        }
    }
}
