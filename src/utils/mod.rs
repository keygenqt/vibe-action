//! Utility modules.
//!
//! Shared helpers with no business logic dependency. Each submodule is
//! self-contained; none import from `engine`, `operator`, `query`, or
//! `system`.
//!
//! # Submodules
//!
//! - `app` — application metadata: name, version, CLI styling (`clap-cargo`).
//! - `clap` — dynamic CLI command tree from YAML pipelines; custom colored
//!   help with grouped actions vs commands.
//! - `clipboard` — all clipboard I/O: text, paths, image, combined reader
//!   (text → paths → image priority); token-limit enforcement; URI list
//!   parsing. No other file should touch clipboard crates directly.
//! - `constants` — directory names, file names, and version strings
//!   (`CONFIG_VERSION`, `PIPELINE_VERSION`, `CACHE_VERSION`).
//! - `escape` — brace-escape helpers for LLM templates and operator arg
//!   parsing. `{X}` → unwrap (escapes `:`/`|`); `{{X}}` → reduce (keeps
//!   braces).
//! - `fetch` — HTTP download: bytes or temp file (deduped by URL hash,
//!   extension from MIME).
//! - `format` — human-readable formatters: duration (`1.23s`/`456ms`),
//!   bytes (`1.2MB`), base64 image extraction from prompt text.
//! - `image` — base64 image detection (prefix scan) and encoding/validation.
//! - `path` — application directory layout (config, cache, actions);
//!   `VIBE_ACTION_PATH`/`VIBE_CONFIG` env overrides; path resolution
//!   (`~`, `./`, `../`, relative → absolute).
//! - `run_guard` — singleton process guard: PID files, stop-file signaling,
//!   stale process cleanup, startup lock. Ensures only one instance runs.
//! - `yaml` — YAML comment injection and escape mnemonic expansion
//!   (`\n`, `\t`, `\s`).

pub mod app;
pub mod clap;
pub mod clipboard;
pub mod constants;
pub mod escape;
pub mod fetch;
pub mod format;
pub mod image;
pub mod path;
pub mod run_guard;
pub mod yaml;
