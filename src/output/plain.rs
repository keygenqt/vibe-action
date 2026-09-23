//! Plain output strategy — no ANSI, no formatting.
//! See [`crate::output`] module-level docs for the architecture overview.

use crate::output::format::FormatOutput;
use crate::output::msg::OutputMsg;
use crate::output::output::OutputType;

use super::output::Output;

pub struct PlainOutput {
    formatter: FormatOutput,
}

impl PlainOutput {
    /// Creates a new plain text output strategy with an injected formatter.
    pub fn new(formatter: FormatOutput) -> Self {
        Self { formatter }
    }
}

impl Output for PlainOutput {
    /// Returns the output type.
    fn output_type(&self) -> OutputType {
        OutputType::Plain
    }

    /// Prints plain message.
    fn plain(&self, msg: &OutputMsg) {
        println!("{}", self.formatter.format(msg));
    }

    /// Formats and prints error message to stderr.
    fn error(&self, msg: &OutputMsg) {
        eprintln!("{}", self.formatter.format(msg));
    }

    /// Formats and prints warning message to stderr.
    fn warning(&self, msg: &OutputMsg) {
        eprintln!("{}", self.formatter.format(msg));
    }

    /// Ignored in plain mode.
    fn info(&self, msg: &OutputMsg) {
        println!("{}", self.formatter.format(msg));
    }

    /// Formats and prints successful result to stdout.
    fn success(&self, msg: &OutputMsg) {
        println!("{}", self.formatter.format(msg));
    }

    /// Ignored in plain mode.
    fn debug(&self, msg: &OutputMsg) {
        println!("{}", self.formatter.format(msg));
    }

    /// Ignored in plain mode.
    fn trace(&self, msg: &OutputMsg) {
        println!("{}", self.formatter.format(msg));
    }

    /// Ignored in plain mode.
    fn progress(&self, msg: &OutputMsg) {
        println!("{}", self.formatter.format(msg));
    }
}
