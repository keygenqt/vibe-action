//! Aggregated actions model.
//! Loads all flows from configured directories and provides lookup.

use anyhow::Result;
use std::fs;
use std::path::PathBuf;

use crate::default::default::default_flows;
use crate::models::flow::FlowModel;
use crate::utils;
use crate::validate::ValidateTrait;

/// Aggregated actions from all sources.
#[derive(Debug, Clone)]
pub struct FlowsModel {
    /// All loaded flows.
    pub flows: Vec<FlowModel>,
}

impl FlowsModel {
    /// Find a flow by name (exact match).
    pub fn find(&self, name: &str) -> Option<&FlowModel> {
        self.flows.iter().find(|f| f.name == name)
    }

    /// Load flows from actions directory.
    /// Uses cache for validated files, falls back to defaults on first run.
    pub fn load() -> Result<Self> {
        let path = utils::path::actions_dir();
        let cache_dir = utils::path::cache_dir();

        fs::create_dir_all(&path)?;
        fs::create_dir_all(&cache_dir)?;

        let scan = vibe_fs::scan(&path, false, Some(&cache_dir), Some(&["yaml", "yml"]))?;

        // First run — no files, no snapshot: create defaults
        if scan.changed.is_empty() && scan.unchanged.is_empty() {
            Self::save_defaults(&path)?;
            return Self::load();
        }

        // Validate changed files, update snapshot on success
        if !scan.changed.is_empty() {
            for file_path in &scan.changed {
                let flow = FlowModel::load(file_path)?;
                flow.validate().map_err(|e| {
                    vibe_fs::clean(&path, Some(&cache_dir)).ok();
                    anyhow::anyhow!("Validation failed for {}: {}", file_path.display(), e)
                })?;
            }
        }

        // Load all valid files (changed just validated, unchanged already valid)
        let mut actions = Self { flows: vec![] };
        for file_path in scan.changed.iter().chain(scan.unchanged.iter()) {
            let flow = FlowModel::load(file_path)?;
            actions.flows.push(flow);
        }

        Ok(actions)
    }

    /// Save default actions to a directory. Only creates files that don't already exist.
    fn save_defaults(path: &PathBuf) -> Result<()> {
        if !path.exists() {
            fs::create_dir_all(path)?;
        }
        for flow in default_flows() {
            let yaml = flow.flow()?;
            let name = flow.name()?;
            let file_path = path.join(format!("{}.yaml", name));
            if file_path.exists() {
                continue;
            }
            fs::write(&file_path, &yaml)?;
        }
        Ok(())
    }
}
