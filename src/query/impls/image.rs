//! Query provider for `{query|image}` — image from clipboard, file path, or IDE screenshot.

use crate::query::query::QueryKey;
use crate::query::query::QueryProvider;
use crate::utils;
use anyhow::Result;
use base64::Engine;
use std::path::Path;

pub struct ImageProvider {
    raw_value: Option<String>,
}

impl ImageProvider {
    pub fn new(raw_value: Option<String>) -> Self {
        Self { raw_value }
    }

    /// Check if a path points to an image file.
    fn is_image_file(path: &Path) -> bool {
        if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
            return matches!(
                ext.to_lowercase().as_str(),
                "png" | "jpg" | "jpeg" | "gif" | "webp" | "bmp"
            );
        }
        false
    }

    /// Read file bytes and encode as base64.
    fn encode_file_to_base64(path: &Path) -> Result<String> {
        let bytes = std::fs::read(path)?;
        Ok(base64::engine::general_purpose::STANDARD.encode(&bytes))
    }
}

impl QueryProvider for ImageProvider {
    fn key(&self) -> QueryKey {
        QueryKey::Image
    }

    fn resolve(&self) -> Result<String> {
        // 1. Check raw_value (might be a file path or base64 from IDE)
        if let Some(raw) = self.raw_value.as_deref().filter(|v| !v.trim().is_empty()) {
            let paths = utils::clipboard::parse_uri_list(raw);
            if let Some(first_path) = paths.first() {
                if let Ok(path) = utils::path::resolve(first_path) {
                    if path.is_file() && Self::is_image_file(&path) {
                        return Ok(Self::encode_file_to_base64(&path)?);
                    }
                }
            }
            // If not a valid image path, assume it's a base64 string
            return Ok(raw.trim().to_string());
        }

        // 2. Check clipboard for copied image files (Finder/Nautilus)
        let file_paths = utils::clipboard::clipboard_file_paths();
        if let Some(first_path) = file_paths.first() {
            if let Ok(path) = utils::path::resolve(first_path) {
                if path.is_file() && Self::is_image_file(&path) {
                    return Ok(Self::encode_file_to_base64(&path)?);
                }
            }
        }

        // 3. Fallback to raw image data in clipboard (e.g., screenshot)
        match utils::clipboard::read_image_png_base64() {
            Some(base64) => Ok(base64),
            None => Ok(String::new()),
        }
    }
}
