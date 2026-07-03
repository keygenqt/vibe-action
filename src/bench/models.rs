//! Benchmark data models.

use indexmap::IndexMap;
use serde::Deserialize;

/// Root benchmark configuration loaded from benchmarks.yaml.
#[derive(Debug, Deserialize)]
pub struct BenchmarksConfig {
    pub benchmarks: IndexMap<String, Vec<BenchmarkCase>>,
}

/// A single benchmark case for an action.
#[derive(Debug, Deserialize)]
pub struct BenchmarkCase {
    /// Arguments passed to the action.
    /// Empty map = no arguments.
    #[serde(default)]
    pub args: IndexMap<String, String>,
}
