//! ActionsModel validation.
//! Validates all flows, checks for duplicate names and tags.

use std::collections::HashSet;

use anyhow::Result;

use crate::{models::actions::ActionsModel, validate::ValidateTrait};

impl ValidateTrait for ActionsModel {
    /// Validate all flows: each flow internally, plus duplicate names across flows.
    fn validate(&self) -> Result<()> {
        for flow in &self.flows {
            flow.validate()?;
        }
        // Check for duplicate names.
        let mut names: HashSet<&str> = HashSet::new();
        for flow in &self.flows {
            if !names.insert(flow.name.as_str()) {
                anyhow::bail!("Duplicate action name '{}' across flows", flow.name);
            }
        }
        Ok(())
    }
}
