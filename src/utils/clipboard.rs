//! Clipboard helpers: native file list + text/uri-list parsing.

use std::path::PathBuf;
use url::Url;

/// File paths copied in Finder / Nautilus / Dolphin.
/// 1) native file list via clipboard-rs; 2) arboard text fallback.
pub fn clipboard_file_paths() -> Vec<PathBuf> {
    use clipboard_rs::Clipboard;
    use clipboard_rs::ClipboardContext;

    if let Ok(ctx) = ClipboardContext::new() {
        if let Ok(files) = ctx.get_files() {
            let paths: Vec<PathBuf> = files.into_iter().map(PathBuf::from).collect();
            if !paths.is_empty() {
                return paths;
            }
        }
    }

    // Fallback: text may be a file:// URI or a text/uri-list payload
    if let Ok(mut cb) = arboard::Clipboard::new() {
        if let Ok(text) = cb.get_text() {
            return parse_uri_list(&text);
        }
    }
    Vec::new()
}

/// Parses text/uri-list (or a single line) into paths.
/// Skips comments ('#'), decodes percent-encoding via `url`,
/// passes plain (non-URI) lines through as-is.
pub fn parse_uri_list(text: &str) -> Vec<PathBuf> {
    text.lines()
        .map(str::trim)
        .filter(|l| !l.is_empty() && !l.starts_with('#'))
        .filter_map(|line| {
            if line.starts_with("file://") {
                Url::parse(line).ok()?.to_file_path().ok()
            } else if line == "cut" || line == "copy" {
                None // x-special/gnome-copied-files first line
            } else {
                Some(PathBuf::from(line))
            }
        })
        .collect()
}
