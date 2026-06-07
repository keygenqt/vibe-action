//! Shell command executor.
//! Runs terminal commands and returns stdout.

use anyhow::Result;
use std::process::Command;

pub struct Shell;

impl Shell {
    /// Execute a shell command and return its output.
    pub async fn exec(command: &str) -> Result<String> {
        let output = Command::new("sh")
            .arg("-c")
            .arg(command)
            .output()
            .map_err(|e| anyhow::anyhow!("Shell command failed: {}", e))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(anyhow::anyhow!(
                "Shell command failed with status {}: {}",
                output.status,
                stderr.trim()
            ));
        }
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    }
}
