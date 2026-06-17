//! Default flow trait — validates built-in YAML flows.
//! Each built-in flow implements this trait and returns its YAML as a static string.

use anyhow::Result;

use crate::{
    default::{commit, extract, find, mock, naming, regex, spellcheck, synonyms, tone, translate},
    models::flow::FlowModel,
    validate::ValidateTrait,
};

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
        // read data
        let raw = self.raw();
        let model = self.model(raw)?;
        // validate model
        model.validate()?;
        // create header
        let header = FLOW_HEADER
            .replace("{name}", &model.name)
            .replace("{about}", &model.about);
        // return formatted
        Ok(format!("{}\n{}", header, raw.trim()))
    }

    /// Flow name (used for filename and CLI subcommand).
    fn name(&self) -> Result<String> {
        Ok(self.model(self.raw())?.name)
    }

    /// Raw YAML flow body (without header).
    fn model(&self, raw: &str) -> Result<FlowModel> {
        Ok(yaml_serde::from_str(raw).map_err(|e| anyhow::anyhow!("{}", e))?)
    }
}

/// Returns all built-in default flows.
pub fn default_flows() -> Vec<Box<dyn DefaultFlow>> {
    vec![
        Box::new(commit::CommitFlow),
        Box::new(extract::ExtractFlow),
        Box::new(find::FindFlow),
        Box::new(mock::MockFlow),
        Box::new(naming::NamingFlow),
        Box::new(regex::RegexFlow),
        Box::new(spellcheck::SpellcheckFlow),
        Box::new(synonyms::SynonymsFlow),
        Box::new(tone::ToneFlow),
        Box::new(translate::TranslateFlow),
    ]
}
