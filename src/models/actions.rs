//! Aggregated actions model.
//! Loads all flows from configured directories and provides lookup.

use anyhow::Result;
use std::fs;
use std::path::PathBuf;
use walkdir::WalkDir;

use crate::default;
use crate::models::flow::FlowModel;
use crate::validate::ValidateTrait;

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
                default::commit::default(),
                default::extract::default(),
                default::find::default(),
                default::mock::default(),
                default::naming::default(),
                default::regex::default(),
                default::spellcheck::default(),
                default::synonyms::default(),
                default::tone::default(),
                default::translate::default(),
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
            let file_path = path.join(format!("{}.yaml", flow.name));
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
#   name      - Action name (CLI subcommand)
#   about     - Short description
#   check     - Optional regex validation for the final flow result
#   clipboard - Copy final result to clipboard
#   args      - CLI arguments (optional)
#   actions   - Pipeline execution steps (Directed Acyclic Graph)
#
# Args:
#   name      - Argument name (used as --name and {{name}} tag)
#   short     - Short flag, e.g. -p (optional)
#   expect    - Expected type: string, number, bool
#   help      - Description for help text (optional)
#   default   - Default value (optional, makes argument non-required)
#
# Actions:
#   tag       - Tag name for {{tag}} references with automatic dependency ordering
#   type      - cmd (shell), llm (AI model), value (static string)
#   expect    - Expected output type: void, bool, number, string, list<T>
#   check     - Optional regex pre-validation for the result
#   confirm   - Ask for user confirmation before executing
#   action    - Shell command, LLM prompt, or static string
#
# Modifiers:
#   {{tag|upper}}      - String: Transforms text to UPPERCASE
#   {{tag|lower}}      - String: Transforms text to lowercase
#   {{tag|join}}       - List  : Collapses list into a single string via newline (\n)
#   {{tag|join:uniq}}  - List  : Collapses list via newline and removes all duplicates
#   {{tag|trim}}       - Any   : Strips whitespace from string or filters empty list elements
#   {{tag|trim:chars}} - Any   : Strips custom chars/whitespace. If string equals chars, returns empty
"#,
                flow.name, flow.about
            );

            let yaml = yaml_serde::to_string(flow)?;
            let content = format!("{}\n{}", header, yaml);
            fs::write(file_path, &content)?;
        }
        Ok(())
    }
}
