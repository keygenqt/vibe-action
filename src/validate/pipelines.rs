//! PipelinesModel validation.
//! See [`crate::validate`] module-level docs for validation rules.

use std::collections::HashSet;

use anyhow::Result;

use crate::models::pipelines::PipelinesModel;
use crate::utils::clap::SYSTEM_COMMANDS;
use crate::validate::ValidateTrait;

impl ValidateTrait for PipelinesModel {
    /// Validate all pipelines: each pipeline internally, plus duplicate names across pipelines.
    fn validate(&self) -> Result<()> {
        // Check for duplicate names. Key is (group, name) — the same action
        // name in different groups is allowed. System-command conflicts are
        // checked for top-level actions only (grouped ones are nested).
        let mut names: HashSet<(&Option<String>, &str)> = HashSet::new();
        for pipeline in &self.pipelines {
            if pipeline.group.is_none() && SYSTEM_COMMANDS.contains(&pipeline.name.as_str()) {
                anyhow::bail!(
                    "Action name '{}' conflicts with built-in command",
                    pipeline.name
                );
            }
            if !names.insert((&pipeline.group, pipeline.name.as_str())) {
                anyhow::bail!("Duplicate action name '{}' across pipelines", pipeline.name);
            }
        }
        Ok(())
    }
}
