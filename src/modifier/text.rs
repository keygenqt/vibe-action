//! Text modifier — extracts text from HTML, PDF, or converts images to base64.

use anyhow::Result;
use std::fs;
use std::path::Path;

use super::modifier::Modifier;
use super::modifier::ModifierKey;
use crate::modifier::modifier::ITEM_SEP;
use crate::utils;

pub struct TextModifier;

impl TextModifier {
    /// Convert HTML content to plain text.
    fn html_to_text(value: &str) -> Result<Option<String>> {
        let lower = value.to_lowercase();
        if !(lower.contains("<html") || lower.contains("<!doctype") || lower.contains("</div>")) {
            return Ok(None);
        }
        Ok(Some(html2text::from_read(value.as_bytes(), 120)?))
    }

    /// Extract text from PDF bytes.
    fn pdf_to_text(bytes: &[u8]) -> Result<Option<String>> {
        if bytes.starts_with(b"%PDF") {
            let text = pdf_extract::extract_text_from_mem(bytes)
                .map_err(|e| anyhow::anyhow!("PDF text extraction failed: {}", e))?;
            return Ok(Some(text));
        }
        Ok(None)
    }

    /// Convert image bytes to base64 string.
    fn img_to_text(bytes: &[u8]) -> Result<Option<String>> {
        let is_webp = bytes.starts_with(b"RIFF") && bytes.len() > 12 && &bytes[8..12] == b"WEBP";

        if bytes.starts_with(b"\x89PNG")
            || bytes.starts_with(b"\xFF\xD8\xFF")
            || bytes.starts_with(b"GIF8")
            || is_webp
        {
            use base64::Engine;
            let base64 = base64::engine::general_purpose::STANDARD.encode(bytes);
            return Ok(Some(base64));
        }
        Ok(None)
    }
}

impl Modifier for TextModifier {
    fn key(&self) -> ModifierKey {
        ModifierKey::Text
    }

    fn apply(&self, value: &str, _arg: &str) -> Result<String> {
        let results: Vec<String> = value
            .split(ITEM_SEP)
            .map(|s| {
                // Base64 image — return as-is
                if crate::utils::image::is_image(s) {
                    return Ok(s.to_string());
                }

                // Direct HTML content
                if let Some(text) = Self::html_to_text(s)? {
                    return Ok(text);
                }

                // Try as file path
                if let Ok(path) = utils::path::resolve(Path::new(s)) {
                    if path.exists() && !path.is_dir() {
                        let bytes = fs::read(&path)?;
                        if let Some(text) = Self::img_to_text(&bytes)? {
                            return Ok(text);
                        }
                        if let Some(text) = Self::pdf_to_text(&bytes)? {
                            return Ok(text);
                        }
                        let content = String::from_utf8_lossy(&bytes).to_string();
                        if let Some(text) = Self::html_to_text(&content)? {
                            return Ok(text);
                        }
                        return Ok(content);
                    }
                }

                anyhow::bail!("File not found or unsupported content: '{}'", s)
            })
            .collect::<Result<Vec<_>>>()?;
        Ok(results.join(ITEM_SEP))
    }
}
