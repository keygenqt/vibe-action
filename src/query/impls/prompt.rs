//! Query provider for `{query|prompt}` — interactive prompt in CLI, dialog in IDE.

use crate::query::query::QueryKey;
use crate::query::query::QueryProvider;
use anyhow::Result;

pub struct PromptProvider {
    raw_value: Option<String>,
}

impl PromptProvider {
    pub fn new(raw_value: Option<String>) -> Self {
        Self { raw_value }
    }
}

impl QueryProvider for PromptProvider {
    fn key(&self) -> QueryKey {
        QueryKey::Prompt
    }

    fn resolve(&self) -> Result<String> {
        let value = self.raw_value.as_deref().unwrap_or("");
        Ok(value.to_string())
    }
}
