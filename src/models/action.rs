//! Action and val-candidate models, and ActionRun enum.
//! See [`crate::models`] module-level docs for the model hierarchy.

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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ValEach {
    Config { split: String, merge: String },
    Flag(bool),
}

impl ValEach {
    /// Resolve to (split, merge). Flag(true) defaults to \n / \n.
    pub fn resolve(&self) -> (String, String) {
        match self {
            ValEach::Config { split, merge } => (split.clone(), merge.clone()),
            ValEach::Flag(_) => ("\n".to_string(), "\n".to_string()),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValCandidate {
    pub name: String,
    #[serde(default)]
    pub data: Option<String>,
    #[serde(default)]
    pub mods: Option<String>,
    #[serde(default)]
    pub when: Option<String>,
    #[serde(default)]
    pub fail: Option<String>,
    #[serde(default)]
    pub each: Option<ValEach>,
    /// Resolved value (set at runtime, not serialized).
    #[serde(skip)]
    pub resolved: Option<String>,
}

/// Run guard: when the condition passes, the action is skipped (dead tag).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuardModel {
    /// Data source tag (same reference rules as val `data`).
    pub data: String,
    /// Inspect operators applied to the data value.
    pub when: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionModel {
    pub tag: String,
    pub run: ActionRun,
    #[serde(default)]
    pub reg: Option<String>,
    #[serde(default)]
    pub ask: bool,
    /// Skip the action (dead tag) when the guard passes.
    #[serde(default)]
    pub off: Option<GuardModel>,
    #[serde(default)]
    pub val: Option<Vec<ValCandidate>>,
    pub action: String,
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
