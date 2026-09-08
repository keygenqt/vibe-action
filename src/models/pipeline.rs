//! Pipeline model — one YAML action file.
//! Defines a pipeline with trigger and preparation steps.

use anyhow::Result;
use clap::ArgMatches;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;

use crate::engine::parser::TagIterator;
use crate::models::action::ActionModel;
use crate::models::api::PipelineApiModel;
use crate::models::arg::ArgActionModel;
use crate::models::arg::ArgInput;
use crate::system::system::SystemKey;
use crate::system::system::SystemRegistry;
use crate::utils;

/// One action pipeline: name, steps, result source, and CLI arguments.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineModel {
    /// Pipeline version (defaults version, bump to force update).
    pub version: String,
    /// Action name (used as CLI subcommand).
    pub name: String,
    /// Short description for help.
    pub about: String,
    /// Optional regex validation for the result.
    #[serde(default)]
    pub check: Option<String>,
    /// Show system notification on completion.
    #[serde(default)]
    pub notify: bool,
    /// CLI arguments.
    #[serde(default)]
    pub args: Vec<ArgActionModel>,
    /// IDE plugin integration metadata (ignored by CLI runtime).
    #[serde(default)]
    pub api: Option<PipelineApiModel>,
    /// Preparation steps.
    #[serde(default)]
    pub actions: Vec<ActionModel>,
    /// Resolved argument values (name -> value).
    #[serde(skip, default)]
    pub input_tags: HashMap<String, String>,
    /// Source YAML file path (set at load time, not serialized).
    #[serde(skip, default)]
    pub file_path: Option<PathBuf>,
}

impl PipelineModel {
    /// Load and validate a PipelineModel from a YAML file.
    pub fn load(path: &PathBuf) -> Result<Self> {
        let content = fs::read_to_string(path)?;
        let mut pipeline: Self = yaml_serde::from_str(&content)
            .map_err(|e| anyhow::anyhow!("Failed to parse {}: {}", path.display(), e))?;
        pipeline.file_path = Some(path.clone());
        Ok(pipeline)
    }

    /// Resolve CLI arguments and store them in state.
    pub fn apply_args(mut self, matches: &ArgMatches) -> Result<Self> {
        for arg in &self.args {
            match &arg.input {
                ArgInput::Bool => {
                    let value = matches.get_flag(&arg.name);
                    self.input_tags.insert(arg.name.clone(), value.to_string());
                }
                ArgInput::Number => {
                    if let Some(value) = matches.get_one::<f64>(&arg.name) {
                        self.input_tags.insert(arg.name.clone(), value.to_string());
                    } else if let Some(default) = &arg.default {
                        let resolved = Self::resolve_default(default, &self.input_tags);
                        self.input_tags.insert(arg.name.clone(), resolved);
                    }
                }
                ArgInput::Path => {
                    if let Some(value) = matches.get_one::<String>(&arg.name) {
                        let resolved = utils::path::resolve(value)?;
                        let path_str = resolved.display().to_string();
                        self.input_tags.insert(arg.name.clone(), path_str);
                    } else if let Some(default) = &arg.default {
                        let resolved = Self::resolve_default(default, &self.input_tags);
                        self.input_tags.insert(arg.name.clone(), resolved);
                    }
                }
                ArgInput::String => {
                    if let Some(value) = matches.get_one::<String>(&arg.name) {
                        self.input_tags.insert(arg.name.clone(), value.clone());
                    } else if let Some(default) = &arg.default {
                        let resolved = Self::resolve_default(default, &self.input_tags);
                        self.input_tags.insert(arg.name.clone(), resolved);
                    }
                }
            }
        }
        // Extract query positional argument if present
        if let Ok(Some(value)) = matches.try_get_one::<String>("query") {
            self.input_tags.insert("query".to_string(), value.clone());
        }
        Ok(self)
    }

    /// Resolve a default value, supporting {system_*} tags.
    fn resolve_default(default: &str, input_tags: &HashMap<String, String>) -> String {
        let tag = default.trim_start_matches('{').trim_end_matches('}');
        if tag.starts_with("system_") {
            let registry = SystemRegistry::new();
            let system_key = SystemKey::all().iter().find(|k| k.as_str() == tag);
            if let Some(key) = system_key {
                if let Ok(value) = registry.resolve(*key) {
                    return value;
                }
            }
        }
        input_tags
            .get(tag)
            .cloned()
            .unwrap_or_else(|| default.to_string())
    }

    /// Add system tags to input arguments.
    pub fn apply_system_tags(mut self) -> Result<Self> {
        let registry = SystemRegistry::new();
        let mut needed_tags = HashSet::new();

        for action in &self.actions {
            for text in action.actions() {
                for mat in TagIterator::new(text) {
                    if mat.base_tag.starts_with("system_") {
                        needed_tags.insert(mat.base_tag.clone());
                    }
                }
            }
        }
        for tag in needed_tags {
            let system_key = SystemKey::all().iter().find(|k| k.as_str() == tag.as_str());
            if let Some(key) = system_key {
                let value = registry.resolve(*key)?;
                self.input_tags.insert(tag, value);
            }
        }
        Ok(self)
    }

    /// Check if the pipeline uses the {query} tag in any of its actions.
    pub fn uses_query(&self) -> bool {
        use crate::engine::parser::TagIterator;

        self.actions.iter().any(|action| {
            // Old-style: {query} in action text.
            if TagIterator::new(&action.action).any(|mat| mat.base_tag == "query") {
                return true;
            }
            // val-style: query referenced in a candidate's `data`.
            action.val.as_ref().map_or(false, |cands| {
                cands
                    .iter()
                    .any(|c| c.data.split('|').next().unwrap_or("") == "query")
            })
        })
    }

    /// Check if the pipeline uses the {query|prompt} tag in any of its actions.
    pub fn needs_prompt(&self) -> bool {
        use crate::engine::parser::TagIterator;

        self.actions.iter().any(|action| {
            // Old-style: {query|prompt} in action text.
            if TagIterator::new(&action.action).any(|mat| {
                mat.base_tag == "query"
                    && mat.modifiers.first().map_or(false, |m| m.name == "prompt")
            }) {
                return true;
            }
            // val-style: query|prompt in a candidate's `data`.
            action.val.as_ref().map_or(false, |cands| {
                cands.iter().any(|c| c.data == "query|prompt")
            })
        })
    }
}
