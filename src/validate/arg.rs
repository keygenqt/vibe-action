//! ArgActionModel validation.
//! Checks name, short flag, and CLI-compatible types.

use anyhow::Result;

use crate::{
    models::{action::ExpectMode, arg::ArgActionModel},
    validate::ValidateTrait,
};

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

        // Валидация short теперь тривиальна и безопасна
        if let Some(short) = self.short {
            if !short.is_ascii_alphabetic() {
                anyhow::bail!("Short flag must be an ASCII letter, got '{}'.", short);
            }
        }

        match &self.expect {
            ExpectMode::Void | ExpectMode::Json | ExpectMode::List(_) => {
                anyhow::bail!("Argument '{}' has unsupported type для CLI.", self.name);
            }
            _ => {}
        }
        Ok(())
    }
}
