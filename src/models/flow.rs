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

/// Visual style density for rendering the final result in the terminal.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum FlowFormat {
    /// Minimum overhead. Just a standard status line: 'success: <result>'.
    Compact,
    /// High visibility. Wrapped inside a beautifully formatted UI block.
    Rich,
}

impl Default for FlowFormat {
    fn default() -> Self {
        FlowFormat::Compact
    }
}

/// One action flow: name, mode, steps, result source.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlowModel {
    /// Action name (used as CLI subcommand).
    pub name: String,
    /// Tag name in context to use as final result.
    pub output: String,
    /// How to handle the final result (Output, Exec, Ask, Clip).
    #[serde(default)]
    pub format: FlowFormat,
    /// Short description for help.
    pub about: String,
    /// Optional regex validation for the result.
    #[serde(default)]
    pub r#match: Option<String>,
    /// Automatically copy the final terminal output to the clipboard.
    #[serde(default)]
    pub clipboard: bool,
    /// CLI arguments.
    #[serde(default)]
    pub args: Vec<ArgActionModel>,
    /// Preparation steps.
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
        self
    }
}
