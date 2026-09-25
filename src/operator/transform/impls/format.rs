//! Format operator — converts between JSON, YAML, TOML, and JSON5 formats.
//! Auto-detects input format and transforms to the specified output format.

use crate::operator::operator::ITEM_SEP;
use crate::operator::operator::Operator;
use crate::operator::operator::OperatorKey;
use crate::operator::transform::transform::TransformKey;
use anyhow::Result;

pub struct FormatOperator;

impl FormatOperator {
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

impl Operator for FormatOperator {
    fn key(&self) -> OperatorKey {
        TransformKey::Format.key()
    }

    fn apply(&self, value: &str, arg: &str) -> Result<String> {
        let output_format = if arg.is_empty() { "json" } else { arg };

        let results: Vec<String> = value
            .split(ITEM_SEP)
            .filter(|i| !i.is_empty())
            .map(|i| {
                let parsed = Self::parse_value(i)?;
                Self::format_value(&parsed, output_format)
            })
            .collect::<Result<Vec<_>>>()?;

        Self::wrap_list(results, output_format)
    }
}
