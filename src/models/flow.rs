//! Flow model — one YAML action file.
//! Defines a pipeline with trigger and preparation steps.

use anyhow::Result;
use clap::ArgMatches;
use image::ImageEncoder;
use image::codecs::png::PngEncoder;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use crate::configs::app::AppConfig;
use crate::models::action::ActionModel;
use crate::models::arg::ArgActionModel;
use crate::models::arg::ArgInput;
use crate::models::context::ContextModel;
use crate::utils;
use crate::validate::ValidateTrait;

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
    pub input_tags: HashMap<String, ContextModel>,
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
                        self.input_tags
                            .insert(arg.name.clone(), ContextModel::String(default.clone()));
                    }
                }
                ArgInput::Path => {
                    if let Some(value) = matches.get_one::<String>(&arg.name) {
                        let resolved = utils::path::resolve(value)?;
                        let path_str = resolved.display().to_string();
                        self.input_tags
                            .insert(arg.name.clone(), ContextModel::String(path_str));
                    } else if let Some(default) = &arg.default {
                        let resolved = self
                            .input_tags
                            .get(default.trim_start_matches('{').trim_end_matches('}'))
                            .cloned()
                            .unwrap_or_else(|| ContextModel::String(default.clone()));
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
                        self.input_tags
                            .insert(arg.name.clone(), ContextModel::String(default.clone()));
                    }
                }
                ArgInput::String => {
                    if let Some(value) = matches.get_one::<String>(&arg.name) {
                        self.input_tags
                            .insert(arg.name.clone(), ContextModel::String(value.clone()));
                    } else if let Some(default) = &arg.default {
                        let tag = default.trim_start_matches('{').trim_end_matches('}');
                        let resolved = self
                            .input_tags
                            .get(tag)
                            .cloned()
                            .unwrap_or_else(|| ContextModel::String(default.clone()));
                        self.input_tags.insert(arg.name.clone(), resolved);
                    }
                }
            }
        }
        Ok(self)
    }

    /// Add system tags to input arguments.
    fn apply_system_tags(mut self) -> Self {
        self.input_tags.insert(
            "system_pwd".into(),
            ContextModel::String(
                std::env::current_dir()
                    .map(|p| p.display().to_string())
                    .unwrap_or_default(),
            ),
        );
        self.input_tags.insert(
            "system_os".into(),
            ContextModel::String(std::env::consts::OS.into()),
        );
        self.input_tags.insert(
            "system_user".into(),
            ContextModel::String(std::env::var("USER").unwrap_or_default()),
        );
        self.input_tags.insert(
            "system_home".into(),
            ContextModel::String(std::env::var("HOME").unwrap_or_default()),
        );
        self.input_tags.insert(
            "system_date".into(),
            ContextModel::String(chrono::Local::now().format("%Y-%m-%d").to_string()),
        );
        self.input_tags.insert(
            "system_time".into(),
            ContextModel::String(chrono::Local::now().format("%H:%M:%S").to_string()),
        );
        self.input_tags.insert(
            "system_clipboard".into(),
            ContextModel::String(self.get_clipboard_text().unwrap_or_default()),
        );
        if let Ok(image_base64) = self.get_clipboard_image() {
            self.input_tags.insert(
                "system_clipboard_image".into(),
                ContextModel::String(image_base64),
            );
        }
        self.input_tags.insert(
            "system_pid".into(),
            ContextModel::String(std::process::id().to_string()),
        );
        self.input_tags.insert(
            "system_temp".into(),
            ContextModel::String(std::env::temp_dir().display().to_string()),
        );
        self
    }

    /// Get clipboard text with validation.
    pub fn get_clipboard_text(&self) -> Result<String> {
        let mut clipboard = arboard::Clipboard::new()
            .map_err(|e| anyhow::anyhow!("Failed to access clipboard: {}", e))?;
        let text = clipboard
            .get_text()
            .map_err(|_| anyhow::anyhow!("Clipboard is empty or contains non-text data."))?;
        let bpe = tiktoken_rs::cl100k_base().unwrap();
        let tokens = bpe.encode_with_special_tokens(&text).len();
        let config = AppConfig::instance()?;
        let max_ctx = config
            .cluster
            .iter()
            .map(|c| c.num_ctx)
            .max()
            .unwrap_or(4096);
        if tokens > max_ctx {
            anyhow::bail!(
                "Clipboard text is too large ({} tokens). Max context size is {} tokens.",
                tokens,
                max_ctx
            );
        }
        Ok(text)
    }

    /// Get clipboard image as base64 PNG string.
    pub fn get_clipboard_image(&self) -> Result<String> {
        let mut clipboard = arboard::Clipboard::new()
            .map_err(|e| anyhow::anyhow!("Failed to access clipboard: {}", e))?;
        let img = clipboard
            .get_image()
            .map_err(|_| anyhow::anyhow!("Clipboard does not contain an image."))?;
        let mut png_bytes = Vec::new();
        let encoder = PngEncoder::new(&mut png_bytes);
        encoder
            .write_image(
                &img.bytes,
                img.width as u32,
                img.height as u32,
                image::ExtendedColorType::Rgba8,
            )
            .map_err(|e| anyhow::anyhow!("Failed to encode PNG: {}", e))?;
        use base64::Engine;
        Ok(base64::engine::general_purpose::STANDARD.encode(&png_bytes))
    }
}
