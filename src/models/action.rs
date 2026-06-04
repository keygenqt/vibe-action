//! Action models for YAML parsing.
//! Defines structures for action files, trigger and steps.

use anyhow::Result;
use regex::Regex;
use serde::Deserialize;

/// Action type: command or LLM.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ActionMode {
    /// Execute command in terminal.
    Cmd,
    /// Send prompt to LLM.
    Llm,
}

/// Expected result type.
#[derive(Debug, Clone, PartialEq)]
pub enum ExpectMode {
    /// No result.
    Void,
    /// Boolean (true/false).
    Bool,
    /// Integer or float.
    Number,
    /// Text.
    String,
    /// Valid JSON.
    Json,
    /// List of values.
    List(Box<ExpectMode>),
}

/// A single action step.
#[derive(Debug, Clone, Deserialize)]
pub struct ActionModel {
    /// Tag name for {tag} references.
    pub tag: String,
    /// Action type.
    pub r#type: ActionMode,
    /// Expected result type.
    pub expect: ExpectMode,
    /// Optional regex pattern for validation.
    #[serde(default)]
    pub r#match: Option<String>,
    /// Command or prompt.
    pub action: String,
}

impl ActionModel {
    /// Validate action fields.
    pub fn validate(&self) -> Result<()> {
        // Action cannot be empty.
        if self.action.is_empty() || self.action.chars().all(|c| c.is_whitespace()) {
            anyhow::bail!("Action '{}' has empty command/prompt.", self.tag);
        }
        // LLM must have expect set (need to know what to parse).
        if self.r#type == ActionMode::Llm && self.expect == ExpectMode::Void {
            anyhow::bail!(
                "LLM action '{}' cannot have expect: void. Specify what to expect.",
                self.tag
            );
        }
        // Validate match regex if present.
        if let Some(pattern) = &self.r#match {
            if matches!(self.expect, ExpectMode::Json) {
                anyhow::bail!(
                    "Action '{}' has a 'match' regex pattern, but 'expect' is set to Json. \
                    Regex validation is not supported for structured JSON objects.",
                    self.tag
                );
            }
            regex::Regex::new(pattern).map_err(|e| {
                anyhow::anyhow!("Action '{}' has invalid match regex: {}", self.tag, e)
            })?;
        }

        // Compile match regex if present.
        if let Some(pattern) = &self.r#match {
            Regex::new(pattern).map_err(|e| {
                anyhow::anyhow!("Action '{}' has invalid match regex: {}", self.tag, e)
            })?;
        }

        Ok(())
    }
}

/// Custom deserializer for ExpectMode.
/// Supports: void, bool, number, string, json, list<string>, list<list<number>>, etc.
impl<'de> Deserialize<'de> for ExpectMode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        if s == "list" {
            return Err(serde::de::Error::custom(
                "Expected 'list<T>', e.g. 'list<string>'. Bare 'list' is not allowed.",
            ));
        }
        if let Some(inner_str) = s.strip_prefix("list<").and_then(|s| s.strip_suffix('>')) {
            let inner = ExpectMode::deserialize(
                serde::de::value::StrDeserializer::<D::Error>::new(inner_str),
            )?;
            return Ok(ExpectMode::List(Box::new(inner)));
        }
        match s.as_str() {
            "void" => Ok(ExpectMode::Void),
            "bool" => Ok(ExpectMode::Bool),
            "number" => Ok(ExpectMode::Number),
            "string" => Ok(ExpectMode::String),
            "json" => Ok(ExpectMode::Json),
            _ => Err(serde::de::Error::custom(format!(
                "Unknown expect type: {}",
                s
            ))),
        }
    }
}
