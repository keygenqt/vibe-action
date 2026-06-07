//! ActionsModel validation.
//! Validates all flows, checks for duplicate names and tags.

use std::collections::HashSet;

use anyhow::Result;

use crate::{models::actions::ActionsModel, validate::ValidateTrait};

impl ValidateTrait for ActionsModel {
    /// Validate all flows: each flow internally, plus duplicate names across flows.
    fn validate(&self) -> Result<()> {
        // Validate each flow.
        for flow in &self.flows {
            flow.validate()?;
        }
        // Check for duplicate names across flows.
        let mut names: HashSet<&str> = HashSet::new();
        for flow in &self.flows {
            if !names.insert(flow.name.as_str()) {
                anyhow::bail!("Duplicate action name '{}' across flows", flow.name);
            }
        }
        // Check for duplicate trigger tags across flows (global {tag} namespace).
        let mut tags: HashSet<&str> = HashSet::new();
        for flow in &self.flows {
            if !tags.insert(flow.trigger.tag.as_str()) {
                anyhow::bail!("Duplicate trigger tag '{}' across flows", flow.trigger.tag);
            }
        }
        Ok(())
    }
}
