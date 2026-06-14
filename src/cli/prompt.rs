//! Prompt command handler.
//! Sends raw text directly to the LLM cluster, bypassing YAML actions.

use clap::ArgMatches;

use crate::{engine::cluster::Cluster, print_error, print_info, print_rich_block, utils};

/// Execute the `prompt` command with the given parsed arguments.
pub async fn execute(matches: &ArgMatches) {
    let start_time = std::time::Instant::now();
    let text = utils::clap::extract_text(matches, "text");
    print_info!("Waiting for cluster response...");

    match Cluster::exec(&[text]).await {
        Ok(results) => {
            print_info!("completed in {:.2?}", start_time.elapsed());
            for r in results {
                print_rich_block!("{}", r.result);
            }
        }
        Err(e) => print_error!("{}", e),
    }
}
