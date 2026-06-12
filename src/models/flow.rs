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

/// One action flow: name, mode, steps, result source.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlowModel {
    /// Action name (used as CLI subcommand).
    pub name: String,
    /// Short description for help.
    pub about: String,
    /// Optional regex validation for the result.
    #[serde(default)]
    pub check: Option<String>,
    /// Automatically copy the final terminal output to the clipboard.
    #[serde(default)]
    pub clipboard: bool,
    /// CLI arguments.
    #[serde(default)]
    pub args: Vec<ArgActionModel>,
    /// Preparation steps.
    #[serde(default)]
    pub actions: Vec<ActionModel>,
}

impl FlowModel {
    /// Load and validate a FlowModel from a YAML file.
    pub fn load(path: &PathBuf) -> Result<Self> {
        let content = fs::read_to_string(path)?;
        let flow: Self = yaml_serde::from_str(&content)
            .map_err(|e| anyhow::anyhow!("Failed to parse {}: {}", path.display(), e))?;
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
        self
    }
}
