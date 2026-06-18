//! Action models for YAML parsing.
//! Defines structures for action files, trigger and steps.

use anyhow::Result;
use serde::{Deserialize, Serialize, Serializer};

/// Action runner type.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ActionRun {
    /// Execute command in terminal.
    Cmd,
    /// Send prompt to LLM (all nodes).
    Llm,
    /// Send prompt to small LLM.
    #[serde(rename = "llm_small")]
    LlmSmall,
    /// Send prompt to medium LLM.
    #[serde(rename = "llm_medium")]
    LlmMedium,
    /// Send prompt to large LLM.
    #[serde(rename = "llm_large")]
    LlmLarge,
    /// Return the action string directly (no shell, no LLM).
    Value,
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
    /// List of values.
    List(Box<ExpectMode>),
}

/// A single switch case.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SwitchCase {
    Case { case: String, action: String },
    Else { action: String },
}

/// A single action step.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionModel {
    /// Tag name for {tag} references.
    pub tag: String,
    /// Action runner type.
    pub run: ActionRun,
    /// Expected result type.
    pub expect: ExpectMode,
    /// Optional regex validation for the result.
    #[serde(default)]
    pub check: Option<String>,
    /// Ask for confirmation before executing.
    #[serde(default)]
    pub confirm: bool,
    /// Command or prompt (optional if switch is set).
    #[serde(default)]
    pub action: Option<String>,
    /// Switch cases for conditional execution.
    #[serde(default)]
    pub switch: Option<Vec<SwitchCase>>,
}

impl ActionModel {
    /// Return all action texts for dependency scanning.
    pub fn actions(&self) -> Vec<&str> {
        let mut texts = Vec::new();
        if let Some(action) = &self.action {
            texts.push(action.as_str());
        }
        if let Some(switch) = &self.switch {
            for branch in switch {
                match branch {
                    SwitchCase::Case { case, action } => {
                        texts.push(case.as_str());
                        texts.push(action.as_str());
                    }
                    SwitchCase::Else { action } => {
                        texts.push(action.as_str());
                    }
                }
            }
        }
        texts
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
                ExpectMode::List(inner) => format!("list<{}>", stringify(inner)),
            }
        }

        serializer.serialize_str(&stringify(self))
    }
}

impl std::fmt::Display for ActionRun {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ActionRun::Cmd => write!(f, "cmd"),
            ActionRun::Llm | ActionRun::LlmSmall | ActionRun::LlmMedium | ActionRun::LlmLarge => {
                write!(f, "llm")
            }
            ActionRun::Value => write!(f, "val"),
        }
    }
}
