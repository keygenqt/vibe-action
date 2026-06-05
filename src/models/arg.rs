use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::models::action::ExpectMode;

/// CLI argument definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionArg {
    /// Argument name (used as --name and {name} tag).
    pub name: String,
    /// Short flag (e.g. -p).
    #[serde(default)]
    pub short: Option<String>,
    /// Expected type.
    pub expect: ExpectMode,
    /// Help text.
    #[serde(default)]
    pub help: Option<String>,
}

impl ActionArg {
    /// Validate argument fields for clap compatibility.
    pub fn validate(&self) -> Result<()> {
        // Name must not be empty.
        if self.name.trim().is_empty() {
            anyhow::bail!("Argument name must not be empty.");
        }
        // Name must be alphanumeric with underscores.
        if !self.name.chars().all(|c| c.is_alphanumeric() || c == '_') {
            anyhow::bail!(
                "Argument name '{}' must only contain letters, numbers, or underscores.",
                self.name
            );
        }
        // Short must be a single letter if present.
        if let Some(short) = &self.short {
            if short.len() != 1 || !short.chars().all(|c| c.is_alphabetic()) {
                anyhow::bail!("Short flag must be a single letter, got '{}'.", short);
            }
        }
        // Validate expect type is clap-compatible.
        match &self.expect {
            ExpectMode::Void | ExpectMode::Json | ExpectMode::List(_) => {
                anyhow::bail!(
                    "Argument '{}' has unsupported type '{}' for CLI. Use string, number, or bool.",
                    self.name,
                    self.expect
                );
            }
            _ => {}
        }
        Ok(())
    }
}

impl std::fmt::Display for ExpectMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExpectMode::Void => write!(f, "void"),
            ExpectMode::Bool => write!(f, "bool"),
            ExpectMode::Number => write!(f, "number"),
            ExpectMode::String => write!(f, "string"),
            ExpectMode::Json => write!(f, "json"),
            ExpectMode::List(inner) => write!(f, "list<{}>", inner),
        }
    }
}
