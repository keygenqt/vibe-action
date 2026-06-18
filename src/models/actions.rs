//! Aggregated actions model.
//! Loads all flows from configured directories and provides lookup.

use anyhow::Result;
use std::fs;
use std::path::PathBuf;
use walkdir::WalkDir;

use crate::default::default::default_flows;
use crate::models::flow::FlowModel;
use crate::print_warning;
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

    /// Load flows from a directory (recursively reads all .yaml files).
    /// If `is_save_default` is true, saves default flows before loading.
    pub fn load(path: &PathBuf, is_save_default: bool) -> Result<Self> {
        // Save default flows if needed (creates directory and files).
        if is_save_default {
            Self::save_defaults(path)?;
        }

        // Load existing flows from the directory.
        let mut actions = Self { flows: vec![] };
        if path.is_dir() {
            for entry in WalkDir::new(path).follow_links(true) {
                let entry = entry?;
                let file_path = entry.path();
                if file_path
                    .extension()
                    .map_or(false, |e| e == "yaml" || e == "yml")
                {
                    if let Ok(flow) = FlowModel::load(&file_path.to_path_buf()) {
                        actions.flows.push(flow);
                    } else if let Err(e) = FlowModel::load(&file_path.to_path_buf()) {
                        print_warning!("Failed to load {}: {}", file_path.display(), e);
                    }
                }
            }
        } else if path.is_file() {
            let flow = FlowModel::load(&path.to_path_buf())?;
            actions.flows.push(flow);
        } else {
            anyhow::bail!("Actions path not found: {}", path.display());
        }

        actions.validate()?;
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
