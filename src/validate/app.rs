//! AppConfig validation.
//! Checks estimator and cluster configuration.

use anyhow::Result;

use crate::{configs::app::AppConfig, validate::ValidateTrait};

impl ValidateTrait for AppConfig {
    /// Validate configuration.
    fn validate(&self) -> Result<()> {
        // Validate each cluster node.
        for node in &self.cluster {
            node.validate()?;
        }
        Ok(())
    }
}
