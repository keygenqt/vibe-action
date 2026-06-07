//! CLI argument value resolution and validation.
//! Converts raw CLI strings into validated, resolved values.

use anyhow::Result;

use crate::utils;

/// Validated argument value from CLI input.
#[derive(Debug, Clone)]
pub struct ArgActionValue {
    /// Argument name (matches ArgActionModel.name).
    pub name: String,
    /// Resolved and validated value.
    pub value: String,
}

impl ArgActionValue {
    /// Create a value with path resolution and future type checks.
    pub fn new_with_check(name: &str, value: &str) -> Self {
        // Check is path
        if let Ok(path) = Self::resolve_path(value) {
            return Self {
                name: name.to_string(),
                value: path,
            };
        }

        // Other checks
        // ...

        // Default — raw value
        Self::new(name, value)
    }

    /// Create a value without any resolution (raw string).
    fn new(name: &str, value: &str) -> Self {
        Self {
            name: name.to_string(),
            value: value.to_string(),
        }
    }

    /// Resolve value to absolute path. Returns Ok if path resolved, Err if not a path.
    fn resolve_path(val: &str) -> Result<String> {
        if val.starts_with('/') || val.starts_with('~') || val.starts_with('.') {
            utils::path::resolve(val)
                .map(|p| p.display().to_string())
                .map_err(|e| anyhow::anyhow!("Failed to resolve path '{}': {}", val, e))
        } else {
            anyhow::bail!("Not a path: '{}'", val)
        }
    }
}
