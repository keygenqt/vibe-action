//! CLI command handlers.
//!
//! # Commands
//!
//! - **Dynamic action** — looks up a YAML-defined pipeline by name, resolves
//!   tags, runs the engine loop, prints result. Supports `ask` confirm
//!   (interactive CLI and JSON confirm protocol), `query_prompt` interactive
//!   input, role mismatch warning, and macOS/Linux notification on completion.
//! - **`clean`** — removes stale cache versions, action cache, temp files
//!   (`vibe-*`), and clipboard.
//! - **`status`** — prints version, config/pipeline/cache paths, action
//!   counts (total, with API, custom with API).
//!
//! # Submodules
//!
//! - `action` — dynamic action executor: pipeline resolution → engine loop →
//!   result output.
//! - `clean` — cache and temp file cleanup.
//! - `status` — system status display.

pub mod action;
pub mod clean;
pub mod status;
