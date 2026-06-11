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
#   name      - Action name (CLI subcommand)
#   output    - Tag name for the final result
#   format    - Output style: compact (default) or rich
#   about     - Short description
#   match     - Optional regex validation for the result
#   clipboard - Copy result to clipboard (default: false)
#   args      - CLI arguments (optional)
#   actions   - Preparation steps
#
# Args:
#   name      - Argument name (used as --name and {{name}} tag)
#   short     - Short flag, e.g. -p (optional)
#   expect    - Expected type: string, number, bool
#   help      - Description for help text (optional)
#   default   - Default value if not provided (optional, makes argument non-required)
#
# Actions:
#   tag       - Tag name for {{tag}} references
#   type      - cmd (shell), llm (AI model), value (static string)
#   expect    - Expected output type: void, bool, number, string, list<T>
#   match     - Optional regex validation for the result
#   confirm   - Ask for confirmation before executing (default: false)
#   action    - Shell command, LLM prompt, or static string
#
# Tags:
#   Use {{tag}} to reference values from other actions.
#   Pipe modifiers: {{tag|upper}}, {{tag|lower}}, {{tag|trim}}, {{tag|trim:-}}
#   List modifier: {{tag|join}} — collapses a list into a single string.
#   Pure {{tag}} with a list value triggers a file-by-file loop.
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
