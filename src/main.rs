//! Vibe Action — AI-native command router.
//! Execute shell commands and LLM prompts via simple YAML actions.

use std::path::PathBuf;

use clap::CommandFactory;
use clap::Parser;
use clap::Subcommand;

use crate::cli::run::RunAction;
use crate::cli::srv::SrvAction;
use crate::configs::app::AppConfig;

mod cli;
mod configs;
mod engine;
mod models;
mod utils;

#[derive(Parser)]
#[command(name = utils::app::app_name())]
#[command(about = utils::app::app_about())]
#[command(styles = utils::app::app_styles())]
#[command(version = utils::app::app_version())]
struct App {
    /// Path to config file (default: ~/.vibe-action/config.yaml).
    #[arg(long, global = true)]
    config: Option<PathBuf>,

    /// Enable debug output (verbose logging).
    #[arg(long, global = true)]
    debug: bool,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Execute an action by key or free-form prompt.
    Run(RunAction),

    /// Validate configuration and cache management.
    Srv {
        #[command(subcommand)]
        action: SrvAction,
    },
}

#[tokio::main]
async fn main() {
    let app = App::parse();

    // Initialize config (creates defaults if missing).
    if let Err(error) = AppConfig::init(app.config.clone(), app.debug) {
        exit_error!("{}", error);
    }

    match app.command {
        Some(Commands::Run(action)) => cli::run::execute(action).await,
        Some(Commands::Srv { action }) => cli::srv::execute(action).await,
        None => {
            let _ = App::command().print_help();
        }
    }
}
