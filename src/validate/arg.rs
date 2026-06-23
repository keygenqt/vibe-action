//! ArgActionModel validation.
//! Checks name, short flag, and CLI-compatible types.

use anyhow::Result;

use crate::{models::arg::ArgActionModel, validate::ValidateTrait};

impl ValidateTrait for ArgActionModel {
    /// Validate argument fields for clap compatibility.
    fn validate(&self) -> Result<()> {
        if self.name.trim().is_empty() {
            anyhow::bail!("Argument name must not be empty.");
        }
        if !self.name.chars().all(|c| c.is_alphanumeric() || c == '_') {
            anyhow::bail!(
                "Argument name '{}' must only contain letters, numbers, or underscores.",
                self.name
            );
        }
        if let Some(short) = self.short {
            if !short.is_ascii_alphabetic() {
                anyhow::bail!("Short flag must be an ASCII letter, got '{}'.", short);
            }
        }
        Ok(())
    }
}
