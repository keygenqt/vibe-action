//! Query provider trait and registry.
//! Each provider resolves a `{query}` or `{query|type}` tag to a runtime value.

use crate::configs::app::AppConfig;
use crate::models::context::ContextModel;
use crate::query::file_path::FilePathProvider;
use crate::query::image::ImageProvider;
use crate::query::line::LineProvider;
use crate::query::project_path::ProjectPathProvider;
use crate::query::prompt::PromptProvider;
use crate::query::query_raw::QueryRawProvider;
use anyhow::Result;
use std::collections::HashMap;

/// Enum of all known query type keys.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum QueryKey {
    Raw,
    FilePath,
    ProjectPath,
    Line,
    Prompt,
    Image,
}

impl QueryKey {
    pub fn as_str(&self) -> &'static str {
        match self {
            QueryKey::Raw => "raw",
            QueryKey::FilePath => "file_path",
            QueryKey::ProjectPath => "project_path",
            QueryKey::Line => "line",
            QueryKey::Prompt => "prompt",
            QueryKey::Image => "image",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "raw" => Some(QueryKey::Raw),
            "file_path" => Some(QueryKey::FilePath),
            "project_path" => Some(QueryKey::ProjectPath),
            "line" => Some(QueryKey::Line),
            "prompt" => Some(QueryKey::Prompt),
            "image" => Some(QueryKey::Image),
            _ => None,
        }
    }
}

/// Trait for query tag providers.
pub trait QueryProvider: Send + Sync {
    fn key(&self) -> QueryKey;
    fn resolve(&self) -> Result<ContextModel>;
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
        registry.register(Box::new(QueryRawProvider::new(raw_value.clone())));
        registry.register(Box::new(FilePathProvider::new(raw_value.clone())));
        registry.register(Box::new(ProjectPathProvider::new(raw_value.clone())));
        registry.register(Box::new(LineProvider::new(raw_value.clone())));
        registry.register(Box::new(PromptProvider::new(raw_value.clone())));
        registry.register(Box::new(ImageProvider::new(raw_value)));
        registry
    }

    /// Register a new provider.
    pub fn register(&mut self, provider: Box<dyn QueryProvider>) {
        self.providers.insert(provider.key(), provider);
    }

    /// Resolve a query tag by its key.
    pub fn resolve(&self, key: QueryKey) -> Result<ContextModel> {
        match self.providers.get(&key) {
            Some(provider) => provider.resolve(),
            None => anyhow::bail!("Unknown query type: '{}'", key.as_str()),
        }
    }
}

/// Reads text from the clipboard and validates its size against the max context size.
/// Falls back to file paths when the clipboard holds copied files.
pub fn read_clipboard_text() -> Result<String> {
    let raw = crate::utils::clipboard::read_text().unwrap_or_default();

    let text = if raw.is_empty() {
        let files = crate::utils::clipboard::clipboard_file_paths();
        if files.is_empty() {
            anyhow::bail!("Clipboard is empty or contains non-text data.");
        }
        files
            .iter()
            .map(|p| p.to_string_lossy().into_owned())
            .collect::<Vec<_>>()
            .join("\n")
    } else {
        // Finder may give us "file:///..." as plain text — decode if real files
        let decoded = crate::utils::clipboard::parse_uri_list(&raw);
        if !decoded.is_empty() && decoded.iter().any(|p| p.exists()) {
            decoded
                .iter()
                .map(|p| p.to_string_lossy().into_owned())
                .collect::<Vec<_>>()
                .join("\n")
        } else {
            raw
        }
    };

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
