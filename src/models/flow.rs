//! Flow model — one YAML action file.
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
use crate::models::api::FlowApiModel;
use crate::models::arg::ArgActionModel;
use crate::models::arg::ArgInput;
use crate::models::context::ContextModel;
use crate::query::query::QueryKey;
use crate::query::query::QueryRegistry;
use crate::system::system::SystemKey;
use crate::system::system::SystemRegistry;
use crate::utils;

/// One action flow: name, steps, result source, and CLI arguments.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlowModel {
    /// Flow version (defaults version, bump to force update).
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
    pub api: Option<FlowApiModel>,
    /// Preparation steps.
    #[serde(default)]
    pub actions: Vec<ActionModel>,
    /// Resolved argument values (name -> value).
    #[serde(skip, default)]
    pub input_tags: HashMap<String, ContextModel>,
}

impl FlowModel {
    /// Load and validate a FlowModel from a YAML file.
    pub fn load(path: &PathBuf) -> Result<Self> {
        let content = fs::read_to_string(path)?;
        let flow: Self = yaml_serde::from_str(&content)
            .map_err(|e| anyhow::anyhow!("Failed to parse {}: {}", path.display(), e))?;
        Ok(flow)
    }

    /// Resolve CLI arguments and store them in state.
    pub fn apply_args(mut self, matches: &ArgMatches) -> Result<Self> {
        for arg in &self.args {
            match &arg.input {
                ArgInput::Bool => {
                    let value = matches.get_flag(&arg.name);
                    self.input_tags
                        .insert(arg.name.clone(), ContextModel::String(value.to_string()));
                }
                ArgInput::Number => {
                    if let Some(value) = matches.get_one::<f64>(&arg.name) {
                        self.input_tags
                            .insert(arg.name.clone(), ContextModel::String(value.to_string()));
                    } else if let Some(default) = &arg.default {
                        let resolved = Self::resolve_default(default, &self.input_tags);
                        self.input_tags.insert(arg.name.clone(), resolved);
                    }
                }
                ArgInput::Path => {
                    if let Some(value) = matches.get_one::<String>(&arg.name) {
                        let resolved = utils::path::resolve(value)?;
                        let path_str = resolved.display().to_string();
                        self.input_tags
                            .insert(arg.name.clone(), ContextModel::String(path_str));
                    } else if let Some(default) = &arg.default {
                        let resolved = Self::resolve_default(default, &self.input_tags);
                        self.input_tags.insert(arg.name.clone(), resolved);
                    }
                }
                ArgInput::List(inner) => {
                    if let Some(values) = matches.get_many::<String>(&arg.name) {
                        let validated: Vec<String> = values
                            .cloned()
                            .map(|v| match inner.as_ref() {
                                ArgInput::Bool => match v.to_lowercase().as_str() {
                                    "true" | "false" | "yes" | "no" | "да" | "нет" => Ok(v),
                                    _ => Err(anyhow::anyhow!("Invalid bool value: {}", v)),
                                },
                                ArgInput::Number => {
                                    v.parse::<f64>().map_err(|e| {
                                        anyhow::anyhow!("Invalid number value '{}': {}", v, e)
                                    })?;
                                    Ok(v)
                                }
                                ArgInput::Path => utils::path::resolve(&v)
                                    .map(|p| p.display().to_string())
                                    .map_err(|e| anyhow::anyhow!("Invalid path '{}': {}", v, e)),
                                _ => Ok(v),
                            })
                            .collect::<Result<Vec<String>>>()?;
                        self.input_tags
                            .insert(arg.name.clone(), ContextModel::List(validated));
                    } else if let Some(default) = &arg.default {
                        let resolved = Self::resolve_default(default, &self.input_tags);
                        self.input_tags.insert(arg.name.clone(), resolved);
                    }
                }
                ArgInput::String => {
                    if let Some(value) = matches.get_one::<String>(&arg.name) {
                        self.input_tags
                            .insert(arg.name.clone(), ContextModel::String(value.clone()));
                    } else if let Some(default) = &arg.default {
                        let resolved = Self::resolve_default(default, &self.input_tags);
                        self.input_tags.insert(arg.name.clone(), resolved);
                    }
                }
            }
        }
        // Extract query positional argument if present
        if let Ok(Some(value)) = matches.try_get_one::<String>("query") {
            self.input_tags
                .insert("query".to_string(), ContextModel::String(value.clone()));
        }
        Ok(self)
    }

    /// Resolve a default value, supporting {system_*} tags.
    fn resolve_default(default: &str, input_tags: &HashMap<String, ContextModel>) -> ContextModel {
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
            .unwrap_or_else(|| ContextModel::String(default.to_string()))
    }

    /// Resolve {query} and {query|type} tags and store them in state.
    pub fn apply_query_tags(mut self) -> Result<Self> {
        let raw_query = self.input_tags.get("query").and_then(|v| {
            if let ContextModel::String(s) = v {
                Some(s.clone())
            } else {
                None
            }
        });

        let registry = QueryRegistry::new(raw_query);
        let mut needed_types = HashSet::new();

        for action in &self.actions {
            for text in action.actions() {
                for mat in TagIterator::new(text) {
                    if mat.base_tag == "query" {
                        if let Some(first_mod) = mat.modifiers.first() {
                            if QueryKey::from_str(&first_mod.name).is_some() {
                                needed_types.insert(first_mod.name.clone());
                            }
                        } else {
                            needed_types.insert("raw".to_string());
                        }
                    }
                }
            }
        }

        for type_name in needed_types {
            if let Some(key) = QueryKey::from_str(&type_name) {
                if let Ok(value) = registry.resolve(key) {
                    let context_key = if type_name == "raw" {
                        "query".to_string()
                    } else {
                        format!("query_{}", type_name)
                    };
                    self.input_tags.insert(context_key, value);
                }
            }
        }

        Ok(self)
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

    /// Check if the flow uses the {query} tag in any of its actions.
    pub fn uses_query(&self) -> bool {
        use crate::engine::parser::TagIterator;
        use crate::models::action::ActionValue;

        self.actions.iter().any(|action| {
            let check_text = |text: &str| TagIterator::new(text).any(|mat| mat.base_tag == "query");

            match &action.action {
                ActionValue::Simple(s) => check_text(s),
                ActionValue::Switch(cases) => cases
                    .iter()
                    .any(|case| check_text(&case.when) || check_text(&case.then)),
            }
        })
    }

    /// Check if the flow uses the {query|prompt} tag in any of its actions.
    pub fn needs_prompt(&self) -> bool {
        use crate::engine::parser::TagIterator;
        use crate::models::action::ActionValue;

        self.actions.iter().any(|action| {
            let check_text = |text: &str| {
                TagIterator::new(text).any(|mat| {
                    mat.base_tag == "query"
                        && mat.modifiers.first().map_or(false, |m| m.name == "prompt")
                })
            };

            match &action.action {
                ActionValue::Simple(s) => check_text(s),
                ActionValue::Switch(cases) => cases
                    .iter()
                    .any(|case| check_text(&case.when) || check_text(&case.then)),
            }
        })
    }
}
