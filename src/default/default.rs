//! Default flow trait — validates built-in YAML flows from embedded YAML files.

use crate::models::flow::FlowModel;
use crate::validate::ValidateTrait;
use anyhow::Result;

/// Common header template for all built-in YAML flows.
pub const FLOW_HEADER: &str = r#"# Vibe Action — {{name}}
# {{about}}
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
            yaml: include_str!("actions/describe.yaml"),
        }),
        Box::new(BuiltinFlow {
            yaml: include_str!("actions/explain.yaml"),
        }),
        Box::new(BuiltinFlow {
            yaml: include_str!("actions/extract.yaml"),
        }),
        Box::new(BuiltinFlow {
            yaml: include_str!("actions/faq.yaml"),
        }),
        Box::new(BuiltinFlow {
            yaml: include_str!("actions/fetch.yaml"),
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
        Box::new(BuiltinFlow {
            yaml: include_str!("actions/whois.yaml"),
        }),
    ]
}
