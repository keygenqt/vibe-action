//! Query provider for `{query|project_path}` — project root path.

use crate::query::query::QueryKey;
use crate::query::query::QueryProvider;
use crate::utils;
use anyhow::Result;
use std::path::Path;
use std::path::PathBuf;

pub struct ProjectPathProvider {
    raw_value: Option<String>,
}

impl ProjectPathProvider {
    pub fn new(raw_value: Option<String>) -> Self {
        Self { raw_value }
    }

    /// Find the project root directory starting from a file's path, walking upwards.
    /// Stops if it reaches the home directory or the filesystem root.
    fn find_project_root(start: &Path) -> Option<PathBuf> {
        let home = dirs::home_dir();
        let mut current = start.parent();

        while let Some(dir) = current {
            // Check for common project root markers
            let has_marker = [
                ".git",
                ".hg",
                "Cargo.toml",
                "package.json",
                "go.mod",
                "pom.xml",
                "build.gradle",
                ".idea",
            ]
            .iter()
            .any(|marker| dir.join(marker).exists());

            if has_marker {
                return Some(dir.to_path_buf());
            }

            // Stop if we reached the home directory or root to avoid scanning the whole disk
            if let Some(ref home) = home {
                if dir == home.as_path() {
                    return None;
                }
            }

            current = dir.parent();
        }
        None
    }
}

impl QueryProvider for ProjectPathProvider {
    fn key(&self) -> QueryKey {
        QueryKey::ProjectPath
    }

    fn resolve(&self) -> Result<String> {
        let start_path =
            if let Some(raw) = self.raw_value.as_deref().filter(|v| !v.trim().is_empty()) {
                let parsed = utils::clipboard::parse_uri_list(raw);
                if let Some(first) = parsed.first() {
                    utils::path::resolve(first)?
                } else {
                    utils::path::resolve(Path::new(raw.trim()))?
                }
            } else {
                // Fallback to current dir if no input
                std::env::current_dir()?
            };

        // If directory, return as is. If file, search for project root.
        let project_root = if start_path.is_dir() {
            start_path
        } else {
            Self::find_project_root(&start_path).unwrap_or_default()
        };

        Ok(project_root.to_string_lossy().into_owned())
    }
}
