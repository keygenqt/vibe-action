//! Default flow trait — validates built-in YAML flows from embedded YAML files.

use crate::{models::flow::FlowModel, validate::ValidateTrait};
use anyhow::Result;

/// Common header template for all built-in YAML flows.
pub const FLOW_HEADER: &str = r#"# Vibe Action — {name}
# {about}
#
# Fields:
#   name      - Action name (CLI subcommand)
#   about     - Short description
#   check     - Optional regex validation for the final flow result
#   clipboard - Copy final result to clipboard
#   args      - CLI arguments (optional)
#   actions   - Pipeline execution steps (execution order resolved automatically by tags)
#
# Args:
#   name      - Argument name (used as --name and {name} tag)
#   short     - Short flag, e.g. -p (optional)
#   expect    - Expected type: string, number, bool
#   help      - Description for help text (optional)
#   default   - Default value (optional, makes argument non-required)
#
# Actions:
#   tag       - Tag name for {tag} references with automatic dependency ordering
#   type      - cmd (shell), llm (AI model), value (static string)
#   expect    - Expected output type: void, bool, number, string, list<T>
#   check     - Optional regex pre-validation for the result
#   confirm   - Ask for user confirmation before executing
#   action    - Shell command, LLM prompt, or static string
#
# Modifiers:
#   {tag|upper}      - String: Transforms text to UPPERCASE
#   {tag|lower}      - String: Transforms text to lowercase
#   {tag|join}       - List  : Collapses list into a single string via newline (\n)
#   {tag|join:uniq}  - List  : Collapses list via newline and removes all duplicates
#   {tag|trim}       - Any   : Strips whitespace from string or filters empty list elements
#   {tag|trim:chars} - Any   : Strips custom chars/whitespace. If string equals chars, returns empty
"#;

pub trait DefaultFlow {
    /// Raw YAML flow body (without header).
    fn raw(&self) -> &'static str;

    /// Returns the YAML content for this flow (header + validated body).
    fn flow(&self) -> Result<String> {
        let raw = self.raw();
        let model = self.model(raw)?;
        model.validate()?;
        let header = FLOW_HEADER
            .replace("{name}", &model.name)
            .replace("{about}", &model.about);
        Ok(format!("{}\n{}", header, raw.trim()))
    }

    /// Flow name from parsed YAML.
    fn name(&self) -> Result<String> {
        Ok(self.model(self.raw())?.name)
    }

    /// Parse raw YAML into FlowModel.
    fn model(&self, raw: &str) -> Result<FlowModel> {
        Ok(yaml_serde::from_str(raw).map_err(|e| anyhow::anyhow!("{}", e))?)
    }
}

/// A built-in flow with embedded YAML.
pub struct BuiltinFlow {
    pub yaml: &'static str,
}

impl DefaultFlow for BuiltinFlow {
    fn raw(&self) -> &'static str {
        self.yaml
    }
}

/// Returns all built-in default flows.
pub fn default_flows() -> Vec<Box<dyn DefaultFlow>> {
    vec![
        Box::new(BuiltinFlow {
            yaml: include_str!("actions/commit.yaml"),
        }),
        Box::new(BuiltinFlow {
            yaml: include_str!("actions/extract.yaml"),
        }),
        Box::new(BuiltinFlow {
            yaml: include_str!("actions/find.yaml"),
        }),
        Box::new(BuiltinFlow {
            yaml: include_str!("actions/mock.yaml"),
        }),
        Box::new(BuiltinFlow {
            yaml: include_str!("actions/naming.yaml"),
        }),
        Box::new(BuiltinFlow {
            yaml: include_str!("actions/regex.yaml"),
        }),
        Box::new(BuiltinFlow {
            yaml: include_str!("actions/spellcheck.yaml"),
        }),
        Box::new(BuiltinFlow {
            yaml: include_str!("actions/synonyms.yaml"),
        }),
        Box::new(BuiltinFlow {
            yaml: include_str!("actions/tone.yaml"),
        }),
        Box::new(BuiltinFlow {
            yaml: include_str!("actions/translate.yaml"),
        }),
    ]
}
