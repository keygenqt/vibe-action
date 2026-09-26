//! GroupConfig validation.
//! See [`crate::validate`] module-level docs for validation rules.

use anyhow::Result;

use crate::configs::group::GroupConfig;
use crate::utils::clap::SYSTEM_COMMANDS;
use crate::validate::ValidateTrait;

impl ValidateTrait for GroupConfig {
    /// Validate a single group config.
    fn validate(&self) -> Result<()> {
        if self.name.trim().is_empty() {
            anyhow::bail!("Group must have a name.");
        }
        if !self
            .name
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_' || c == '-')
        {
            anyhow::bail!(
                "Group name '{}' must only contain lowercase letters, numbers, '_' or '-'.",
                self.name
            );
        }
        if SYSTEM_COMMANDS.contains(&self.name.as_str()) || self.name == "help" {
            anyhow::bail!(
                "Group name '{}' conflicts with a built-in command.",
                self.name
            );
        }
        if self.about.trim().is_empty() {
            anyhow::bail!("Group '{}' must have a non-empty about.", self.name);
        }
        // Source: git (optional `path` subfolder inside the clone), or local path.
        match (&self.git, &self.path) {
            (Some(git), path) => {
                if git.trim().is_empty() {
                    anyhow::bail!("Group '{}': 'git' URL is empty.", self.name);
                }
                if let Some(p) = path {
                    if p.trim().is_empty() {
                        anyhow::bail!("Group '{}': 'path' is empty.", self.name);
                    }
                    // Leading '/' is allowed (repo root convention); any
                    // '..' would escape the clone.
                    let sub = p.trim_matches('/');
                    if sub.split('/').any(|c| c == "..") {
                        anyhow::bail!(
                            "Group '{}': 'path' must be a subfolder inside the git clone, no '..'.",
                            self.name
                        );
                    }
                }
            }
            (None, Some(dir)) => {
                if dir.trim().is_empty() {
                    anyhow::bail!("Group '{}': 'path' is empty.", self.name);
                }
            }
            (None, None) => {
                anyhow::bail!("Group '{}': either 'git' or 'path' is required.", self.name);
            }
        }

        if let Some(r) = &self.r#ref {
            if r.trim().is_empty() {
                anyhow::bail!("Group '{}': 'ref' must not be empty.", self.name);
            }
            if self.git.is_none() {
                anyhow::bail!(
                    "Group '{}': 'ref' is only allowed with a 'git' source.",
                    self.name
                );
            }
        }
        Ok(())
    }
}
