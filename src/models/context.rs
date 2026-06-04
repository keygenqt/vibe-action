//! Runtime context model for tag values.
//! Stores resolved action results with type information.

use anyhow::Result;
use regex::Regex;
use serde_json::Value;

use crate::models::action::ExpectMode;

/// A resolved tag value with its type.
#[derive(Debug, Clone)]
pub enum ContextModel {
    Void,
    Bool(bool),
    Number(f64),
    String(String),
    Json(Value),
    List(Vec<ContextModel>),
}

impl ContextModel {
    /// Parse and validate a string into ContextModel.
    pub fn from_str(
        tag: &str,
        value: &str,
        expect: &ExpectMode,
        match_regex: Option<&Regex>,
    ) -> Result<Self> {
        // Validate regex first (fail fast).
        if let Some(re) = match_regex {
            if !re.is_match(value.trim()) {
                anyhow::bail!(
                    "Validation {{{tag}}} failed: value does not match pattern '{}'. Got: '{}'",
                    re.as_str(),
                    if value.len() > 50 {
                        &value[..50]
                    } else {
                        value
                    }
                );
            }
        }
        // Then parse into typed value.
        Self::parse(value, expect)
    }

    /// Parse a string into ContextModel based on expected type.
    fn parse(value: &str, expect: &ExpectMode) -> Result<Self> {
        match expect {
            ExpectMode::Void => Ok(ContextModel::Void),
            ExpectMode::Bool => {
                let lower = value.trim().to_lowercase();
                match lower.as_str() {
                    "true" | "yes" | "да" | "1" => Ok(ContextModel::Bool(true)),
                    "false" | "no" | "нет" | "0" => Ok(ContextModel::Bool(false)),
                    _ => anyhow::bail!("Expected bool, got: '{}'", value),
                }
            }
            ExpectMode::Number => {
                let clean: String = value.trim().replace(',', ".");
                let num: f64 = clean
                    .parse()
                    .map_err(|_| anyhow::anyhow!("Expected number, got: '{}'", value))?;
                Ok(ContextModel::Number(num))
            }
            ExpectMode::String => Ok(ContextModel::String(value.trim().to_string())),
            ExpectMode::Json => {
                let json: Value = serde_json::from_str(value).map_err(|e| {
                    anyhow::anyhow!("Expected JSON, got: '{}'. Error: {}", value, e)
                })?;
                Ok(ContextModel::Json(json))
            }
            ExpectMode::List(inner) => {
                let trimmed = value.trim();
                // Try JSON array first.
                if trimmed.starts_with('[') && trimmed.ends_with(']') {
                    let json_array: Value = serde_json::from_str(trimmed)
                        .map_err(|_| anyhow::anyhow!("Invalid list JSON: '{}'", trimmed))?;
                    if let Value::Array(elements) = json_array {
                        let mut parsed = Vec::with_capacity(elements.len());
                        for el in elements {
                            let el_str = match el {
                                Value::String(s) => s,
                                other => other.to_string(),
                            };
                            parsed.push(Self::parse(&el_str, inner)?);
                        }
                        return Ok(ContextModel::List(parsed));
                    }
                }
                // Fallback: split by newline.
                let lines: Vec<&str> = trimmed
                    .lines()
                    .map(|l| l.trim())
                    .filter(|l| !l.is_empty())
                    .collect();
                if lines.is_empty() {
                    return Ok(ContextModel::List(vec![]));
                }
                let mut parsed = Vec::with_capacity(lines.len());
                for line in lines {
                    parsed.push(Self::parse(line, inner)?);
                }
                Ok(ContextModel::List(parsed))
            }
        }
    }
}

impl std::fmt::Display for ContextModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ContextModel::Void => write!(f, ""),
            ContextModel::Bool(v) => write!(f, "{}", v),
            ContextModel::Number(v) => write!(f, "{}", v),
            ContextModel::String(v) => write!(f, "{}", v),
            ContextModel::Json(v) => write!(f, "{}", v),
            ContextModel::List(items) => {
                let strings: Vec<String> = items.iter().map(|i| i.to_string()).collect();
                write!(f, "{}", strings.join("\n"))
            }
        }
    }
}
