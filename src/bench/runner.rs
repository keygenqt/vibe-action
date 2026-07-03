//! Benchmark runner.
//! Executes benchmark cases from benchmarks.yaml and collects results.

use std::process::Command;

use crate::{
    bench::models::{BenchmarkCase, BenchmarksConfig},
    utils,
};

/// Benchmark configuration and runner.
pub struct Bench {
    pub config: BenchmarksConfig,
}

impl Bench {
    /// Load benchmarks from file, create default if missing.
    pub fn load() -> anyhow::Result<Self> {
        let path = utils::path::config_dir().join("bench.yaml");
        let config = if path.exists() {
            let content = std::fs::read_to_string(&path)?;
            yaml_serde::from_str(&content)?
        } else {
            let default_yaml = include_str!("bench.yaml");
            std::fs::write(&path, default_yaml)?;
            yaml_serde::from_str(default_yaml)?
        };

        Ok(Self { config })
    }

    pub async fn run(&self, action: &str, case: &BenchmarkCase) -> anyhow::Result<String> {
        let mut cmd = Command::new(std::env::current_exe()?);
        cmd.arg(action);
        cmd.env("VIBE_LOG_TYPE", "plain");
        cmd.stdout(std::process::Stdio::piped());
        cmd.stderr(std::process::Stdio::piped());

        for (key, value) in &case.args {
            cmd.arg(format!("--{}", key));
            cmd.arg(value);
        }

        let output = cmd.output()?;
        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();

        if output.status.success() {
            Ok(format!("{}{}", stdout, stderr))
        } else {
            Err(anyhow::anyhow!("{}{}", stderr, stdout))
        }
    }
}
