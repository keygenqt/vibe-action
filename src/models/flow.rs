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
use crate::models::arg::ArgExpect;
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

            // Resolve Image type: file path → base64
            if arg.expect == ArgExpect::Image {
                if let Some(value) = self.input_tags.get(&arg.name) {
                    let resolved = Self::resolve_image(value);
                    self.input_tags.insert(arg.name.clone(), resolved);
                }
            }
        }
        self
    }

    /// Convert image file path to base64, or return as-is if already base64.
    fn resolve_image(value: &str) -> String {
        // Already base64
        if value.starts_with("data:") || value.starts_with("iVBOR") || value.starts_with("/9j/") {
            return value.to_string();
        }
        // Try as file path — resolve ~, ., ..
        let resolved = crate::utils::path::resolve(value).unwrap_or_else(|_| PathBuf::from(value));
        if resolved.exists() {
            std::fs::read(&resolved)
                .ok()
                .map(|bytes| {
                    use base64::Engine;
                    base64::engine::general_purpose::STANDARD.encode(&bytes)
                })
                .unwrap_or_else(|| value.to_string())
        } else {
            value.to_string()
        }
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

        // Clipboard content — detect type and store both
        self.input_tags.insert(
            "system_clipboard".into(),
            self.get_clipboard_text().unwrap_or_default(),
        );

        // Try image — if it fails, that's fine (not an image or empty)
        if let Ok(image_base64) = self.get_clipboard_image() {
            self.input_tags
                .insert("system_clipboard_image".into(), image_base64);
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

    /// Get clipboard text with validation.
    pub fn get_clipboard_text(&self) -> Result<String> {
        // 1. Get text
        let mut clipboard = arboard::Clipboard::new()
            .map_err(|e| anyhow::anyhow!("Failed to access clipboard: {}", e))?;
        let text = clipboard
            .get_text()
            .map_err(|_| anyhow::anyhow!("Clipboard is empty or contains non-text data."))?;

        // 2. Check size against cluster limits
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

    /// Get images from flow input tags (for vision actions).
    pub fn get_images(&self) -> Option<Vec<String>> {
        let images: Vec<String> = self
            .args
            .iter()
            .filter(|a| a.expect == ArgExpect::Image)
            .filter_map(|a| self.input_tags.get(&a.name))
            .cloned()
            .collect();
        if images.is_empty() {
            None
        } else {
            Some(images)
        }
    }
}
