//! Tracing output — structured logs via tracing crate.

use crate::output::output::OutputLevel;

use super::output::Output;

pub struct TracingOutput;

impl TracingOutput {
    pub fn new(level: &str) -> Self {
        let filter = format!("vibe_action={}", level);
        let _ = tracing_subscriber::fmt().with_env_filter(filter).try_init();
        Self
    }
}

impl Output for TracingOutput {
    /// Returns the output level.
    fn level(&self) -> OutputLevel {
        OutputLevel::Tracing
    }

    /// Maps to tracing::error!
    fn error(&self, msg: &str) {
        tracing::error!("{}", msg);
    }

    /// Maps to tracing::warn!
    fn warning(&self, msg: &str) {
        tracing::warn!("{}", msg);
    }

    /// Maps to tracing::info!
    fn info(&self, msg: &str) {
        tracing::info!("{}", msg);
    }

    /// Maps to tracing::info!
    fn success(&self, msg: &str) {
        tracing::info!("{}", msg);
    }

    /// Maps to tracing::debug!
    fn debug(&self, msg: &str) {
        tracing::debug!("{}", msg);
    }

    /// Maps to tracing::trace!
    fn trace(&self, msg: &str) {
        tracing::trace!("{}", msg);
    }

    /// Ignored in tracing mode.
    fn progress(&self, _msg: &str) {}
}
