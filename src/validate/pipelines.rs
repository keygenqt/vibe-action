//! ActionsModel validation.
//! Validates all pipelines, checks for duplicate names and tags.

use std::collections::HashSet;

use anyhow::Result;

use crate::models::pipelines::PipelinesModel;
use crate::utils::clap::SYSTEM_COMMANDS;
use crate::validate::ValidateTrait;

impl ValidateTrait for PipelinesModel {
    /// Validate all pipelines: each pipeline internally, plus duplicate names across pipelines.
    fn validate(&self) -> Result<()> {
        // Check for duplicate names.
        let mut names: HashSet<&str> = HashSet::new();
        for pipeline in &self.pipelines {
            if SYSTEM_COMMANDS.contains(&pipeline.name.as_str()) {
                anyhow::bail!(
                    "Action name '{}' conflicts with built-in command",
                    pipeline.name
                );
            }
            if !names.insert(pipeline.name.as_str()) {
                anyhow::bail!("Duplicate action name '{}' across pipelines", pipeline.name);
            }
        }
        Ok(())
    }
}
