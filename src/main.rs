//! Vibe Action — YAML shell/llm pipeline runner.
//! Execute shell commands and LLM prompts via simple YAML actions.

use clap::{Parser, Subcommand};

use crate::{cli::refresh::RefreshArgs, configs::app::AppConfig};

mod cli;
mod configs;
mod default;
mod engine;
mod models;
mod modifier;
mod output;
mod system;
mod utils;
mod validate;

#[derive(Parser)]
#[command(name = utils::app::app_name())]
#[command(about = utils::app::app_about())]
#[command(styles = utils::app::app_styles())]
#[command(version = utils::app::app_version())]
struct App {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Show system status
    Status,

    /// Refresh actions or context
    Refresh(RefreshArgs),
}

#[tokio::main]
async fn main() {
    if let Err(e) = AppConfig::init() {
        exit_error!("{}", e);
    }

    let config = match AppConfig::instance() {
        Ok(v) => v,
        Err(e) => exit_error!("{}", e),
    };

    let app_builder = build_app!(&config);

    let args: Vec<String> = std::env::args().collect();
    if args.len() == 1 || args.len() == 2 && (args[1] == "-h" || args[1] == "--help") {
        utils::clap::print_custom_help(&app_builder);
        return;
    }

    let app = App::try_parse();
    match app {
        Ok(app) => match app.command {
            Some(Commands::Status) => cli::status::execute().await,
            Some(Commands::Refresh(args)) => cli::refresh::execute(args).await,
            _ => utils::clap::print_custom_help(&app_builder),
        },
        Err(_) => {
            let matches = app_builder.clone().get_matches();
            match matches.subcommand() {
                Some((cmd_name, action_matches)) => {
                    cli::action::execute(cmd_name, action_matches, config).await;
                }
                _ => utils::clap::print_custom_help(&app_builder),
            }
        }
    }
}
