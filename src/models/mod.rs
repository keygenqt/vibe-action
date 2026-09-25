//! Data models for action pipelines.
//!
//! # Model hierarchy
//!
//! `PipelinesModel` (collection) → `PipelineModel` (one YAML file) →
//! `ActionModel` (one step) → `ValCandidate` (one data source).
//! `ArgActionModel` defines CLI arguments; `PipelineApiModel` defines
//! IDE plugin metadata.
//!
//! # Resolution flow
//!
//! 1. `PipelinesModel::load()` — scan actions directory, validate changed
//!    files, cache unchanged, save defaults on first run.
//! 2. `PipelineModel::apply_args()` — resolve CLI flags into `input_tags`.
//! 3. `PipelineModel::apply_system_tags()` — resolve `system_*` tags.
//! 4. `PipelineModel::apply_query_tags()` — resolve `query_*` tags
//!    (clipboard, file path, etc.).
//! 5. `PipelineModel::validate_query_tags()` — reject empty query values
//!    without a `when` guard.
//!
//! # Submodules
//!
//! - `action` — `ActionModel` (tag, run size, val candidates, action prompt)
//!   and `ValCandidate` (data source, mods chain, when/fail guards, each
//!   split/merge). `ActionRun`: `cmd`, `value`, `tiny`..`large`, `vision`.
//! - `api` — `PipelineApiModel`: IDE plugin metadata (input source, output
//!   target, extra args). Ignored by CLI runtime.
//! - `arg` — `ArgActionModel`: CLI argument definition (name, short flag,
//!   input type, default). Converts to `clap::Arg`.
//! - `pipeline` — `PipelineModel`: one YAML action file. Loading, argument
//!   resolution, system/query tag resolution, validation.
//! - `pipelines` — `PipelinesModel`: collection loader with file-system
//!   cache, lock, and default pipeline seeding.

pub mod action;
pub mod api;
pub mod arg;
pub mod pipeline;
pub mod pipelines;
