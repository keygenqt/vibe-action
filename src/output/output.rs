//! Output trait, registry, and level enum.

/// Output level key.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OutputLevel {
    Cli,
    Tracing,
    Plain,
    Json,
}

/// Output trait.
pub trait Output: Send + Sync {
    fn level(&self) -> OutputLevel;
    fn error(&self, msg: &str);
    fn warning(&self, msg: &str);
    fn info(&self, msg: &str);
    fn success(&self, msg: &str);
    fn debug(&self, msg: &str);
    fn trace(&self, msg: &str);
    fn progress(&self, msg: &str);
}

/// Registry of outputs with a single active output.
pub struct OutputRegistry {
    current: Box<dyn Output>,
}

impl OutputRegistry {
    /// Create registry based on VIBE_LOG_TYPE and VIBE_TRACE_LEVEL.
    pub fn new(log_type: &str, trace_level: &str) -> Self {
        let current: Box<dyn Output> = match log_type {
            "cli" => Box::new(super::cli::CliOutput),
            "tracing" => Box::new(super::tracing::TracingOutput::new(trace_level)),
            "plain" => Box::new(super::plain::PlainOutput),
            "json" => Box::new(super::json::JsonOutput),
            _ => Box::new(super::cli::CliOutput),
        };
        Self { current }
    }

    pub fn level(&self) -> OutputLevel {
        self.current.level()
    }
    pub fn error(&self, msg: &str) {
        self.current.error(msg);
    }
    pub fn warning(&self, msg: &str) {
        self.current.warning(msg);
    }
    pub fn info(&self, msg: &str) {
        self.current.info(msg);
    }
    pub fn success(&self, msg: &str) {
        self.current.success(msg);
    }
    pub fn debug(&self, msg: &str) {
        self.current.debug(msg);
    }
    pub fn trace(&self, msg: &str) {
        self.current.trace(msg);
    }
    pub fn progress(&self, msg: &str) {
        self.current.progress(msg);
    }
}
