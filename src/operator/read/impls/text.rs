//! Text operator — extracts text from HTML, PDF, or converts images to base64.

use crate::operator::operator::Operator;
use crate::operator::operator::OperatorKey;
use crate::operator::operator::map_items;
use crate::operator::read::read::ReadKey;
use crate::utils;
use anyhow::Result;
use std::fs;
use std::path::Path;

pub struct TextOperator;

impl TextOperator {
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

    /// Convert image bytes to base64 string (if the bytes are a valid image).
    fn img_to_text(bytes: &[u8]) -> Result<Option<String>> {
        if utils::image::is_image_bytes(bytes) {
            return Ok(Some(utils::image::encode_to_base64(bytes)));
        }
        Ok(None)
    }

    /// Process a single value: base64 → HTML → file (image/PDF/HTML/text).
    fn process_one(s: &str) -> Result<String> {
        // Base64 image — return as-is
        if crate::utils::image::is_image_base64(s) {
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
    }
}

impl Operator for TextOperator {
    fn key(&self) -> OperatorKey {
        ReadKey::Text.key()
    }

    fn apply(&self, value: &str, _arg: &str) -> Result<String> {
        map_items(value, Self::process_one)
    }
}
