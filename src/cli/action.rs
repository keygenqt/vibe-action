//! Dynamic action command handler.
//! Looks up a YAML-defined action by name and runs it with the given arguments.

use clap::ArgMatches;

use crate::{
    configs::app::AppConfig, engine::engine::Engine, exit_error, print_newline, print_progress,
    print_success,
};

/// Execute a dynamic action command.
pub async fn execute(name: &str, matches: &ArgMatches, config: &AppConfig) {
    let flow = match config.find_flow(name) {
        Ok(f) => f.apply_args(&matches),
        Err(e) => exit_error!("{}", e),
    };

    // Execute the flow pipeline with progress reporting.
    match Engine::run(
        &flow,
        Some(|p| {
            print_progress!(
                "{} ({})... {:.0}% ({}/{})",
                p.tag,
                p.step_type,
                p.percent,
                p.current,
                p.total
            );
            if p.current == p.total {
                print_newline!();
            }
        }),
    )
    .await
    {
        Ok(result) => {
            if result.is_empty() {
                print_success!("Done.");
            } else {
                print_success!("{}", result);
            }
        }
        Err(e) => {
            print_newline!();
            exit_error!("{}", e)
        }
    };
}
