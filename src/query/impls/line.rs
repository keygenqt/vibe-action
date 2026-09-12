//! Query provider for `{query|line}` — cursor line in IDE, first line in CLI.

use crate::query::query::QueryKey;
use crate::query::query::QueryProvider;
use crate::utils;
use anyhow::Result;

pub struct LineProvider {
    raw_value: Option<String>,
}

impl LineProvider {
    pub fn new(raw_value: Option<String>) -> Self {
        Self { raw_value }
    }
}

impl QueryProvider for LineProvider {
    fn key(&self) -> QueryKey {
        QueryKey::Line
    }

    fn resolve(&self) -> Result<String> {
        let text = self
            .raw_value
            .as_deref()
            .filter(|v| !v.is_empty())
            .map(|s| s.to_string())
            .unwrap_or_else(|| utils::clipboard::read_text().unwrap_or_default());

        let line = text.lines().next().unwrap_or("").to_string();
        Ok(line)
    }
}
