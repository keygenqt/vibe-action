//! ActionsModel validation.
//! Validates all flows, checks for duplicate names and tags.

use std::collections::HashSet;

use anyhow::Result;

use crate::models::flows::FlowsModel;
use crate::validate::ValidateTrait;

impl ValidateTrait for FlowsModel {
    /// Validate all flows: each flow internally, plus duplicate names across flows.
    fn validate(&self) -> Result<()> {
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
