//! Query provider trait and registry.
//! Each provider resolves a `{query}` or `{query|type}` tag to a runtime value.

use crate::configs::app::AppConfig;
use crate::query::impls::clipboard::ClipboardProvider;
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
            QueryKey::Raw,
            QueryKey::FilePath,
            QueryKey::ProjectPath,
            QueryKey::Line,
            QueryKey::Prompt,
            QueryKey::Image,
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
