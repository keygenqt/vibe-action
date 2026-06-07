//! Flow model — one YAML action file.
//! Defines a pipeline with trigger and preparation steps.

use anyhow::Result;
use clap::ArgMatches;
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

use crate::{
    models::{action::ActionModel, arg::ArgActionModel},
    validate::ValidateTrait,
};

/// Flow execution mode — controls what happens with the trigger result.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum FlowMode {
    /// Print the final result to console (default).
    Output,
    /// Execute the trigger command without confirmation.
    Exec,
    /// Ask user for confirmation (Y/N) before executing.
    Ask,
}

impl Default for FlowMode {
    fn default() -> Self {
        FlowMode::Output
    }
}

/// One action flow: name, args, trigger, steps.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlowModel {
    /// Action name (used as CLI subcommand).
    pub name: String,
    /// Execution mode for the trigger.
    #[serde(default)] // default = Output
    pub mode: FlowMode,
    /// Short description for help.
    pub about: String,
    /// CLI arguments.
    #[serde(default)]
    pub args: Vec<ArgActionModel>,
    /// Main action executed last.
    pub trigger: ActionModel,
    /// Preparation steps executed before trigger.
    #[serde(default)]
    pub actions: Vec<ActionModel>,
    /// File path this flow was loaded from (for save/reload).
    #[serde(skip)]
    pub path: PathBuf,
}

impl FlowModel {
    /// Load and validate a FlowModel from a YAML file.
    pub fn load(path: &PathBuf) -> Result<Self> {
        let content = fs::read_to_string(path)?;
        let mut flow: Self = yaml_serde::from_str(&content)
            .map_err(|e| anyhow::anyhow!("Failed to parse {}: {}", path.display(), e))?;
        flow.path = path.clone();
        flow.validate()?;
        Ok(flow)
    }

    /// Apply CLI arguments by replacing {arg} placeholders in all action strings.
    pub fn apply_args(mut self, matches: &ArgMatches) -> Self {
        for arg in &mut self.args {
            arg.resolve_values(matches);
        }
        for action in &mut self.actions {
            for arg in &self.args {
                for v in &arg.values {
                    action.action = action.action.replace(&format!("{{{}}}", v.name), &v.value);
                }
            }
        }
        for arg in &self.args {
            for v in &arg.values {
                self.trigger.action = self
                    .trigger
                    .action
                    .replace(&format!("{{{}}}", v.name), &v.value);
            }
        }
        self
    }
}
