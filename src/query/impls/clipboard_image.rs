//! Query provider for `query_clipboard_image` — clipboard image as base64 PNG.

use anyhow::Result;

use crate::query::query::QueryKey;
use crate::query::query::QueryProvider;
use crate::utils::clipboard::clipboard_read_image;

pub struct ClipboardImageProvider;

impl ClipboardImageProvider {
    pub fn new() -> Self {
        Self
    }
}

impl QueryProvider for ClipboardImageProvider {
    fn key(&self) -> QueryKey {
        QueryKey::ClipboardImage
    }

    fn resolve(&self) -> Result<String> {
        Ok(clipboard_read_image().unwrap_or_default())
    }
}
