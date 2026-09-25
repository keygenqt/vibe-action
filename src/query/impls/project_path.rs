//! Query provider for `query_project_path` — project root path.

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

    /// Find the project root directory starting from a path, walking upwards.
    /// Checks the start directory itself, then each ancestor.
    /// Stops at the home directory or filesystem root.
    fn find_project_root(start: &Path) -> Option<PathBuf> {
        let home = dirs::home_dir();
        // If `start` is a file, begin at its parent; otherwise start at the dir itself.
        let mut current = if start.is_dir() {
            Some(start)
        } else {
            start.parent()
        };

        while let Some(dir) = current {
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

            // Stop at home dir or root to avoid scanning the whole disk.
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
        // Empty input → no data → empty string (consistent with other validators).
        let raw = match self
            .raw_value
            .as_deref()
            .map(str::trim)
            .filter(|v| !v.is_empty())
        {
            Some(v) => v,
            None => return Ok(String::new()),
        };

        // Resolve to an existing path; fall through to "" on non-existent input.
        let parsed = utils::clipboard::parse_uri_list(raw);
        let start_path = match parsed
            .first()
            .map(|p| utils::path::resolve(p))
            .unwrap_or_else(|| utils::path::resolve(Path::new(raw.trim())))
            .ok()
            .filter(|p| p.exists())
        {
            Some(p) => p,
            None => return Ok(String::new()),
        };

        // Walk up to the project root from the resolved path.
        let project_root = Self::find_project_root(&start_path).unwrap_or_default();

        Ok(project_root.to_string_lossy().into_owned())
    }
}
