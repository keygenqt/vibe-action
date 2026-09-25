//! Query key enum, provider trait, and registry.
//! See [`crate::query`] module-level docs for the provider contract and tag list.

use crate::query::impls::clipboard::ClipboardProvider;
use crate::query::impls::clipboard_image::ClipboardImageProvider;
use crate::query::impls::clipboard_path::ClipboardPathProvider;
use crate::query::impls::clipboard_text::ClipboardTextProvider;
use crate::query::impls::file_path::FilePathProvider;
use crate::query::impls::image::ImageProvider;
use crate::query::impls::line::LineProvider;
use crate::query::impls::project_path::ProjectPathProvider;
use crate::query::impls::prompt::PromptProvider;
use crate::query::impls::raw::RawProvider;
use anyhow::Result;
use std::collections::HashMap;

/// Enum of all known query type keys.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum QueryKey {
    Clipboard,
    ClipboardText,
    ClipboardPath,
    ClipboardImage,
    FilePath,
    ProjectPath,
    Line,
    Prompt,
    Image,
    Raw,
}

impl QueryKey {
    pub fn as_str(&self) -> &'static str {
        match self {
            QueryKey::Clipboard => "query_clipboard",
            QueryKey::ClipboardText => "query_clipboard_text",
            QueryKey::ClipboardPath => "query_clipboard_path",
            QueryKey::ClipboardImage => "query_clipboard_image",
            QueryKey::FilePath => "query_file_path",
            QueryKey::ProjectPath => "query_project_path",
            QueryKey::Line => "query_line",
            QueryKey::Prompt => "query_prompt",
            QueryKey::Image => "query_image",
            QueryKey::Raw => "query_raw",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "query_clipboard" => Some(QueryKey::Clipboard),
            "query_clipboard_text" => Some(QueryKey::ClipboardText),
            "query_clipboard_path" => Some(QueryKey::ClipboardPath),
            "query_clipboard_image" => Some(QueryKey::ClipboardImage),
            "query_file_path" => Some(QueryKey::FilePath),
            "query_project_path" => Some(QueryKey::ProjectPath),
            "query_line" => Some(QueryKey::Line),
            "query_prompt" => Some(QueryKey::Prompt),
            "query_image" => Some(QueryKey::Image),
            "query_raw" => Some(QueryKey::Raw),
            _ => None,
        }
    }

    /// All known query keys.
    pub fn all() -> &'static [QueryKey] {
        &[
            QueryKey::Clipboard,
            QueryKey::ClipboardText,
            QueryKey::ClipboardPath,
            QueryKey::ClipboardImage,
            QueryKey::FilePath,
            QueryKey::ProjectPath,
            QueryKey::Line,
            QueryKey::Prompt,
            QueryKey::Image,
            QueryKey::Raw,
        ]
    }
}

/// Trait for query tag providers.
pub trait QueryProvider: Send + Sync {
    fn key(&self) -> QueryKey;
    fn resolve(&self) -> Result<String>;
}

/// Registry of all query providers.
pub struct QueryRegistry {
    providers: HashMap<QueryKey, Box<dyn QueryProvider>>,
}

impl QueryRegistry {
    /// Create a new registry with all built-in providers.
    pub fn new(raw_value: Option<String>) -> Self {
        let mut registry = Self {
            providers: HashMap::new(),
        };
        registry.register(Box::new(ClipboardProvider::new()));
        registry.register(Box::new(ClipboardTextProvider::new()));
        registry.register(Box::new(ClipboardPathProvider::new()));
        registry.register(Box::new(ClipboardImageProvider::new()));
        registry.register(Box::new(FilePathProvider::new(raw_value.clone())));
        registry.register(Box::new(ProjectPathProvider::new(raw_value.clone())));
        registry.register(Box::new(LineProvider::new(raw_value.clone())));
        registry.register(Box::new(PromptProvider::new(raw_value.clone())));
        registry.register(Box::new(ImageProvider::new(raw_value.clone())));
        registry.register(Box::new(RawProvider::new(raw_value.clone())));
        registry
    }

    /// Register a new provider.
    pub fn register(&mut self, provider: Box<dyn QueryProvider>) {
        self.providers.insert(provider.key(), provider);
    }

    /// Resolve a query tag by its key.
    pub fn resolve(&self, key: QueryKey) -> Result<String> {
        match self.providers.get(&key) {
            Some(provider) => provider.resolve(),
            None => anyhow::bail!("Unknown query type: '{}'", key.as_str()),
        }
    }
}
