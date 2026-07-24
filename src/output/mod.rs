//! Output module — CLI, tracing, plain, and JSON output types.
//! Select via VIBE_LOG_TYPE env var.

pub mod cli;
pub mod format;
pub mod json;
pub mod macros;
pub mod msg;
pub mod output;
pub mod plain;
pub mod test;
pub mod tracing;
