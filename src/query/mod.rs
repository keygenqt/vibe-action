//! Query provider trait and registry.
//! Each provider resolves a `query_*` tag.
//!
//! # Provider contract
//!
//! ## query_raw
//! Raw data, no validation, no checks.
//!
//! ## query_prompt
//! Marker only. Implementation lives in the app layer (exception).
//!
//! ## Other query_*
//! Validate the input and return its typed form as a string. Non-empty
//! result wins; empty input → `""` (skipped, `val` moves to the next
//! candidate).
//!
//! - `query_clipboard` — combined clipboard by priority (text → paths → image), token-limited.
//! - `query_clipboard_text` — raw text from the clipboard.
//! - `query_clipboard_path` — copied file paths from the clipboard.
//! - `query_clipboard_image` — clipboard image as base64 PNG.
//! - `query_file_path` — explicit input → existing file path, else `""`.
//! - `query_project_path` — explicit input → project root (walks up to a marker), else `""`.
//! - `query_line` — explicit input → first line, else `""`.
//! - `query_image` — explicit input (URL/file/base64) → validated base64, `Err` if invalid.

mod impls;

pub mod query;
