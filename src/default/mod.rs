//! Default action templates embedded in the binary.
//!
//! On first run, [`default_pipelines()`] writes all built-in YAML files to
//! `~/.vibe-action/actions/`. Existing files are overwritten only when their
//! `version` field doesn't match [`PIPELINE_VERSION`](crate::utils::constants::PIPELINE_VERSION).
//!
//! # Built-in actions
//!
//! - `docs` — ask a question about Vibe Action.
//! - `info` — show system information.
//!
//! Additional actions ship in external groups (git repos or local dirs)
//! declared in the `groups` config and are loaded by
//! [`PipelinesModel::load`](crate::models::pipelines::PipelinesModel::load).
//!
//! # Submodules
//!
//! - `default` — [`DefaultPipeline`] trait, [`BuiltinPipeline`] struct,
//!   [`default_pipelines()`] registry, and [`PIPELINE_HEADER`] template.

pub mod default;
