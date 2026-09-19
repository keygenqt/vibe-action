//! Query provider for `query_raw` — raw input (positional arg), passed through unchanged.

use crate::query::query::QueryKey;
use crate::query::query::QueryProvider;
use anyhow::Result;

pub struct RawProvider {
    raw_value: Option<String>,
}

impl RawProvider {
    pub fn new(raw_value: Option<String>) -> Self {
        Self { raw_value }
    }
}

impl QueryProvider for RawProvider {
    fn key(&self) -> QueryKey {
        QueryKey::Raw
    }

    fn resolve(&self) -> Result<String> {
        Ok(self
            .raw_value
            .clone()
            .filter(|v| !v.is_empty())
            .unwrap_or_default())
    }
}
