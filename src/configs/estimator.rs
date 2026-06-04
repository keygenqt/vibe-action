//! Complexity estimator configuration.
//! Separate from cluster — one estimator for the whole pipeline.

use anyhow::Result;
use serde::Deserialize;
use serde::Serialize;
use url::Url;

/// Configuration for the complexity estimator model.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EstimatorConfig {
    /// Provider type: "ollama", "deepseek", "qwen".
    pub provider: String,
    /// API endpoint URL.
    pub host: String,
    /// Model name to use for complexity estimation.
    pub model: String,
    /// Number of parallel connections to this provider (default 1).
    pub parallel: usize,
}

impl Default for EstimatorConfig {
    /// Creates default estimator configuration.
    fn default() -> Self {
        Self {
            provider: "ollama".to_string(),
            host: "http://localhost:11434".to_string(),
            model: "qwen2.5-coder:14b-instruct".to_string(),
            parallel: 1,
        }
    }
}

use std::borrow::Cow;

impl EstimatorConfig {
    /// Validate estimator configuration.
    pub fn validate(&self) -> Result<()> {
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
