//! Prompt command handler.
//! Sends raw text directly to the LLM cluster, bypassing YAML actions.

use clap::ArgMatches;

use crate::utils;

/// Execute the `prompt` command with the given parsed arguments.
pub async fn execute(matches: &ArgMatches) {
    let text = utils::clap::extract_text(matches, "text");
    // TODO: send `text` directly to the LLM cluster
    println!("TODO: direct prompt to LLM — {}", text);
}
