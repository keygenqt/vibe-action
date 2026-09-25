//! Tracing output strategy via the tracing crate.
//! See [`crate::output`] module-level docs for the architecture overview.

use crate::output::format::FormatOutput;
use crate::output::msg::OutputMsg;
use crate::output::output::OutputType;

use super::output::Output;

pub struct TracingOutput {
    formatter: FormatOutput,
}

impl TracingOutput {
    /// Initializes the global tracing subscriber and stores the formatter.
    pub fn new(level: &str, formatter: FormatOutput) -> Self {
        let filter = format!("vibe_action={}", level);
        let _ = tracing_subscriber::fmt().with_env_filter(filter).try_init();
        Self { formatter }
    }
}

impl Output for TracingOutput {
    /// Returns the output type.
    fn output_type(&self) -> OutputType {
        OutputType::Tracing
    }

    /// Prints plain message.
    fn plain(&self, msg: &OutputMsg) {
        tracing::info!("{}", self.formatter.format(msg));
    }

    /// Maps to tracing::error!
    fn error(&self, msg: &OutputMsg) {
        tracing::error!("{}", self.formatter.format(msg));
    }

    /// Maps to tracing::warn!
    fn warning(&self, msg: &OutputMsg) {
        tracing::warn!("{}", self.formatter.format(msg));
    }

    /// Maps to tracing::info!
    fn info(&self, msg: &OutputMsg) {
        tracing::info!("{}", self.formatter.format(msg));
    }

    /// Maps to tracing::info!
    fn success(&self, msg: &OutputMsg) {
        tracing::info!("{}", self.formatter.format(msg));
    }

    /// Maps to tracing::debug!
    fn debug(&self, msg: &OutputMsg) {
        tracing::debug!("{}", self.formatter.format(msg));
    }

    /// Maps to tracing::trace!
    fn trace(&self, msg: &OutputMsg) {
        tracing::trace!("{}", self.formatter.format(msg));
    }

    /// Maps progress to tracing::trace! for granular pipeline diagnostics.
    fn progress(&self, msg: &OutputMsg) {
        tracing::trace!("{}", self.formatter.format(msg));
    }
}
