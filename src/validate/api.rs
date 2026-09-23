//! PipelineApiModel validation.
//! See [`crate::validate`] module-level docs for validation rules.

use anyhow::Result;

use crate::models::api::PipelineApiModel;
use crate::query::query::QueryKey;
use crate::validate::ValidateTrait;

impl ValidateTrait for PipelineApiModel {
    /// Validate API model.
    fn validate(&self) -> Result<()> {
        let valid_inputs: Vec<&str> = QueryKey::all().iter().map(|k| k.as_str()).collect();

        if let Some(input) = &self.input {
            if !valid_inputs.contains(&input.as_str()) {
                anyhow::bail!(
                    "Invalid api input: '{}'. Expected one of: {}",
                    input.as_str(),
                    valid_inputs.join(", ")
                );
            }
        }

        for (name, query_type) in &self.args {
            if !valid_inputs.contains(&query_type.as_str()) {
                anyhow::bail!(
                    "Invalid api args value for '{}': '{}'. Expected one of: {}",
                    name,
                    query_type,
                    valid_inputs.join(", ")
                );
            }
        }

        // ApiTarget is an enum, so Serde already validates it,
        // but we can leave it here for future complex checks.
        match &self.output {
            crate::models::api::ApiTarget::Replace
            | crate::models::api::ApiTarget::Clipboard
            | crate::models::api::ApiTarget::Dialog => {}
        }

        Ok(())
    }
}
