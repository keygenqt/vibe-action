//! Default pipeline trait — validates built-in YAML pipelines from embedded YAML files.

use crate::models::pipeline::PipelineModel;
use anyhow::Result;

/// Common header template for all built-in YAML pipelines.
pub const PIPELINE_HEADER: &str = r#"# Vibe Action — {{name}}
# {{about}}
#
# ⚠️ This file is auto-updated when the cache version or built-in actions change. Manual edits will be overwritten.
#
# args:
#   name    — arg identifier (used as --name and {name})
#   short   — short flag, e.g. 'f'  [optional]
#   input   — string, bool, number, path  (expected value type)
#   help    — help text shown in CLI usage  [optional]
#   default — default value; makes the arg optional  [optional]
#
# api:
#   input  — IDE query source (query_*)
#   output — replace, clipboard, dialog
#   args   — maps CLI args to IDE query providers
#
# action:
#   tag    — identifier; referenced by other actions via data
#   run    — cmd, value, tiny, small, medium, large, vision
#   val    — name — placeholder used as {name} in `action`
#            data — source tag, arg ref, or free text (query_*, arg_*, system_*)
#            mods — pipe of operators (read, transform, inspect, write)  [optional]
#            each — bool or {split, merge}; split into lines, run once per item (fan-out)  [optional]
#            when — guard; skips this input if it fails  [optional]
#   action — text template; {name} placeholders filled from `val`
#
# Documentation: https://vibe-action.keygenqt.com/
"#;

pub trait DefaultPipeline {
    /// Raw YAML pipeline body (without header).
    fn raw(&self) -> &'static str;

    /// Returns the YAML content for this pipeline (header + validated body).
    fn pipeline(&self) -> Result<String> {
        let raw = self.raw();
        let model = self.model(raw)?;
        let header = PIPELINE_HEADER
            .replace("{{name}}", &model.name)
            .replace("{{about}}", &model.about);
        Ok(format!("{}\n{}", header, raw.trim()))
    }

    /// Pipeline name from parsed YAML.
    fn name(&self) -> Result<String> {
        Ok(self.model(self.raw())?.name)
    }

    /// Parse raw YAML into PipelineModel.
    fn model(&self, raw: &str) -> Result<PipelineModel>;
}

/// A built-in pipeline with embedded YAML.
pub struct BuiltinPipeline {
    pub file: &'static str,
    pub yaml: &'static str,
}

impl DefaultPipeline for BuiltinPipeline {
    fn raw(&self) -> &'static str {
        self.yaml
    }

    fn model(&self, raw: &str) -> Result<PipelineModel> {
        yaml_serde::from_str(raw).map_err(|e| {
            anyhow::anyhow!("Failed to parse built-in pipeline '{}': {}", self.file, e)
        })
    }
}

/// Returns all built-in default pipelines.
pub fn default_pipelines() -> Vec<Box<dyn DefaultPipeline>> {
    vec![
        Box::new(BuiltinPipeline {
            file: "extract2.yaml",
            yaml: include_str!("actions/extract2.yaml"),
        }),
        // Box::new(BuiltinPipeline {
        //     file: "comment.yaml",
        //     yaml: include_str!("actions/comment.yaml"),
        // }),
        // Box::new(BuiltinPipeline {
        //     file: "commit.yaml",
        //     yaml: include_str!("actions/commit.yaml"),
        // }),
        // Box::new(BuiltinPipeline {
        //     file: "describe.yaml",
        //     yaml: include_str!("actions/describe.yaml"),
        // }),
        // Box::new(BuiltinPipeline {
        //     file: "explain.yaml",
        //     yaml: include_str!("actions/explain.yaml"),
        // }),
        // Box::new(BuiltinPipeline {
        //     file: "extract.yaml",
        //     yaml: include_str!("actions/extract.yaml"),
        // }),
        // Box::new(BuiltinPipeline {
        //     file: "faq.yaml",
        //     yaml: include_str!("actions/faq.yaml"),
        // }),
        // Box::new(BuiltinPipeline {
        //     file: "fetch.yaml",
        //     yaml: include_str!("actions/fetch.yaml"),
        // }),
        // Box::new(BuiltinPipeline {
        //     file: "find.yaml",
        //     yaml: include_str!("actions/find.yaml"),
        // }),
        // Box::new(BuiltinPipeline {
        //     file: "mock.yaml",
        //     yaml: include_str!("actions/mock.yaml"),
        // }),
        // Box::new(BuiltinPipeline {
        //     file: "naming.yaml",
        //     yaml: include_str!("actions/naming.yaml"),
        // }),
        // Box::new(BuiltinPipeline {
        //     file: "regex.yaml",
        //     yaml: include_str!("actions/regex.yaml"),
        // }),
        // Box::new(BuiltinPipeline {
        //     file: "review.yaml",
        //     yaml: include_str!("actions/review.yaml"),
        // }),
        // Box::new(BuiltinPipeline {
        //     file: "scan.yaml",
        //     yaml: include_str!("actions/scan.yaml"),
        // }),
        // Box::new(BuiltinPipeline {
        //     file: "spellcheck.yaml",
        //     yaml: include_str!("actions/spellcheck.yaml"),
        // }),
        // Box::new(BuiltinPipeline {
        //     file: "synonyms.yaml",
        //     yaml: include_str!("actions/synonyms.yaml"),
        // }),
        // Box::new(BuiltinPipeline {
        //     file: "sysinfo.yaml",
        //     yaml: include_str!("actions/sysinfo.yaml"),
        // }),
        // Box::new(BuiltinPipeline {
        //     file: "tone.yaml",
        //     yaml: include_str!("actions/tone.yaml"),
        // }),
        // Box::new(BuiltinPipeline {
        //     file: "translate-deep.yaml",
        //     yaml: include_str!("actions/translate-deep.yaml"),
        // }),
        // Box::new(BuiltinPipeline {
        //     file: "translate-large.yaml",
        //     yaml: include_str!("actions/translate-large.yaml"),
        // }),
        // Box::new(BuiltinPipeline {
        //     file: "translate-small.yaml",
        //     yaml: include_str!("actions/translate-small.yaml"),
        // }),
        // Box::new(BuiltinPipeline {
        //     file: "whois.yaml",
        //     yaml: include_str!("actions/whois.yaml"),
        // }),
    ]
}
