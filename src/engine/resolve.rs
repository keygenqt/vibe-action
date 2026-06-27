//! Value resolver — converts raw strings into ContextModel values.

use anyhow::Result;

use crate::models::context::ContextModel;

/// Resolves raw strings into typed ContextModel values.
pub struct Resolve;

impl Resolve {
    /// Main entry point — route to specific type handler based on explicit engine blueprints.
    pub fn resolve(raw: &str, expect: &Option<ContextModel>) -> Result<Option<ContextModel>> {
        match expect {
            // None explicitly defines a 'void' step — no context registration tracking is required
            None => Ok(None),

            // Explicitly retain the String contract container even if the underlying literal buffer is empty
            Some(ContextModel::String(_)) => Ok(Some(ContextModel::String(raw.to_string()))),

            // Explicitly retain the List contract container even if no matching rows survived the filtering pass
            Some(ContextModel::List(_)) => {
                let items: Vec<String> = raw
                    .lines()
                    .filter(|l| !l.is_empty())
                    .map(|l| l.to_string())
                    .collect();

                Ok(Some(ContextModel::List(items)))
            }
        }
    }
}
