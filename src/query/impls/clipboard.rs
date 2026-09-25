//! Query provider for `query_clipboard` — combined clipboard content by priority (text → paths → image).

use anyhow::Result;

use crate::configs::app::AppConfig;
use crate::query::query::QueryKey;
use crate::query::query::QueryProvider;
use crate::utils::clipboard::clipboard_read_all;

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
        let max_tokens = AppConfig::instance().ok().map(|c| {
            c.cluster
                .iter()
                .map(|cluster| cluster.num_ctx)
                .max()
                .unwrap_or(4096)
        });
        match clipboard_read_all(max_tokens) {
            Ok(text) => Ok(text),
            Err(_) => Ok(String::new()),
        }
    }
}
