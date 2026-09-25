//! Output module — CLI, tracing, plain, JSON, and test output strategies.
//!
//! # Architecture
//!
//! All output flows through a single [`Output`] trait. [`OutputRegistry`]
//! selects the active strategy at startup from `VIBE_LOG_TYPE` (or `Test`
//! when `VIBE_TEST=1`). Every call site uses `print_template!` / `print_text!`
//! macros; no direct `println!` outside this module.
//!
//! # Output types
//!
//! - `Cli` — ANSI colors, framed success block with Markdown rendering and
//!   syntax highlighting (`syntect` + `termimad`).
//! - `Tracing` — delegates to the `tracing` crate (`VIBE_TRACE_LEVEL`).
//! - `Plain` — no ANSI, no formatting. For CI and redirected output.
//! - `Json` — structured `{"level":"...","value":{...}}` envelope for IDE
//!   plugin / Kotlin-Compose UI.
//! - `Test` — minimal: only `error` and `success` visible; all others no-op.
//!
//! # Message model
//!
//! [`OutputMsg`] carries a semantic [`OutputKind`], a template string, and
//! key-value fields. [`FormatOutput`] renders templates with styled
//! placeholders (`{key|color|style}` for CLI; plain substitution otherwise).
//! [`ExportContext`] tags messages for plugin routing
//! (`Actions`/`Status`/`Success`/`Confirm`).
//!
//! # Submodules
//!
//! - `cli` — ANSI-colored terminal output with progress carriage return.
//! - `format` — template rendering, Markdown layout, syntax highlighting.
//! - `json` — JSON envelope output.
//! - `msg` — [`OutputMsg`] struct, `print_template!` / `print_text!` macros.
//! - `output` — [`Output`] trait, [`OutputRegistry`], [`OutputKind`],
//!   [`OutputType`], [`ExportContext`].
//! - `plain` — unformatted stdout/stderr output.
//! - `test` — minimal output for test assertions.
//! - `tracing` — structured log output via `tracing` crate.

pub mod cli;
pub mod format;
pub mod json;
pub mod msg;
pub mod output;
pub mod plain;
pub mod test;
pub mod tracing;
