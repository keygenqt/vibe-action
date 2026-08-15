//! Query provider for `{query|file_path}` — valid file path from input or clipboard.

use super::query::QueryKey;
use super::query::QueryProvider;
use crate::models::context::ContextModel;
use crate::utils::clipboard;
use crate::utils::{self};
use anyhow::Result;
use std::path::PathBuf;

pub struct FilePathProvider {
    raw_value: Option<String>,
}

impl FilePathProvider {
    pub fn new(raw_value: Option<String>) -> Self {
        Self { raw_value }
    }

    /// First candidate that resolves to an existing file.
    fn first_existing_file(candidates: Vec<PathBuf>) -> Option<PathBuf> {
        candidates
            .into_iter()
            .find_map(|c| utils::path::resolve(&c).ok().filter(|p| p.is_file()))
    }
}

impl QueryProvider for FilePathProvider {
    fn key(&self) -> QueryKey {
        QueryKey::FilePath
    }

    fn resolve(&self) -> Result<ContextModel> {
        // Priority 1: explicit raw value (may be plain path or file:// URI)
        if let Some(raw) = self.raw_value.as_deref().filter(|v| !v.trim().is_empty()) {
            if let Some(path) = Self::first_existing_file(clipboard::parse_uri_list(raw)) {
                return Ok(ContextModel::String(path.to_string_lossy().into_owned()));
            }
            return Ok(ContextModel::String(String::new()));
        }

        // Priority 2: clipboard (file copy in FM, or pasted path/URI text)
        if let Some(path) = Self::first_existing_file(clipboard::clipboard_file_paths()) {
            return Ok(ContextModel::String(path.to_string_lossy().into_owned()));
        }

        Ok(ContextModel::String(String::new()))
    }
}
