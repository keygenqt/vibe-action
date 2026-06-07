//! ClusterConfig validation.
//! Checks provider, host, model, and numeric ranges.

use anyhow::Result;
use url::Url;

use crate::{configs::cluster::ClusterConfig, validate::ValidateTrait};

impl ValidateTrait for ClusterConfig {
    /// Validate cluster node configuration.
    fn validate(&self) -> Result<()> {
        let is_empty_or_whitespace = |s: &str| s.is_empty() || s.chars().all(|c| c.is_whitespace());
        // Required fields must not be empty.
        if is_empty_or_whitespace(&self.provider) {
            anyhow::bail!("Cluster node: provider is empty.");
        }
        if is_empty_or_whitespace(&self.model) {
            anyhow::bail!("Cluster node: model is empty.");
        }
        // Host must be a valid URL.
        if is_empty_or_whitespace(&self.host) {
            anyhow::bail!("Cluster node: host is empty.");
        }
        Url::parse(&self.host).map_err(|e| {
            anyhow::anyhow!("Cluster node: invalid host URL '{}': {}", self.host, e)
        })?;
        // Complexity range must be valid (NaN-safe).
        if self.complexity_from.is_nan() || !(0.0..=1.0).contains(&self.complexity_from) {
            anyhow::bail!(
                "Cluster node: complexity_from must be 0.0-1.0, got {}",
                self.complexity_from
            );
        }
        if self.complexity_to.is_nan() || !(0.0..=1.0).contains(&self.complexity_to) {
            anyhow::bail!(
                "Cluster node: complexity_to must be 0.0-1.0, got {}",
                self.complexity_to
            );
        }
        if self.complexity_from > self.complexity_to {
            anyhow::bail!(
                "Cluster node: complexity_from ({}) must be <= complexity_to ({})",
                self.complexity_from,
                self.complexity_to
            );
        }
        // Temperature must be valid (NaN-safe).
        if self.temperature.is_nan() || !(0.0..=2.0).contains(&self.temperature) {
            anyhow::bail!(
                "Cluster node: temperature must be 0.0-2.0, got {}",
                self.temperature
            );
        }
        // Numeric limits must be > 0.
        if self.num_ctx == 0 {
            anyhow::bail!("Cluster node: num_ctx must be > 0.");
        }
        if self.num_predict == 0 {
            anyhow::bail!("Cluster node: num_predict must be > 0.");
        }
        if self.parallel == 0 {
            anyhow::bail!("Cluster node: parallel must be > 0.");
        }
        Ok(())
    }
}
