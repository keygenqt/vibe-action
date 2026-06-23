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
    /// Return the action string directly (no shell, no LLM).
    Value,
    /// Send prompt to tiny LLM.
    Tiny,
    /// Send prompt to small LLM.
    Small,
    /// Send prompt to medium LLM.
    Medium,
    /// Send prompt to large LLM.
    Large,
    /// Send prompt to vision LLM.
    Vision,
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

/// Action value: simple string or switch with when/then pairs.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ActionValue {
    Simple(String),
    Switch(Vec<SwitchCase>),
}

/// A single switch case.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwitchCase {
    pub when: String,
    pub then: String,
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
    /// Action value: simple string or switch with when/then pairs.
    pub action: ActionValue,
}

impl ActionModel {
    /// Return all action texts for dependency scanning.
    pub fn actions(&self) -> Vec<&str> {
        let mut texts = Vec::new();
        match &self.action {
            ActionValue::Simple(s) => {
                if !s.is_empty() {
                    texts.push(s.as_str());
                }
            }
            ActionValue::Switch(cases) => {
                for case in cases {
                    texts.push(case.when.as_str());
                    texts.push(case.then.as_str());
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
            ActionRun::Value => write!(f, "val"),
            ActionRun::Tiny => write!(f, "tiny"),
            ActionRun::Small => write!(f, "small"),
            ActionRun::Medium => write!(f, "medium"),
            ActionRun::Large => write!(f, "large"),
            ActionRun::Vision => write!(f, "vision"),
        }
    }
}
