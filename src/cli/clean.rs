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

    // Clean temp files created by load modifier
    match std::fs::read_dir(std::env::temp_dir()) {
        Ok(entries) => {
            let mut count = 0;
            for entry in entries.flatten() {
                let name = entry.file_name();
                let name_str = name.to_string_lossy();
                if name_str.starts_with("vibe-") {
                    if std::fs::remove_file(entry.path()).is_ok() {
                        count += 1;
                    }
                }
            }
            if count > 0 {
                print_info!("Temp files cleaned: {}", count);
            }
        }
        Err(_) => {}
    }

    // Clean context
    // print_info!("Context cache cleaned");
    // @todo
}
