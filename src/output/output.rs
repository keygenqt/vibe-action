//! Output trait, enums, and registry.
//! See [`crate::output`] module-level docs for the architecture and output type descriptions.

use serde::Deserialize;
use serde::Serialize;

use crate::output::cli::CliOutput;
use crate::output::json::JsonOutput;
use crate::output::msg::OutputMsg;
use crate::output::plain::PlainOutput;
use crate::output::test::TestOutput;
use crate::output::tracing::TracingOutput;

/// Output type key.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OutputType {
    Cli,
    Tracing,
    Plain,
    Json,
    Test,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum OutputKind {
    Plain,
    Info,
    Success,
    Warning,
    Error,
    Debug,
    Trace,
    Progress,
}

impl OutputKind {
    /// Parses VIBE_TRACE_LEVEL (same values as the tracing EnvFilter).
    pub fn from_trace_level(s: &str) -> Self {
        match s {
            "error" => Self::Error,
            "warn" => Self::Warning,
            "info" => Self::Info,
            "debug" => Self::Debug,
            "trace" => Self::Trace,
            _ => Self::Info,
        }
    }
}

/// Export context for plugin integration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ExportContext {
    Actions,
    Groups,
    Status,
    Success,
    Confirm,
}

impl ExportContext {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Actions => "actions",
            Self::Groups => "groups",
            Self::Status => "status",
            Self::Success => "success",
            Self::Confirm => "confirm",
        }
    }
}

/// Output trait.
pub trait Output: Send + Sync {
    fn output_type(&self) -> OutputType;
    fn plain(&self, msg: &OutputMsg);
    fn error(&self, msg: &OutputMsg);
    fn warning(&self, msg: &OutputMsg);
    fn info(&self, msg: &OutputMsg);
    fn success(&self, msg: &OutputMsg);
    fn debug(&self, msg: &OutputMsg);
    fn trace(&self, msg: &OutputMsg);
    fn progress(&self, msg: &OutputMsg);
}

/// Registry of outputs with a single active output.
pub struct OutputRegistry {
    current: Box<dyn Output>,
    trace_level: OutputKind,
}

impl OutputRegistry {
    /// Create registry based on VIBE_LOG_TYPE and VIBE_TRACE_LEVEL.
    pub fn new(log_type: &str, trace_level: &str) -> Self {
        let output_type = if crate::utils::path::is_test() {
            OutputType::Test
        } else {
            match log_type {
                "tracing" => OutputType::Tracing,
                "plain" => OutputType::Plain,
                "json" => OutputType::Json,
                _ => OutputType::Cli,
            }
        };
        let formatter = super::format::FormatOutput::new(output_type);
        let current: Box<dyn Output> = match output_type {
            OutputType::Cli => Box::new(CliOutput::new(formatter)),
            OutputType::Tracing => Box::new(TracingOutput::new(trace_level, formatter)),
            OutputType::Plain => Box::new(PlainOutput::new(formatter)),
            OutputType::Json => Box::new(JsonOutput::new(formatter)),
            OutputType::Test => Box::new(TestOutput::new(formatter)),
        };

        Self {
            current,
            trace_level: OutputKind::from_trace_level(trace_level),
        }
    }

    /// Whether a message kind passes the configured trace level.
    fn level_allows(&self, kind: OutputKind) -> bool {
        match kind {
            OutputKind::Debug => {
                matches!(self.trace_level, OutputKind::Debug | OutputKind::Trace)
            }
            OutputKind::Trace => matches!(self.trace_level, OutputKind::Trace),
            _ => true,
        }
    }

    /// Universally dispatches any structural log message to its specific strategy method.
    /// Debug/Trace diagnostics are gated by VIBE_TRACE_LEVEL in every output mode.
    pub fn write(&self, msg: &OutputMsg) {
        if !self.level_allows(msg.kind) {
            return;
        }
        match msg.kind {
            OutputKind::Plain => self.current.plain(msg),
            OutputKind::Info => self.current.info(msg),
            OutputKind::Success => self.current.success(msg),
            OutputKind::Warning => self.current.warning(msg),
            OutputKind::Error => self.current.error(msg),
            OutputKind::Debug => self.current.debug(msg),
            OutputKind::Trace => self.current.trace(msg),
            OutputKind::Progress => self.current.progress(msg),
        }
    }

    pub fn output_type(&self) -> OutputType {
        self.current.output_type()
    }
}
