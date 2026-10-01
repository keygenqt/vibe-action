//! Vibe Action — YAML shell/llm pipeline runner.
//! Execute shell commands and LLM prompts via simple YAML actions.

use clap::Parser;
use clap::Subcommand;

use crate::configs::app::AppConfig;
use crate::output::output::OutputKind;
use crate::utils::run_guard::RunGuard;

mod cli;
mod configs;
mod default;
mod engine;
mod models;
mod operator;
mod output;
mod query;
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
    /// Remove all cache
    Clean,
    /// Stop all running processes
    Stop,
}

/// Acquires the singleton guard: notifies the previous instance (if any),
/// waits for it to exit, force-kills it on timeout, then registers this
/// process as the active one and arms the stop monitor.
/// On drop, the guard removes our pid file to allow clean shutdown.
fn acquire_run_guard() -> RunGuard {
    match RunGuard::start() {
        Ok(guard) => guard,
        Err(e) => {
            print_text!(OutputKind::Error, "{}", e);
            std::process::exit(1);
        }
    }
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
            Some(Commands::Clean) => {
                // Clean wipes the shared cache — must own the singleton,
                // otherwise it races a running action using that cache.
                let _guard = acquire_run_guard();
                cli::clean::execute().await
            }
            Some(Commands::Stop) => {
                let _guard = acquire_run_guard();
                print_text!(OutputKind::Info, "All running processes stopped");
            }
            _ => utils::clap::print_custom_help(&app_builder, &config),
        },
        Err(_) => {
            // Singleton guard – stops the previous command when a new one starts,
            // instead of failing with an error. This keeps the shared state
            // (cache, local LLMs) predictable for beginners.
            //
            // VIBE_SKIP_LOCK disables the guard: the process runs without affecting
            // other instances (useful for parallel API clusters).
            let _guard = std::env::var_os("VIBE_SKIP_LOCK")
                .is_none()
                .then(acquire_run_guard);

            let matches = app_builder.clone().get_matches();
            match matches.subcommand() {
                Some((cmd_name, action_matches)) => {
                    // Group subcommand — unwrap one more level.
                    let is_group = config.groups.iter().any(|g| g.name == cmd_name);
                    if is_group {
                        match action_matches.subcommand() {
                            Some((inner_name, inner_matches)) => {
                                cli::action::execute(
                                    Some(cmd_name),
                                    inner_name,
                                    inner_matches,
                                    config,
                                )
                                .await;
                            }
                            // No action given — print the group's help.
                            None => utils::clap::print_group_help(&config, cmd_name),
                        }
                    } else {
                        cli::action::execute(None, cmd_name, action_matches, config).await;
                    }
                }
                _ => utils::clap::print_custom_help(&app_builder, &config),
            }
        }
    }
}
