//! Prompt command handler.
//! Sends raw text directly to the LLM cluster, bypassing YAML actions.

use clap::ArgMatches;

/// Execute the `prompt` command with the given parsed arguments.
pub async fn execute(sub_matches: &ArgMatches) {
    let text = sub_matches
        .get_many::<String>("text")
        .map(|vals| vals.cloned().collect::<Vec<_>>().join(" "))
        .unwrap_or_default();
    // TODO: send `text` directly to the LLM cluster
    println!("TODO: direct prompt to LLM — {}", text);
}
