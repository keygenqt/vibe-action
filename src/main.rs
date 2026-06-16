//! Vibe Action — YAML shell/llm pipeline runner.
//! Execute shell commands and LLM prompts via simple YAML actions.

use clap::Parser;

use crate::configs::app::AppConfig;

mod cli;
mod configs;
mod default;
mod engine;
mod models;
mod modifier;
mod utils;
mod validate;

#[derive(Parser)]
#[command(name = utils::app::app_name())]
#[command(about = utils::app::app_about())]
#[command(styles = utils::app::app_styles())]
#[command(version = utils::app::app_version())]
struct App;

#[tokio::main]
async fn main() {
    let debug = std::env::var("VIBE_DEBUG").unwrap_or("0".into());
    let level = std::env::var("VIBE_LOG_LEVEL").unwrap_or("0".into());

    // Initialize configuration.
    if let Err(e) = AppConfig::init(debug, level) {
        exit_error!("{}", e);
    }

    // Obtain the global configuration singleton.
    let config = match AppConfig::instance() {
        Ok(v) => v,
        Err(e) => exit_error!("{}", e),
    };

    // Build the full CLI tree with dynamically loaded YAML actions.
    let mut app_builder = build_app!(&config);
    let matches = app_builder.clone().get_matches();

    // Dispatch to the appropriate handler.
    match matches.subcommand() {
        Some((cmd_name, action_matches)) => {
            cli::action::execute(cmd_name, action_matches, config).await;
        }
        _ => {
            let _ = app_builder.print_help();
        }
    }
}
