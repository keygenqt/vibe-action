//! Query provider for `{query}` — raw input (positional arg or clipboard text).

use super::query::QueryKey;
use super::query::QueryProvider;
use super::query::read_clipboard_text;
use anyhow::Result;

pub struct QueryRawProvider {
    raw_value: Option<String>,
}

impl QueryRawProvider {
    pub fn new(raw_value: Option<String>) -> Self {
        Self { raw_value }
    }
}

impl QueryProvider for QueryRawProvider {
    fn key(&self) -> QueryKey {
        QueryKey::Raw
    }

    fn resolve(&self) -> Result<String> {
        if let Some(value) = &self.raw_value {
            if !value.is_empty() {
                return Ok(value.clone());
            }
        }

        match read_clipboard_text() {
            Ok(text) => Ok(text),
            Err(_) => Ok(String::new()),
        }
    }
}
