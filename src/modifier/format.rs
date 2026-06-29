//! Format modifier — converts between JSON, YAML, TOML, and JSON5 formats.
//! Auto-detects input format and transforms to the specified output format.

use anyhow::Result;

use super::modifier::Modifier;
use super::modifier::ModifierKey;
use crate::models::context::ContextModel;

pub struct FormatModifier;

impl FormatModifier {
    /// Try to parse input as JSON, JSON5, YAML, or TOML. First successful wins.
    fn parse_value(value: &str) -> Result<serde_json::Value> {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            return Ok(serde_json::Value::Null);
        }
        if let Ok(v) = serde_json::from_str(trimmed) {
            return Ok(v);
        }
        if let Ok(v) = json5::from_str(trimmed) {
            return Ok(v);
        }
        if let Ok(v) = yaml_serde::from_str::<serde_json::Value>(trimmed) {
            return Ok(v);
        }
        if let Ok(v) = toml::from_str::<toml::Value>(trimmed) {
            if let Ok(json_val) = serde_json::to_value(v) {
                return Ok(json_val);
            }
        }
        anyhow::bail!("Cannot parse input. Tried json, json5, yaml, toml.")
    }

    /// Convert serde_json::Value to the specified output format.
    fn format_value(value: &serde_json::Value, format: &str) -> Result<String> {
        match format {
            "json" => Ok(serde_json::to_string(value)?),
            "json5" => Ok(json5::to_string(value)?),
            "yaml" => Ok(yaml_serde::to_string(value)?),
            "toml" => {
                let v: toml::Value = serde_json::from_value(value.clone())?;
                Ok(toml::to_string(&v)?)
            }
            _ => anyhow::bail!(
                "Unknown output format: '{}'. Use json, json5, yaml, or toml.",
                format
            ),
        }
    }

    /// Wrap formatted items into the appropriate container for the output format.
    fn wrap_list(items: Vec<String>, format: &str) -> Result<String> {
        match format {
            "json" => Ok(format!("[{}]", items.join(","))),
            "json5" => Ok(format!("[{}]", items.join(",\n"))),
            "yaml" => Ok(items.join("\n")),
            "toml" => anyhow::bail!("TOML does not support top-level arrays. Use JSON or YAML."),
            _ => anyhow::bail!("Unknown output format: '{}'", format),
        }
    }
}

impl Modifier for FormatModifier {
    fn key(&self) -> ModifierKey {
        ModifierKey::Format
    }

    fn apply(&self, value: &ContextModel, arg: &str) -> Result<ContextModel> {
        let output_format = if arg.is_empty() { "json" } else { arg };

        match value {
            ContextModel::String(s) => {
                let parsed = Self::parse_value(s)?;
                let formatted = Self::format_value(&parsed, output_format)?;
                Ok(ContextModel::String(formatted))
            }
            ContextModel::List(items) => {
                let results: Vec<String> = items
                    .iter()
                    .filter(|i| !i.is_empty())
                    .map(
                        |i| match self.apply(&ContextModel::String(i.clone()), arg)? {
                            ContextModel::String(s) => Ok(s),
                            _ => unreachable!(),
                        },
                    )
                    .collect::<Result<Vec<_>>>()?;
                let formatted = Self::wrap_list(results, output_format)?;
                Ok(ContextModel::String(formatted))
            }
        }
    }
}
