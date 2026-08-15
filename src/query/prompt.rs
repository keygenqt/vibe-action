//! Query provider for `{query|prompt}` — interactive prompt in CLI, dialog in IDE.

use super::query::QueryKey;
use super::query::QueryProvider;
use crate::models::context::ContextModel;
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

    fn resolve(&self) -> Result<ContextModel> {
        // TODO: Show interactive prompt
        let value = self.raw_value.as_deref().unwrap_or("");
        Ok(ContextModel::String(format!("[prompt] {}", value)))
    }
}
