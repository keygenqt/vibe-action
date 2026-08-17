//! Clean command handler.
//! Manages cleaning actions cache and resetting context.

use crate::output::output::OutputKind;
use crate::print_template;
use crate::print_text;
use crate::utils;
use crate::utils::constants;

/// Execute the `clean` command.
pub async fn execute() {
    let cache_dir = utils::path::cache_dir();
    let actions_dir = utils::path::actions_dir();

    // Remove stale cache from previous versions
    let cache_root = utils::path::config_dir().join("cache");
    if cache_root.exists() {
        if let Ok(entries) = std::fs::read_dir(&cache_root) {
            for entry in entries.flatten() {
                let name = entry.file_name();
                let name_str = name.to_string_lossy();
                if name_str != constants::FLOW_VERSION {
                    let _ = std::fs::remove_dir_all(entry.path());
                }
            }
        }
    }

    // Clean actions cache
    if let Err(e) = vibe_fs::clean(&actions_dir, Some(&cache_dir)) {
        print_template!(
            OutputKind::Error,
            "Failed to clean actions cache: {error}",
            "error" => e.to_string()
        );
    } else {
        print_text!(OutputKind::Info, "Actions cache cleaned");
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
                print_template!(
                    OutputKind::Info,
                    "Temp files cleaned: {count}",
                    "count" => count
                );
            }
        }
        Err(_) => {}
    }

    // Clear clipboard (stale input fallback for {query} clipboard path)
    match utils::clipboard::clear() {
        Ok(()) => print_text!(OutputKind::Info, "Clipboard cleared"),
        Err(e) => print_template!(
            OutputKind::Error,
            "Failed to clear clipboard: {error}",
            "error" => e.to_string()
        ),
    }
}
