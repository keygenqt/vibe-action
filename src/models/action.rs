//! Action models for YAML parsing.
//! Defines structures for action files, trigger and steps.

use serde::Deserialize;
use serde::Serialize;

use crate::models::context::ContextModel;

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
    pub expect: Option<ContextModel>,
    /// Optional regex validation for the result.
    #[serde(default)]
    pub check: Option<String>,
    /// Ask for confirmation before executing.
    #[serde(default)]
    pub confirm: bool,
    /// @todo
    #[serde(default)]
    pub when: Option<String>,
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
