//! Default flow trait — validates built-in YAML flows from embedded YAML files.

use crate::models::flow::FlowModel;
use anyhow::Result;

/// Common header template for all built-in YAML flows.
pub const FLOW_HEADER: &str = r#"# Vibe Action — {{name}}
# {{about}}
# Auto-updated when FLOW_VERSION changes. Custom edits to default flows will be overwritten.
"#;

pub trait DefaultFlow {
    /// Raw YAML flow body (without header).
    fn raw(&self) -> &'static str;

    /// Returns the YAML content for this flow (header + validated body).
    fn flow(&self) -> Result<String> {
        let raw = self.raw();
        let model = self.model(raw)?;
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
    fn model(&self, raw: &str) -> Result<FlowModel>;
}

/// A built-in flow with embedded YAML.
pub struct BuiltinFlow {
    pub file: &'static str,
    pub yaml: &'static str,
}

impl DefaultFlow for BuiltinFlow {
    fn raw(&self) -> &'static str {
        self.yaml
    }

    fn model(&self, raw: &str) -> Result<FlowModel> {
        yaml_serde::from_str(raw)
            .map_err(|e| anyhow::anyhow!("Failed to parse built-in flow '{}': {}", self.file, e))
    }
}

/// Returns all built-in default flows.
pub fn default_flows() -> Vec<Box<dyn DefaultFlow>> {
    vec![
        Box::new(BuiltinFlow {
            file: "comment.yaml",
            yaml: include_str!("actions/comment.yaml"),
        }),
        Box::new(BuiltinFlow {
            file: "commit.yaml",
            yaml: include_str!("actions/commit.yaml"),
        }),
        Box::new(BuiltinFlow {
            file: "describe.yaml",
            yaml: include_str!("actions/describe.yaml"),
        }),
        Box::new(BuiltinFlow {
            file: "explain.yaml",
            yaml: include_str!("actions/explain.yaml"),
        }),
        Box::new(BuiltinFlow {
            file: "extract.yaml",
            yaml: include_str!("actions/extract.yaml"),
        }),
        Box::new(BuiltinFlow {
            file: "faq.yaml",
            yaml: include_str!("actions/faq.yaml"),
        }),
        Box::new(BuiltinFlow {
            file: "fetch.yaml",
            yaml: include_str!("actions/fetch.yaml"),
        }),
        Box::new(BuiltinFlow {
            file: "find.yaml",
            yaml: include_str!("actions/find.yaml"),
        }),
        Box::new(BuiltinFlow {
            file: "mock.yaml",
            yaml: include_str!("actions/mock.yaml"),
        }),
        Box::new(BuiltinFlow {
            file: "naming.yaml",
            yaml: include_str!("actions/naming.yaml"),
        }),
        Box::new(BuiltinFlow {
            file: "regex.yaml",
            yaml: include_str!("actions/regex.yaml"),
        }),
        Box::new(BuiltinFlow {
            file: "review.yaml",
            yaml: include_str!("actions/review.yaml"),
        }),
        Box::new(BuiltinFlow {
            file: "scan.yaml",
            yaml: include_str!("actions/scan.yaml"),
        }),
        Box::new(BuiltinFlow {
            file: "spellcheck.yaml",
            yaml: include_str!("actions/spellcheck.yaml"),
        }),
        Box::new(BuiltinFlow {
            file: "synonyms.yaml",
            yaml: include_str!("actions/synonyms.yaml"),
        }),
        Box::new(BuiltinFlow {
            file: "sysinfo.yaml",
            yaml: include_str!("actions/sysinfo.yaml"),
        }),
        Box::new(BuiltinFlow {
            file: "tone.yaml",
            yaml: include_str!("actions/tone.yaml"),
        }),
        Box::new(BuiltinFlow {
            file: "translate-deep.yaml",
            yaml: include_str!("actions/translate-deep.yaml"),
        }),
        Box::new(BuiltinFlow {
            file: "translate-large.yaml",
            yaml: include_str!("actions/translate-large.yaml"),
        }),
        Box::new(BuiltinFlow {
            file: "translate-small.yaml",
            yaml: include_str!("actions/translate-small.yaml"),
        }),
        Box::new(BuiltinFlow {
            file: "whois.yaml",
            yaml: include_str!("actions/whois.yaml"),
        }),
    ]
}
