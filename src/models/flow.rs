//! Flow model — one YAML action file.
//! Defines a pipeline with trigger and preparation steps.

use anyhow::Result;
use clap::ArgMatches;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs, path::PathBuf};

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
    /// Show system notification on completion.
    #[serde(default)]
    pub notify: bool,
    /// CLI arguments.
    #[serde(default)]
    pub args: Vec<ArgActionModel>,
    /// Preparation steps.
    #[serde(default)]
    pub actions: Vec<ActionModel>,
    /// Resolved argument values (name -> value).
    #[serde(skip, default)]
    pub input_tags: HashMap<String, String>,
}

impl FlowModel {
    /// Load and validate a FlowModel from a YAML file.
    pub fn load(path: &PathBuf) -> Result<Self> {
        let content = fs::read_to_string(path)?;
        let flow: Self = yaml_serde::from_str(&content)
            .map_err(|e| anyhow::anyhow!("Failed to parse {}: {}", path.display(), e))?;
        flow.validate()?;
        Ok(flow.apply_system_tags())
    }

    /// Resolve CLI arguments and store them in state.
    pub fn apply_args(mut self, matches: &ArgMatches) -> Self {
        for arg in &self.args {
            if let Some(value) = matches.get_one::<String>(&arg.name) {
                self.input_tags.insert(arg.name.clone(), value.clone());
            } else if let Some(default) = &arg.default {
                let tag = default.trim_start_matches('{').trim_end_matches('}');
                let resolved = self
                    .input_tags
                    .get(tag)
                    .cloned()
                    .unwrap_or_else(|| default.clone());
                self.input_tags.insert(arg.name.clone(), resolved);
            }
        }
        self
    }

    /// Add system tags to input arguments.
    fn apply_system_tags(mut self) -> Self {
        // Current working directory.
        self.input_tags.insert(
            "system_pwd".into(),
            std::env::current_dir()
                .map(|p| p.display().to_string())
                .unwrap_or_default(),
        );

        // Operating system.
        self.input_tags
            .insert("system_os".into(), std::env::consts::OS.into());

        // Current user.
        self.input_tags.insert(
            "system_user".into(),
            std::env::var("USER").unwrap_or_default(),
        );

        // Home directory.
        self.input_tags.insert(
            "system_home".into(),
            std::env::var("HOME").unwrap_or_default(),
        );

        // Current date (ISO 8601).
        self.input_tags.insert(
            "system_date".into(),
            chrono::Local::now().format("%Y-%m-%d").to_string(),
        );

        // Current time.
        self.input_tags.insert(
            "system_time".into(),
            chrono::Local::now().format("%H:%M:%S").to_string(),
        );

        // Clipboard content.
        if let Ok(mut clipboard) = arboard::Clipboard::new() {
            self.input_tags.insert(
                "system_clipboard".into(),
                clipboard.get_text().unwrap_or_default(),
            );
        }

        // Process ID.
        self.input_tags
            .insert("system_pid".into(), std::process::id().to_string());

        // Temporary directory.
        self.input_tags.insert(
            "system_temp".into(),
            std::env::temp_dir().display().to_string(),
        );
        self
    }
}
