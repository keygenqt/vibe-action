//! Default flow trait — validates built-in YAML flows from embedded YAML files.

use crate::{models::flow::FlowModel, validate::ValidateTrait};
use anyhow::Result;

/// Common header template for all built-in YAML flows.
pub const FLOW_HEADER: &str = r#"# Vibe Action — {{name}}
# {{about}}
#
# Fields:
#   name      - Action name (CLI subcommand)
#   about     - Short description
#   check     - Optional regex validation for the final flow result
#   clipboard - Copy final result to clipboard (default: false)
#   notify    - Show desktop notification on completion (default: false)
#   args      - CLI arguments (optional)
#   actions   - Pipeline steps (execution order resolved automatically by tags)
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
#   run       - cmd (shell), llm / llm_small / llm_medium / llm_large (AI), value (static)
#   expect    - Expected output type: void, bool, number, string, list<string>
#   check     - Optional regex validation for the step result
#   confirm   - Ask for user confirmation before executing (default: false)
#   action    - Shell command, LLM prompt, static string, or when/then list
#
# Modifiers:
#   {tag|upper}        - Any   : transform to UPPERCASE
#   {tag|lower}        - Any   : transform to lowercase
#   {tag|reverse}      - Any   : reverse order
#   {tag|take:N}       - Any   : first N chars (string) or elements (list)
#   {tag|trim}         - Any   : strip whitespace, remove empty
#   {tag|trim:chars}   - Any   : strip custom chars, remove matching
#   {tag|join}         - List  : join elements with \n
#   {tag|join:uniq}    - List  : join with deduplication
#   {tag|sort}         - List  : sort ascending
#   {tag|sort:desc}    - List  : sort descending
#   {tag|split}        - String: split into list by \n
#   {tag|ast:lang}     - String: parse source code to JSON (rs, py, ts, js, kt, ...)
#   {tag|contains:X}   - Any   : check if contains X (predicate, supports :not)
#   {tag|empty}        - Any   : check if empty (predicate, supports :not)
#   {tag|equals:X}     - Any   : check if equals X (predicate, supports :not)
#   {tag|is_file}      - Any   : check if path is a file (predicate, supports :not)
#   {tag|is_dir}       - Any   : check if path is a directory (predicate, supports :not)
#   {tag|size}         - Any   : length of string or list
#   {tag|resolve}      - Any   : resolve path to absolute (~, ., .. expanded)
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
            .replace("{{name}}", &model.name)
            .replace("{{about}}", &model.about);
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
            yaml: include_str!("actions/comment.yaml"),
        }),
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
            yaml: include_str!("actions/review.yaml"),
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
            yaml: include_str!("actions/translate-deep.yaml"),
        }),
        Box::new(BuiltinFlow {
            yaml: include_str!("actions/translate-fast.yaml"),
        }),
    ]
}
