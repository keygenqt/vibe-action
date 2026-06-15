//! Vibe Action — AI-native command router.
//! Execute shell commands and LLM prompts via simple YAML actions.

use std::path::PathBuf;

use clap::Parser;

use crate::configs::app::AppConfig;

mod cli;
mod configs;
mod default;
mod engine;
mod models;
mod utils;
mod validate;

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
}

#[tokio::main]
async fn main() {
    // Parse command-line arguments (global flags only at this stage)
    let (config_path, debug) = utils::clap::parse_global_flags();

    // Initialize configuration: load or create default config, validate, set up logging
    if let Err(e) = AppConfig::init(config_path, debug) {
        exit_error!("{}", e);
    }

    // Obtain the global configuration singleton (initialized above)
    let config = match AppConfig::instance() {
        Ok(v) => v,
        Err(e) => exit_error!("{}", e),
    };

    // Build the full CLI tree, enriching the `action` subcommand with
    // dynamically loaded YAML actions from the configuration
    let mut app_builder = build_app!(&config);
    let matches = app_builder.clone().get_matches();

    // Dispatch to the appropriate handler (built-in commands or dynamic actions)
    match matches.subcommand() {
        Some((cmd_name, action_matches)) => {
            cli::action::execute(cmd_name, action_matches, config).await;
        }
        _ => {
            let _ = app_builder.print_help();
        }
    }
}
