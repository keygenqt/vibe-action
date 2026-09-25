//! Query provider for `query_image` — image from input (URL, file path, or base64), returned as base64.

use crate::query::query::QueryKey;
use crate::query::query::QueryProvider;
use crate::utils;
use anyhow::Result;
use base64::Engine;
use base64::engine::general_purpose::STANDARD as BASE64;
use std::path::Path;

pub struct ImageProvider {
    raw_value: Option<String>,
}

impl ImageProvider {
    pub fn new(raw_value: Option<String>) -> Self {
        Self { raw_value }
    }
}

impl QueryProvider for ImageProvider {
    fn key(&self) -> QueryKey {
        QueryKey::Image
    }

    fn resolve(&self) -> Result<String> {
        // Empty input → no data → empty string.
        let raw = match self
            .raw_value
            .as_deref()
            .map(str::trim)
            .filter(|v| !v.is_empty())
        {
            Some(v) => v,
            None => return Ok(String::new()),
        };

        // Resolve input to bytes: URL → download, local file → read, else → base64 decode.
        let bytes = if raw.starts_with("http://") || raw.starts_with("https://") {
            utils::fetch::download_bytes(raw)?
        } else {
            match utils::path::resolve(Path::new(raw)) {
                Ok(path) if path.is_file() => std::fs::read(&path)?,
                _ => BASE64.decode(raw.as_bytes()).map_err(|e| {
                    anyhow::anyhow!("Input is not a URL, existing file, or base64: {e}")
                })?,
            }
        };

        // Validate: must be a real image.
        if !utils::image::is_image_bytes(&bytes) {
            anyhow::bail!("Input does not decode into a valid image.");
        }

        Ok(utils::image::encode_to_base64(&bytes))
    }
}
