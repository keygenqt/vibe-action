//! Default action templates embedded in the binary.
//!
//! On first run, [`default_pipelines()`] writes all built-in YAML files to
//! `~/.vibe-action/actions/`. Existing files are overwritten only when their
//! `version` field doesn't match [`PIPELINE_VERSION`](crate::utils::constants::PIPELINE_VERSION).
//!
//! # Built-in actions
//!
//! - `comment` — replace TODO with a meaningful comment.
//! - `commit` — AI-generated commit message (with `--dry-run`).
//! - `describe` — describe a screenshot for text-only LLMs.
//! - `explain` — explain what the selected code does.
//! - `extract` — extract structured data or matching lines from text/logs.
//! - `faq` — ask a question about Vibe Action.
//! - `fetch` — fetch a web page or PDF and describe its content.
//! - `find` — semantic file finder by meaning, not name.
//! - `mock` — generate realistic mock data (JSON, YAML, CSV).
//! - `naming` — generate code naming suggestions.
//! - `regex` — generate a regex pattern from a description.
//! - `review` — critically analyze code for bugs and flaws.
//! - `scan` — scan project codebase, export as structured JSON.
//! - `spellcheck` — check and fix spelling in text or files.
//! - `synonyms` — find programming/technical synonyms.
//! - `sysinfo` — generate a human-readable system report.
//! - `tone` — transform rude text into professional tone.
//! - `translate` — translate text or files to another language.
//! - `whois` — identify a person from a screenshot.
//!
//! # Submodules
//!
//! - `default` — [`DefaultPipeline`] trait, [`BuiltinPipeline`] struct,
//!   [`default_pipelines()`] registry, and [`PIPELINE_HEADER`] template.

pub mod default;
