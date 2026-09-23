//! Validation trait and utilities.
//!
//! # Validation contract
//!
//! Every model implements [`ValidateTrait`] with a single `validate() -> Result<()>`
//! method. Validation runs after parsing, before execution. A failure aborts
//! startup with a clear error — no invalid config reaches the engine.
//!
//! # Validation order
//!
//! `PipelinesModel` → `PipelineModel` → `ActionModel` / `ArgActionModel` /
//! `PipelineApiModel` → `ClusterConfig`.
//! Parent validators delegate to children; the root call is a single
//! `pipelines.validate()`.
//!
//! # Submodules
//!
//! - `action` — non-empty command/prompt; `when`/`fail` use inspect operators
//!   only; `mods` use known operators; `{name}` interpolation references only
//!   declared val-candidate names; regex validity.
//! - `api` — `input` and `args` values must be known `query_*` keys; output
//!   target is enum (Serde-validated).
//! - `app` — config version match; each cluster node validated.
//! - `arg` — non-empty name, alphanumeric + underscore only; short flag is an
//!   ASCII letter.
//! - `cluster` — non-empty provider/model; host is a valid URL; temperature in
//!   `0.0..=2.0`; `num_ctx`, `num_predict`, `parallel` > 0.
//! - `pipeline` — version match; non-empty name; no duplicate tags/args;
//!   `query_*`/`system_*` prefixes are reserved; val `data` references a known
//!   tag (not self, not bare `query`); check regex validity.
//! - `pipelines` — no duplicate action names across pipelines; no conflicts
//!   with built-in CLI commands.

use anyhow::Result;

pub mod action;
pub mod api;
pub mod app;
pub mod arg;
pub mod cluster;
pub mod pipeline;
pub mod pipelines;

/// Trait for types that can be validated.
pub trait ValidateTrait {
    fn validate(&self) -> Result<()>;
}
