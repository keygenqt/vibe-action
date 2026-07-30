//! Vibe Action — YAML shell/llm pipeline runner.
//! Execute shell commands and LLM prompts via simple YAML actions.

use clap::{Parser, Subcommand};

use crate::{cli::bench::BenchArgs, configs::app::AppConfig, output::output::OutputKind};

mod bench;
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
    /// Run benchmarks
    Bench(BenchArgs),
    /// Remove all cache
    Clean,
    /// Stop all running processes
    Stop,
}

#[tokio::main]
async fn main() {
    if let Err(e) = AppConfig::init() {
        print_text!(OutputKind::Error, "{}", e);
        std::process::exit(1);
    }

    let config = match AppConfig::instance() {
        Ok(v) => v,
        Err(e) => {
            print_text!(OutputKind::Error, "{}", e);
            std::process::exit(1);
        }
    };

    let app_builder = build_app!(&config);
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 1 || args.len() == 2 && (args[1] == "-h" || args[1] == "--help") {
        utils::clap::print_custom_help(&app_builder, &config);
        return;
    }

    let app = App::try_parse();
    match app {
        Ok(app) => match app.command {
            Some(Commands::Status) => cli::status::execute().await,
            Some(Commands::Clean) => cli::clean::execute().await,
            Some(Commands::Stop) => match utils::run_guard::RunGuard::start() {
                Ok(_) => {
                    print_text!(OutputKind::Info, "Stopping running processes...");
                    std::process::exit(0)
                }
                Err(e) => {
                    print_text!(OutputKind::Error, "{}", e);
                    std::process::exit(1);
                }
            },
            Some(Commands::Bench(args)) => cli::bench::execute(args).await,
            _ => utils::clap::print_custom_help(&app_builder, &config),
        },
        Err(_) => {
            // Singleton guard – stops the previous command when a new one starts,
            // instead of failing with an error. This keeps the shared state
            // (cache, local LLMs) predictable for beginners.
            //
            // On drop, the guard removes our pid file to allow clean shutdown.
            //
            // VIBE_SKIP_LOCK disables the guard: the process runs without affecting
            // other instances (useful for parallel API clusters).
            let _guard = if std::env::var_os("VIBE_SKIP_LOCK").is_none() {
                match utils::run_guard::RunGuard::start() {
                    Ok(guard) => Some(guard),
                    Err(e) => {
                        print_text!(OutputKind::Error, "{}", e);
                        std::process::exit(1);
                    }
                }
            } else {
                None
            };
            let matches = app_builder.clone().get_matches();
            match matches.subcommand() {
                Some((cmd_name, action_matches)) => {
                    cli::action::execute(cmd_name, action_matches, config).await;
                }
                _ => utils::clap::print_custom_help(&app_builder, &config),
            }
        }
    }
}
