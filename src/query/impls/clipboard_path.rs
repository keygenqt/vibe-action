//! Query provider for `query_clipboard_path` — copied file paths from the clipboard.

use anyhow::Result;

use crate::query::query::QueryKey;
use crate::query::query::QueryProvider;
use crate::utils::clipboard::clipboard_read_path;

pub struct ClipboardPathProvider;

impl ClipboardPathProvider {
    pub fn new() -> Self {
        Self
    }
}

impl QueryProvider for ClipboardPathProvider {
    fn key(&self) -> QueryKey {
        QueryKey::ClipboardPath
    }

    fn resolve(&self) -> Result<String> {
        let paths = clipboard_read_path();
        Ok(paths
            .iter()
            .map(|p| p.to_string_lossy().into_owned())
            .collect::<Vec<_>>()
            .join("\n"))
    }
}
