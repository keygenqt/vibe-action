//! Clipboard I/O: text, paths, image, combined reader, and URI list parsing.
//! See [`crate::utils`] module-level docs for summary.

use clipboard_rs::RustImageData;
use std::path::PathBuf;
use url::Url;

use anyhow::Result;
use base64::Engine as _;
use base64::engine::general_purpose::STANDARD as BASE64;
use clipboard_rs::Clipboard;
use clipboard_rs::ClipboardContext;
use clipboard_rs::common::RustImage;

/// Raw plain text from the clipboard, or `None` if unavailable.
pub fn clipboard_read_text() -> Option<String> {
    let ctx = ClipboardContext::new().ok()?;
    ctx.get_text().ok()
}

/// File paths copied in Finder / Nautilus / Dolphin.
/// 1) native file list via clipboard-rs; 2) text fallback (file:// URI or path).
pub fn clipboard_read_path() -> Vec<PathBuf> {
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

/// Image from the clipboard as base64-encoded PNG, or `None` if unavailable.
pub fn clipboard_read_image() -> Option<String> {
    let ctx = ClipboardContext::new().ok()?;
    let img = ctx.get_image().ok()?;
    let png = img.to_png().ok()?;
    Some(BASE64.encode(png.get_bytes()))
}

/// Combined clipboard content by priority: text → paths → image.
/// First non-empty kind wins; `file://` text is decoded to paths when the
/// targets exist. `max_tokens` bounds the result size (skipped when `None`);
/// images are never size-checked (token count is meaningless on base64).
pub fn clipboard_read_all(max_tokens: Option<usize>) -> Result<String> {
    // Priority 1: text (decode file:// URIs to paths if they point at real files)
    if let Some(raw) = clipboard_read_text() {
        if !raw.is_empty() {
            return enforce_token_limit(resolve_text_or_uris(raw), max_tokens);
        }
    }

    // Priority 2: copied file paths
    let files = clipboard_read_path();
    if !files.is_empty() {
        let text = files
            .iter()
            .map(|p| p.to_string_lossy().into_owned())
            .collect::<Vec<_>>()
            .join("\n");
        return enforce_token_limit(text, max_tokens);
    }

    // Priority 3: image as base64 PNG (no size check)
    if let Some(b64) = clipboard_read_image() {
        return Ok(b64);
    }

    anyhow::bail!("Clipboard is empty or contains non-text data.");
}

/// Decodes `file://` URI lists to joined paths when the targets exist;
/// otherwise returns the raw text unchanged.
fn resolve_text_or_uris(raw: String) -> String {
    let decoded = parse_uri_list(&raw);
    if !decoded.is_empty() && decoded.iter().any(|p| p.exists()) {
        decoded
            .iter()
            .map(|p| p.to_string_lossy().into_owned())
            .collect::<Vec<_>>()
            .join("\n")
    } else {
        raw
    }
}

/// Bails if the text exceeds `max_tokens`; no-op when `max_tokens` is `None`.
fn enforce_token_limit(text: String, max_tokens: Option<usize>) -> Result<String> {
    let max = match max_tokens {
        Some(max) => max,
        None => return Ok(text),
    };
    let bpe =
        tiktoken_rs::cl100k_base().map_err(|e| anyhow::anyhow!("Failed to load tokenizer: {e}"))?;
    let tokens = bpe.encode_with_special_tokens(&text).len();
    if tokens > max {
        anyhow::bail!(
            "Clipboard content is too large ({} tokens). Max context size is {} tokens.",
            tokens,
            max
        );
    }
    Ok(text)
}

/// Sets the clipboard to plain text.
pub fn clipboard_write_text(text: &str) -> Result<()> {
    let ctx =
        ClipboardContext::new().map_err(|e| anyhow::anyhow!("Failed to access clipboard: {e}"))?;
    ctx.set_text(text.to_string())
        .map_err(|e| anyhow::anyhow!("Failed to set clipboard text: {e}"))?;
    Ok(())
}

/// Decodes base64 PNG and sets the clipboard image.
pub fn clipboard_write_image(base64_png: &str) -> Result<()> {
    let ctx =
        ClipboardContext::new().map_err(|e| anyhow::anyhow!("Failed to access clipboard: {e}"))?;
    let bytes = BASE64
        .decode(base64_png.as_bytes())
        .map_err(|e| anyhow::anyhow!("Invalid base64: {e}"))?;
    let img = image::load_from_memory(&bytes)
        .map_err(|e| anyhow::anyhow!("Failed to decode image: {e}"))?;
    let rust_img = RustImageData::from_dynamic_image(img);
    ctx.set_image(rust_img)
        .map_err(|e| anyhow::anyhow!("Failed to set clipboard image: {e}"))?;
    Ok(())
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
