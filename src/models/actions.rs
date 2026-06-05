//! Aggregated actions model.
//! Loads all flows from configured directories and provides lookup.

use anyhow::Result;
use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;
use walkdir::WalkDir;

use crate::default;
use crate::models::flow::FlowModel;

/// Aggregated actions from all sources.
#[derive(Debug, Clone)]
pub struct ActionsModel {
    /// All loaded flows.
    pub flows: Vec<FlowModel>,
}

impl Default for ActionsModel {
    /// Create with all default actions embedded in the binary.
    fn default() -> Self {
        Self {
            flows: vec![
                default::act_default::commit::default(),
                default::act_default::extract::default(),
                default::act_default::find::default(),
                default::gen_default::naming::default(),
                default::gen_default::synonyms::default(),
                default::gen_default::tone::default(),
                default::mod_default::spellcheck::default(),
                default::mod_default::todo::default(),
                default::mod_default::translate::default(),
            ],
        }
    }
}
impl ActionsModel {
    /// Find a flow by name (exact match).
    pub fn find(&self, name: &str) -> Option<&FlowModel> {
        self.flows.iter().find(|f| f.name == name)
    }

    /// Load flows from a directory (recursively reads all .yaml files).
    pub fn load(path: &PathBuf) -> Result<Self> {
        let mut actions = Self { flows: vec![] };
        if path.is_dir() {
            for entry in WalkDir::new(path).follow_links(true) {
                let entry = entry?;
                let file_path = entry.path();
                if file_path
                    .extension()
                    .map_or(false, |e| e == "yaml" || e == "yml")
                {
                    let flow = FlowModel::load(&file_path.to_path_buf())?;
                    actions.flows.push(flow);
                }
            }
        } else if path.is_file() {
            let flow = FlowModel::load(&path.to_path_buf())?;
            actions.flows.push(flow);
        } else {
            return Ok(ActionsModel::default());
        }
        actions.validate()?;
        Ok(actions)
    }

    /// Save all flows to their respective YAML files (only if file doesn't exist).
    pub fn save(&self, path: &PathBuf) -> Result<()> {
        for flow in &self.flows {
            let file_path = path.join(flow.path.clone());
            if file_path.exists() {
                continue;
            }
            if let Some(parent) = file_path.parent() {
                if !parent.exists() {
                    fs::create_dir_all(parent)?;
                }
            }

            let header = format!(
                r#"# Vibe Action — {}
# {}
#
# Fields:
#   name    - Action name (CLI subcommand)
#   about   - Short description
#   args    - CLI arguments (optional)
#   trigger - Main action (executed last)
#   actions - Preparation steps (optional)
"#,
                flow.name, flow.about
            );

            let yaml = yaml_serde::to_string(flow)?;
            let content = format!("{}\n{}", header, yaml);
            fs::write(file_path, &content)?;
        }
        Ok(())
    }

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
