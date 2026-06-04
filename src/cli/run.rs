//! Run command — execute an action by key or free-form prompt.

use crate::{
    configs::app::AppConfig, engine::engine::Engine, exit_error, print_newline, print_progress,
    print_success,
};

/// Arguments for `run` command.
#[derive(clap::Args)]
pub struct RunAction {
    /// Prompt text to execute.
    pub text: Vec<String>,
}

/// Execute run command.
pub async fn execute(action: RunAction) {
    let prompt = action.text.join(" ");
    let config = match AppConfig::instance() {
        Ok(c) => c,
        Err(e) => exit_error!("{}", e),
    };
    let flow = match config.search_action(&prompt) {
        Ok(f) => f,
        Err(e) => exit_error!("{}", e),
    };
    match Engine::run(
        flow,
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
        Err(e) => exit_error!("{}", e),
    };
}
