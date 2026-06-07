//! EstimatorConfig validation.
//! Checks provider, host, model, and parallel settings.

use std::borrow::Cow;

use anyhow::Result;
use url::Url;

use crate::{configs::estimator::EstimatorConfig, validate::ValidateTrait};

impl ValidateTrait for EstimatorConfig {
    /// Validate estimator configuration.
    fn validate(&self) -> Result<()> {
        let is_empty_or_whitespace = |s: &str| s.is_empty() || s.chars().all(|c| c.is_whitespace());
        if is_empty_or_whitespace(&self.provider) {
            anyhow::bail!("Estimator: provider is empty.");
        }
        if is_empty_or_whitespace(&self.model) {
            anyhow::bail!("Estimator: model is empty.");
        }
        if is_empty_or_whitespace(&self.host) {
            anyhow::bail!("Estimator: host is empty.");
        }
        // Reject invalid URL schemes (e.g. "hhtp://").
        if self.host.contains("://")
            && !self.host.starts_with("http://")
            && !self.host.starts_with("https://")
        {
            anyhow::bail!(
                "Estimator: invalid URL scheme in host '{}'. Use 'http://' or 'https://'.",
                self.host
            );
        }
        // Normalize host: add http:// if no scheme present. Zero-allocation for valid URLs.
        let url_to_check: Cow<str> =
            if self.host.starts_with("http://") || self.host.starts_with("https://") {
                Cow::Borrowed(&self.host)
            } else {
                Cow::Owned(format!("http://{}", self.host))
            };
        Url::parse(&url_to_check)
            .map_err(|e| anyhow::anyhow!("Estimator: invalid host '{}': {}", self.host, e))?;
        if self.parallel == 0 {
            anyhow::bail!("Estimator: parallel must be > 0.");
        }

        Ok(())
    }
}
