//! LLM provider configuration for model inference.
//! Defines connection parameters and model settings for code analysis.

use serde::Deserialize;
use serde::Serialize;

/// Role of a cluster node by model size.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ClusterRole {
    /// Tiny model for trivial tasks (e.g., 1-3b).
    Tiny,
    /// Small model for simple tasks (e.g., 3-7b).
    Small,
    /// Medium model for general tasks (e.g., 7-14b).
    Medium,
    /// Large model for complex tasks (e.g., 14b+).
    Large,
    /// Vision model for image tasks.
    Vision,
}

/// Implements Display trait for ClusterRole to format as string.
impl std::fmt::Display for ClusterRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ClusterRole::Tiny => write!(f, "tiny"),
            ClusterRole::Small => write!(f, "small"),
            ClusterRole::Medium => write!(f, "medium"),
            ClusterRole::Large => write!(f, "large"),
            ClusterRole::Vision => write!(f, "vision"),
        }
    }
}

/// Configuration for a single LLM provider connection.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClusterConfig {
    /// Provider run: "ollama", "deepseek", "qwen".
    pub provider: String,
    /// API endpoint URL.
    pub host: String,
    /// Model name to use for inference.
    pub model: String,
    /// Role of this node: small, medium, or large.
    #[serde(default)]
    pub role: Option<ClusterRole>,
    /// Request timeout in seconds.
    pub timeout_secs: u64,
    /// Temperature for generation (0.0-2.0, lower = more deterministic).
    pub temperature: f64,
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
            role: Some(ClusterRole::Small),
            timeout_secs: 30,
            temperature: 0.1,
            seed: 42,
            num_ctx: 4096,
            num_predict: 2048,
            api_key: None,
            parallel: 1,
        }
    }
}
