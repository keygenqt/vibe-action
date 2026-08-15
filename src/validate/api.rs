//! FlowApiModel validation.
//! Validates the input and output settings for IDE plugin integration.

use anyhow::Result;

use crate::models::api::FlowApiModel;
use crate::validate::ValidateTrait;

impl ValidateTrait for FlowApiModel {
    /// Validate API model.
    fn validate(&self) -> Result<()> {
        let valid_inputs = [
            "query",
            "query|file_path",
            "query|project_path",
            "query|line",
            "query|prompt",
            "query|image",
        ];

        if !valid_inputs.contains(&self.input.as_str()) {
            anyhow::bail!(
                "Invalid api input: '{}'. Expected one of: {}",
                self.input.as_str(),
                valid_inputs.join(", ")
            );
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
