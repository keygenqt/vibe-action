//! Aggregated actions model.
//! Loads all flows from configured directories and provides lookup.

use anyhow::Result;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;

use crate::configs::source::ActionSourceConfig;
use crate::models::flow::FlowModel;

/// Aggregated actions from all sources.
#[derive(Debug, Clone, Default)]
pub struct ActionsModel {
    /// All loaded flows.
    pub flows: Vec<FlowModel>,
}

impl ActionsModel {
    /// Find a flow matching the user prompt.
    pub fn find(&self, prompt: &str) -> Option<&FlowModel> {
        let prompt_lower = prompt.to_lowercase();
        self.flows.iter().find(|f| {
            f.keys
                .iter()
                .any(|k| prompt_lower.contains(&k.to_lowercase()))
        })
    }

    /// Load flows from all configured sources.
    pub fn load(sources: &[ActionSourceConfig]) -> Result<Self> {
        let mut flows = Vec::new();
        for source in sources {
            if source.is_dir() {
                flows.extend(Self::load_from_dir(&source.resolve()?)?);
            } else if source.is_file() {
                flows.push(Self::load_from_file(&source.resolve()?)?);
            } else {
                anyhow::bail!("Action source not found or unsupported: {}", source);
            }
        }
        let actions = Self { flows };
        actions.validate()?;
        Ok(actions)
    }

    /// Load all .yaml files from a directory recursively.
    fn load_from_dir(dir: &Path) -> Result<Vec<FlowModel>> {
        let mut flows = Vec::new();
        for entry in walkdir::WalkDir::new(dir).follow_links(true) {
            let entry = entry?;
            let path = entry.path();
            if path
                .extension()
                .map_or(false, |e| e == "yaml" || e == "yml")
            {
                let content = fs::read_to_string(path)?;
                let flow: FlowModel = yaml_serde::from_str(&content)
                    .map_err(|e| anyhow::anyhow!("Failed to parse {}: {}", path.display(), e))?;
                flows.push(flow);
            }
        }
        Ok(flows)
    }

    /// Load a single .yaml file.
    fn load_from_file(path: &Path) -> Result<FlowModel> {
        let content = fs::read_to_string(path)?;
        yaml_serde::from_str(&content)
            .map_err(|e| anyhow::anyhow!("Failed to parse {}: {}", path.display(), e))
    }

    /// Validate all flows: each flow internally, plus duplicate keys across flows.
    fn validate(&self) -> Result<()> {
        // Validate each flow.
        for flow in &self.flows {
            flow.validate()?;
        }
        // Check for duplicate keys across different flows.
        let mut all_keys: HashMap<&str, &str> = HashMap::new(); // key -> flow tag
        for flow in &self.flows {
            for key in &flow.keys {
                if let Some(existing_flow) =
                    all_keys.insert(key.as_str(), flow.trigger.tag.as_str())
                {
                    anyhow::bail!(
                        "Duplicate key '{}' found in flows '{}' and '{}'",
                        key,
                        existing_flow,
                        flow.trigger.tag
                    );
                }
            }
        }
        // Check for duplicate trigger tags across flows.
        let mut all_tags: HashSet<&str> = HashSet::new();
        for flow in &self.flows {
            if !all_tags.insert(flow.trigger.tag.as_str()) {
                anyhow::bail!("Duplicate trigger tag '{}' across flows", flow.trigger.tag);
            }
        }
        Ok(())
    }
}
