//! Configuration models.
//!
//! # Architecture
//!
//! [`AppConfig`] is the root, loaded once at startup via [`AppConfig::init()`]
//! and cached in a global `OnceLock`. It holds:
//!
//! - `version` — config schema version (must match `CONFIG_VERSION`).
//! - `action` — global runtime settings (system prompt, retries).
//! - `cluster` — LLM provider nodes with role-based routing
//!   (`tiny`/`small`/`medium`/`large`/`vision`).
//! - `pipelines` — loaded [`PipelinesModel`] (not serialized, set at runtime).
//!
//! # Initialization order
//!
//! 1. Output registry from `VIBE_LOG_TYPE` / `VIBE_TRACE_LEVEL`.
//! 2. Config file from `VIBE_CONFIG` or default path; create if missing.
//! 3. Validate config (version, cluster nodes).
//! 4. Load pipelines (scan actions dir, validate, cache).
//!
//! # Cluster routing
//!
//! Each pipeline action declares an [`ActionRun`] size. The config filters
//! cluster nodes by matching [`ClusterRole`]. If no node matches the role,
//! all nodes are used (fallback). Missing role for a required size triggers
//! a warning at pipeline start.
//!
//! # Submodules
//!
//! - `action` — [`ActionConfig`]: system prompt and retry count.
//! - `app` — [`AppConfig`]: root config, init/save, cluster construction,
//!   pipeline lookup, role mismatch detection.
//! - `cluster` — [`ClusterConfig`]: provider, host, model, role, temperature,
//!   context/predict limits, API key, parallelism. [`ClusterRole`] enum.

pub mod action;
pub mod app;
pub mod cluster;
