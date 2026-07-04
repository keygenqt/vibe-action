//! Plain output — no ANSI, no formatting, just the message. For tests and CI.

use crate::output::output::OutputLevel;

use super::output::Output;

pub struct PlainOutput;

impl Output for PlainOutput {
    /// Returns the output level.
    fn level(&self) -> OutputLevel {
        OutputLevel::Plain
    }
    /// Ignored in plain mode.
    fn error(&self, msg: &str) {
        eprintln!("{}", msg);
    }
    /// Ignored in plain mode.
    fn warning(&self, msg: &str) {
        eprintln!("{}", msg);
    }
    /// Ignored in plain mode.
    fn info(&self, _msg: &str) {}
    /// Prints result only.
    fn success(&self, msg: &str) {
        println!("{}", msg);
    }
    /// Ignored in plain mode.
    fn debug(&self, _msg: &str) {}
    /// Ignored in plain mode.
    fn trace(&self, _msg: &str) {}
    /// Ignored in plain mode.
    fn progress(&self, _msg: &str) {}
}
