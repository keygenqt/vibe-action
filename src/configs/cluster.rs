//! LLM provider configuration for model inference.
//! Defines connection parameters and model settings for code analysis.

use anyhow::Result;
use serde::Deserialize;
use serde::Serialize;
use url::Url;

/// Configuration for a single LLM provider connection.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClusterConfig {
    /// Provider type: "ollama", "deepseek", "qwen".
    pub provider: String,
    /// API endpoint URL.
    pub host: String,
    /// Model name to use for inference.
    pub model: String,
    /// Request timeout in seconds.
    pub timeout_secs: u64,
    /// Lower bound of complexity range (0.0-1.0), inclusive.
    pub complexity_from: f32,
    /// Upper bound of complexity range (0.0-1.0), inclusive.
    pub complexity_to: f32,
    /// Temperature for generation (0.0-2.0, lower = more deterministic).
    pub temperature: f32,
    /// Seed for reproducible outputs.
    pub seed: u64,
    /// Context window size in tokens.
    pub num_ctx: usize,
    /// Maximum tokens to predict/generate.
    pub num_predict: usize,
    /// API key for cloud providers (not needed for Ollama).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key: Option<String>,
    /// Number of parallel connections to this provider (default 1).
    pub parallel: usize,
}

impl Default for ClusterConfig {
    /// Creates default Ollama configuration.
    fn default() -> Self {
        Self {
            provider: "ollama".to_string(),
            host: "http://localhost:11434".to_string(),
            model: "qwen2.5-coder:3b-instruct".to_string(),
            complexity_from: 0.0,
            complexity_to: 1.0,
            timeout_secs: 60,
            temperature: 0.1,
            seed: 42,
            num_ctx: 4096,
            num_predict: 2048,
            api_key: None,
            parallel: 1,
        }
    }
}

impl ClusterConfig {
    /// Validate cluster node configuration.
    pub fn validate(&self) -> Result<()> {
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
