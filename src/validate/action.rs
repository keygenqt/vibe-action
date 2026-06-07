//! ActionModel validation.
//! Checks type, expect, match regex, and non-empty action.

use anyhow::Result;
use regex::Regex;

use crate::{
    models::action::{ActionMode, ActionModel, ExpectMode},
    validate::ValidateTrait,
};

impl ValidateTrait for ActionModel {
    /// Validate action fields.
    fn validate(&self) -> Result<()> {
        // Action cannot be empty.
        if self.action.is_empty() || self.action.chars().all(|c| c.is_whitespace()) {
            anyhow::bail!("Action '{}' has empty command/prompt.", self.tag);
        }
        // LLM must have expect set (need to know what to parse).
        if self.r#type == ActionMode::Llm && self.expect == ExpectMode::Void {
            anyhow::bail!(
                "LLM action '{}' cannot have expect: void. Specify what to expect.",
                self.tag
            );
        }
        // Validate match regex if present.
        if let Some(pattern) = &self.r#match {
            if matches!(self.expect, ExpectMode::Json) {
                anyhow::bail!(
                    "Action '{}' has a 'match' regex pattern, but 'expect' is set to Json. \
                    Regex validation is not supported for structured JSON objects.",
                    self.tag
                );
            }
            regex::Regex::new(pattern).map_err(|e| {
                anyhow::anyhow!("Action '{}' has invalid match regex: {}", self.tag, e)
            })?;
        }

        // Compile match regex if present.
        if let Some(pattern) = &self.r#match {
            Regex::new(pattern).map_err(|e| {
                anyhow::anyhow!("Action '{}' has invalid match regex: {}", self.tag, e)
            })?;
        }

        Ok(())
    }
}
