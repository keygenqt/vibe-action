//! Query provider for `{query_clipboard}` — reads text from the clipboard.

use super::query::QueryKey;
use super::query::QueryProvider;
use super::query::read_clipboard_text;
use anyhow::Result;

pub struct ClipboardProvider;

impl ClipboardProvider {
    pub fn new() -> Self {
        Self
    }
}

impl QueryProvider for ClipboardProvider {
    fn key(&self) -> QueryKey {
        QueryKey::Clipboard
    }

    fn resolve(&self) -> Result<String> {
        match read_clipboard_text() {
            Ok(text) => Ok(text),
            Err(_) => Ok(String::new()),
        }
    }
}
