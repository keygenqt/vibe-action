//! Global action runtime settings: system prompt and retry policy.
//! These apply to all flows by default.

use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionConfig {
    /// Default system prompt for all LLM requests.
    pub system: String,
    /// Number of retries for failed LLM steps (default: 0).
    pub retries: u32,
}

impl Default for ActionConfig {
    /// Creates default action configuration.
    fn default() -> Self {
        Self {
            system: r#"
You are Vibe Action — a CLI tool, not a chatbot.
Work fast. Just do the task.
Reasoning models: keep chain of thought extremely brief.
Output ONLY the requested result.
No explanations, markdown fences, or extra text unless explicitly requested.
            "#
            .trim()
            .into(),
            retries: 2,
        }
    }
}
