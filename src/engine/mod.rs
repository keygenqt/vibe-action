//! Action execution engine.
//!
//! # Pipeline
//!
//! The engine drives a pipeline from resolved tags to final output:
//!
//! 1. `Engine::new()` — collect `input_tags` from the pipeline, build
//!    operator registry.
//! 2. `resolve_data()` — for each action's val candidates, check `when`
//!    guards, resolve data + apply `mods` chain, check `fail` predicates.
//! 3. `next()` — topologically sort actions by data dependencies, return
//!    the next ready action (all candidates resolved, no unresolved deps).
//!    `off` guard passing → dead tag (skipped, empty value). A val name
//!    with no winning candidate is an error, not a skip.
//! 4. `expand()` — substitute `{name}` placeholders from winning candidates;
//!    fan out via `each.split`/`each.merge`; shell-quote for `cmd` runs.
//! 5. `exec_item()` — dispatch by `ActionRun`: `cmd` → shell, `value` →
//!    passthrough, `tiny`..`large`/`vision` → LLM cluster. Apply `check`
//!    regex on output.
//! 6. `store_result()` — join items with merge separator, save under tag.
//!
//! # Action run types
//!
//! - `cmd` — shell command via `sh -c`.
//! - `value` — literal passthrough (no execution).
//! - `tiny`/`small`/`medium`/`large` — LLM prompt, sized for cluster node
//!   selection.
//! - `vision` — LLM prompt with extracted base64 images.
//!
//! # Submodules
//!
//! - `engine` — `Engine` orchestrator: tag resolution, topological sort,
//!   placeholder expansion, candidate selection, result storage.
//! - `cluster` — LLM cluster executor via `vibe-cluster`; progress callback
//!   with token count and duration.
//! - `shell` — synchronous shell command execution (`sh -c`).
//! - `log` — trace-level action execution log (original, resolved, result).

pub mod cluster;
pub mod engine;
pub mod log;
pub mod shell;
