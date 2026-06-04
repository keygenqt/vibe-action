//! Action source configuration.
//! Represents a source of action files (directory, file, remote).

use anyhow::Result;
use std::{
    ops::Deref,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

use crate::utils::path;

/// Action source (internal representation).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionSourceConfig(String);

impl ActionSourceConfig {
    /// Resolve to absolute path.
    pub fn resolve(&self) -> Result<PathBuf> {
        path::resolve(&self.0)
    }

    /// Check if path is an existing YAML file.
    pub fn is_file(&self) -> bool {
        if let Ok(resolved) = self.resolve() {
            resolved.is_file() && (self.0.ends_with(".yaml") || self.0.ends_with(".yml"))
        } else {
            false
        }
    }

    /// Check if path is an existing directory.
    pub fn is_dir(&self) -> bool {
        if let Ok(resolved) = self.resolve() {
            resolved.is_dir()
        } else {
            false
        }
    }

    /// Validate action source exists.
    pub fn validate(&self) -> Result<()> {
        if !self.is_dir() && !self.is_file() {
            anyhow::bail!(
                "Action source must be an existing directory or .yaml file: {}",
                self
            );
        }
        Ok(())
    }
}

impl From<&str> for ActionSourceConfig {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

impl From<String> for ActionSourceConfig {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl AsRef<Path> for ActionSourceConfig {
    fn as_ref(&self) -> &Path {
        Path::new(&self.0)
    }
}

impl Deref for ActionSourceConfig {
    type Target = str;
    fn deref(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for ActionSourceConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
