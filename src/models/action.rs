//! Action models for YAML parsing.
//! Defines structures for action files, trigger and steps.

use anyhow::Result;
use serde::{Deserialize, Serialize, Serializer};

/// Action type: command or LLM.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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
#[derive(Debug, Clone, Serialize, Deserialize)]
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

impl Serialize for ExpectMode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Выносим рекурсивное форматирование в отдельную строковую логику
        fn stringify(mode: &ExpectMode) -> String {
            match mode {
                ExpectMode::Void => "void".to_string(),
                ExpectMode::Bool => "bool".to_string(),
                ExpectMode::Number => "number".to_string(),
                ExpectMode::String => "string".to_string(),
                ExpectMode::Json => "json".to_string(),
                ExpectMode::List(inner) => format!("list<{}>", stringify(inner)),
            }
        }

        serializer.serialize_str(&stringify(self))
    }
}

impl std::fmt::Display for ActionMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ActionMode::Cmd => write!(f, "cmd"),
            ActionMode::Llm => write!(f, "llm"),
        }
    }
}
