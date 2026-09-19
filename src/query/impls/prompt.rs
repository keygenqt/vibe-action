//! Query provider for `query_prompt` — marker signaling interactive fallback.
//! Returns raw_value unchanged; its presence in a val chain tells
//! `needs_prompt()` (and `action.rs`) to prompt the user when the query
//! slot is empty (CLI only).

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
        // Marker provider — returns raw_value unchanged (like query_raw).
        // Its real role is presence-based: `needs_prompt()` detects
        // `query_prompt` in any val chain, and `action.rs` runs the
        // interactive fallback (CLI only) when the query slot is empty.
        // The prompt can't live here — it must fire at a fixed early point
        // in the flow, not during val resolution.
        let value = self.raw_value.as_deref().unwrap_or("");
        Ok(value.to_string())
    }
}
