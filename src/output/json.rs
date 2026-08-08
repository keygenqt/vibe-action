//! JSON output — all messages as JSON objects.

use crate::output::format::FormatOutput;
use crate::output::msg::OutputMsg;
use crate::output::output::OutputType;

use super::output::Output;

pub struct JsonOutput {
    formatter: FormatOutput,
}

impl JsonOutput {
    /// Creates a new JSON output strategy with an injected formatter.
    pub fn new(formatter: FormatOutput) -> Self {
        Self { formatter }
    }
}

impl Output for JsonOutput {
    /// Returns the output type.
    fn output_type(&self) -> OutputType {
        OutputType::Json
    }

    /// Prints plain message.
    fn plain(&self, msg: &OutputMsg) {
        println!("{}", self.formatter.format(msg));
    }

    /// Prints error as JSON.
    fn error(&self, msg: &OutputMsg) {
        println!("{}", self.formatter.format(msg));
    }

    /// Prints warning as JSON.
    fn warning(&self, msg: &OutputMsg) {
        println!("{}", self.formatter.format(msg));
    }

    /// Prints info as JSON.
    fn info(&self, msg: &OutputMsg) {
        println!("{}", self.formatter.format(msg));
    }

    /// Prints success as JSON.
    fn success(&self, msg: &OutputMsg) {
        println!("{}", self.formatter.format(msg));
    }

    /// Formats and prints debug messages as clean JSON to stdout.
    fn debug(&self, msg: &OutputMsg) {
        println!("{}", self.formatter.format(msg));
    }

    /// Formats and prints trace messages as clean JSON to stdout.
    fn trace(&self, msg: &OutputMsg) {
        println!("{}", self.formatter.format(msg));
    }

    /// Formats and prints progress indicators as clean JSON to stdout.
    fn progress(&self, msg: &OutputMsg) {
        println!("{}", self.formatter.format(msg));
    }
}
