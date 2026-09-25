//! AppConfig validation.
//! See [`crate::validate`] module-level docs for validation rules.

use anyhow::Result;

use crate::configs::app::AppConfig;
use crate::utils::constants;
use crate::validate::ValidateTrait;

impl ValidateTrait for AppConfig {
    /// Validate configuration.
    fn validate(&self) -> Result<()> {
        // Check version.
        if self.version != constants::CONFIG_VERSION {
            anyhow::bail!(
                "Config version mismatch: '{}', expected '{}'. Please update your config.",
                self.version,
                constants::CONFIG_VERSION
            );
        }
        // Validate each cluster node.
        for node in &self.cluster {
            node.validate()?;
        }
        Ok(())
    }
}
