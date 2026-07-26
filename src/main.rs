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
    /// Run benchmarks
    Bench(BenchArgs),
    /// Remove all cache
    Clean,
}

#[tokio::main]
async fn main() {
    // Singleton guard: only one command runs at a time.
    //
    // Every invocation works with shared state (cache files, LLM cluster
    // under tight local limits), so parallel commands are not allowed.
    // RunGuard::start() enforces this:
    //   1. takes a startup lock so two instances can't start concurrently;
    //   2. removes stale pid files left by dead processes;
    //   3. signals the previous instance to shut down (renames its pid file
    //      to *.pid.stop, which that instance's monitor thread watches for);
    //   4. waits for it to exit, then force-kills it if it didn't;
    //   5. registers us as the active instance and spawns our own stop
    //      monitor, so the *next* invocation can replace us the same way.
    //
    // The guard must live until the end of main: on drop it deletes our pid
    // file. The underscore prefix keeps it alive without "unused" warnings.
    //
    // VIBE_SKIP_LOCK bypasses the guard entirely (dev escape hatch). Such a
    // run is invisible to the protocol: it doesn't replace the active
    // instance and can't be stopped by the next invocation.
    let _guard = if std::env::var_os("VIBE_SKIP_LOCK").is_none() {
        match utils::active_run::RunGuard::start() {
            Ok(guard) => Some(guard),
            Err(e) => {
                print_text!(OutputKind::Error, "{}", e);
                std::process::exit(1);
            }
        }
    } else {
        None
    };

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
        utils::clap::print_custom_help(&app_builder);
        return;
    }

    let app = App::try_parse();
    match app {
        Ok(app) => match app.command {
            Some(Commands::Status) => cli::status::execute().await,
            Some(Commands::Clean) => cli::clean::execute().await,
            Some(Commands::Bench(args)) => cli::bench::execute(args).await,
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
