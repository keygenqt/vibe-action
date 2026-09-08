//! Action models for YAML parsing.
//! Defines structures for action files, trigger and steps.

use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ActionRun {
    Cmd,
    Value,
    Tiny,
    Small,
    Medium,
    Large,
    Vision,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValEach {
    pub split: String,
    pub merge: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValCandidate {
    pub name: String,
    pub data: String,
    #[serde(default)]
    pub mods: Option<String>,
    #[serde(default)]
    pub when: Option<String>,
    #[serde(default)]
    pub each: Option<ValEach>,
    /// Resolved value (set at runtime, not serialized).
    #[serde(skip)]
    pub resolved: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionModel {
    pub tag: String,
    pub run: ActionRun,
    #[serde(default)]
    pub check: Option<String>,
    #[serde(default)]
    pub confirm: bool,
    #[serde(default)]
    pub when: Option<String>,
    #[serde(default)]
    pub val: Option<Vec<ValCandidate>>,
    pub action: String,
}

impl ActionModel {
    pub fn actions(&self) -> Vec<&str> {
        if self.action.is_empty() {
            Vec::new()
        } else {
            vec![self.action.as_str()]
        }
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
