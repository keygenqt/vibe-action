//! Text modifier — extracts text from HTML, PDF, or converts images to base64.

use anyhow::Result;
use std::fs;
use std::path::Path;

use super::modifier::Modifier;
use super::modifier::ModifierKey;
use crate::models::context::ContextModel;
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

    fn apply(&self, value: &ContextModel, _arg: &str) -> Result<ContextModel> {
        match value {
            ContextModel::String(s) => {
                // Base64 image — return as-is
                if crate::utils::image::is_image(s) {
                    return Ok(ContextModel::String(s.clone()));
                }

                // Direct HTML content
                if let Some(text) = Self::html_to_text(s)? {
                    return Ok(ContextModel::String(text));
                }

                // Try as file path
                if let Ok(path) = utils::path::resolve(Path::new(s)) {
                    if path.exists() && !path.is_dir() {
                        let bytes = fs::read(&path)?;
                        if let Some(text) = Self::img_to_text(&bytes)? {
                            return Ok(ContextModel::String(text));
                        }
                        if let Some(text) = Self::pdf_to_text(&bytes)? {
                            return Ok(ContextModel::String(text));
                        }
                        let content = String::from_utf8_lossy(&bytes).to_string();
                        if let Some(text) = Self::html_to_text(&content)? {
                            return Ok(ContextModel::String(text));
                        }
                        return Ok(ContextModel::String(content));
                    }
                }

                // If we got here, it's an invalid path or unsupported content
                anyhow::bail!("File not found or unsupported content: '{}'", s)
            }

            ContextModel::List(items) => {
                let results: Vec<String> = items
                    .iter()
                    .map(
                        |i| match self.apply(&ContextModel::String(i.clone()), "")? {
                            ContextModel::String(s) => Ok(s),
                            _ => unreachable!(),
                        },
                    )
                    .collect::<Result<Vec<_>>>()?;
                Ok(ContextModel::List(results))
            }
        }
    }
}
