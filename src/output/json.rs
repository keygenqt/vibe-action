//! JSON output — all messages as JSON objects.

use crate::output::output::OutputLevel;

use super::output::Output;
use serde::Serialize;

#[derive(Serialize)]
struct JsonMessage {
    level: String,
    message: String,
}

pub struct JsonOutput;

impl Output for JsonOutput {
    /// Returns the output level.
    fn level(&self) -> OutputLevel {
        OutputLevel::Json
    }

    /// Prints error as JSON.
    fn error(&self, msg: &str) {
        println!(
            "{}",
            serde_json::to_string(&JsonMessage {
                level: "error".into(),
                message: msg.to_string()
            })
            .unwrap()
        );
    }

    /// Prints warning as JSON.
    fn warning(&self, msg: &str) {
        println!(
            "{}",
            serde_json::to_string(&JsonMessage {
                level: "warning".into(),
                message: msg.to_string()
            })
            .unwrap()
        );
    }

    /// Prints info as JSON.
    fn info(&self, msg: &str) {
        println!(
            "{}",
            serde_json::to_string(&JsonMessage {
                level: "info".into(),
                message: msg.to_string()
            })
            .unwrap()
        );
    }

    /// Prints success as JSON.
    fn success(&self, msg: &str) {
        println!(
            "{}",
            serde_json::to_string(&JsonMessage {
                level: "success".into(),
                message: msg.to_string()
            })
            .unwrap()
        );
    }

    /// Ignored in JSON mode.
    fn debug(&self, _msg: &str) {}

    /// Ignored in JSON mode.
    fn trace(&self, _msg: &str) {}

    /// Ignored in JSON mode.
    fn progress(&self, _msg: &str) {}
}
