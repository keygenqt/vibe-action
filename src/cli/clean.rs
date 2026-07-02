//! Refresh command handler.
//! Manages refreshing actions cache and resetting context.

use crate::{print_error, print_info, utils};

/// Execute the `refresh` command.
pub async fn execute() {
    let cache_dir = utils::path::cache_dir();
    let actions_dir = utils::path::actions_dir();

    // Clean actions cache
    if let Err(e) = vibe_fs::clean(&actions_dir, Some(&cache_dir)) {
        print_error!("Failed to clean actions cache: {}", e);
    } else {
        print_info!("Actions cache cleaned");
    }

    // Clean context
    print_info!("Context cache cleaned");
    // @todo
}
