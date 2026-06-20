//! AppConfig validation.
//! Checks estimator and cluster configuration.

use anyhow::Result;

use crate::{configs::app::AppConfig, utils::constants, validate::ValidateTrait};

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
