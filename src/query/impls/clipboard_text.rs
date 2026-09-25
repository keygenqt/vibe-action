//! Query provider for `query_clipboard_text` — raw text from the clipboard.

use anyhow::Result;

use crate::query::query::QueryKey;
use crate::query::query::QueryProvider;
use crate::utils::clipboard::clipboard_read_text;

pub struct ClipboardTextProvider;

impl ClipboardTextProvider {
    pub fn new() -> Self {
        Self
    }
}

impl QueryProvider for ClipboardTextProvider {
    fn key(&self) -> QueryKey {
        QueryKey::ClipboardText
    }

    fn resolve(&self) -> Result<String> {
        Ok(clipboard_read_text().unwrap_or_default())
    }
}
