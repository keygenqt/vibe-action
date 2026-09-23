//! Test output strategy — only error and success visible.
//! See [`crate::output`] module-level docs for the architecture overview.

use crate::output::format::FormatOutput;
use crate::output::msg::OutputMsg;
use crate::output::output::OutputType;

use super::output::Output;

pub struct TestOutput {
    formatter: FormatOutput,
}

impl TestOutput {
    /// Creates a new test text output strategy with an injected formatter.
    pub fn new(formatter: FormatOutput) -> Self {
        Self { formatter }
    }
}

impl Output for TestOutput {
    /// Returns the output type.
    fn output_type(&self) -> OutputType {
        OutputType::Test
    }

    /// Formats and prints error message to stderr.
    fn error(&self, msg: &OutputMsg) {
        eprintln!("{}", self.formatter.format(msg));
    }

    /// Formats and prints successful result to stdout.
    fn success(&self, msg: &OutputMsg) {
        println!("{}", self.formatter.format(msg));
    }

    /// Ignored in Test mode.
    fn plain(&self, _msg: &OutputMsg) {}
    fn warning(&self, _msg: &OutputMsg) {}
    fn info(&self, _msg: &OutputMsg) {}
    fn debug(&self, _msg: &OutputMsg) {}
    fn trace(&self, _msg: &OutputMsg) {}
    fn progress(&self, _msg: &OutputMsg) {}
}
