//! Clipboard helpers: text, image, file list, and clearing.
//! All clipboard I/O consolidated here — no other file should touch
//! clipboard crates directly.

use std::path::PathBuf;
use url::Url;

use anyhow::Result;
use base64::Engine as _;
use base64::engine::general_purpose::STANDARD as BASE64;
use clipboard_rs::Clipboard;
use clipboard_rs::ClipboardContext;
use clipboard_rs::common::RustImage;
use clipboard_rs::common::RustImageData;

/// File paths copied in Finder / Nautilus / Dolphin.
/// 1) native file list via clipboard-rs; 2) text fallback (file:// URI or path).
pub fn clipboard_file_paths() -> Vec<PathBuf> {
    let ctx = match ClipboardContext::new() {
        Ok(ctx) => ctx,
        Err(_) => return Vec::new(),
    };

    // Priority 1: native file list (file-manager copy)
    if let Ok(files) = ctx.get_files() {
        let paths: Vec<PathBuf> = files.into_iter().map(PathBuf::from).collect();
        if !paths.is_empty() {
            return paths;
        }
    }

    // Priority 2: clipboard text may be a file:// URI or a plain path
    if let Ok(text) = ctx.get_text() {
        return parse_uri_list(&text);
    }

    Vec::new()
}

/// Plain text from the clipboard, or `None` if unavailable.
pub fn read_text() -> Option<String> {
    let ctx = ClipboardContext::new().ok()?;
    ctx.get_text().ok()
}

/// Image from the clipboard as base64-encoded PNG, or `None` if unavailable.
pub fn read_image_png_base64() -> Option<String> {
    let ctx = ClipboardContext::new().ok()?;
    let img = ctx.get_image().ok()?;
    let png = img.to_png().ok()?;
    Some(BASE64.encode(png.get_bytes()))
}

/// Sets the clipboard to plain text.
pub fn set_text(text: &str) -> Result<()> {
    let ctx =
        ClipboardContext::new().map_err(|e| anyhow::anyhow!("Failed to access clipboard: {e}"))?;
    ctx.set_text(text.to_string())
        .map_err(|e| anyhow::anyhow!("Failed to set clipboard text: {e}"))?;
    Ok(())
}

/// Sets the clipboard to an image.
pub fn set_image(img: image::DynamicImage) -> Result<()> {
    let ctx =
        ClipboardContext::new().map_err(|e| anyhow::anyhow!("Failed to access clipboard: {e}"))?;
    let rust_img = RustImageData::from_dynamic_image(img);
    ctx.set_image(rust_img)
        .map_err(|e| anyhow::anyhow!("Failed to set clipboard image: {e}"))?;
    Ok(())
}

/// If the clipboard holds an image, returns it as base64-encoded PNG.
/// Unlike `read_image_png_base64`, this is for the *output* side: read the
/// current clipboard image so it can be passed along as base64.
pub fn get_image_as_base64() -> Option<String> {
    let ctx = ClipboardContext::new().ok()?;
    let img = ctx.get_image().ok()?;
    let png = img.to_png().ok()?;
    Some(BASE64.encode(png.get_bytes()))
}

/// Clears the system clipboard.
pub fn clear() -> Result<()> {
    let ctx =
        ClipboardContext::new().map_err(|e| anyhow::anyhow!("Failed to access clipboard: {e}"))?;
    ctx.clear()
        .map_err(|e| anyhow::anyhow!("Failed to clear clipboard: {e}"))?;
    Ok(())
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
