//! Action group configuration: local dirs and git repositories.
//! See [`crate::configs`] module-level docs for group loading.

use serde::Deserialize;
use serde::Serialize;

/// External action group declared in config.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupConfig {
    /// Git repository URL (cloned once into the cache).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub git: Option<String>,
    /// Local dir with YAML pipelines (no git), or a subfolder inside the git clone ('/' = repo root).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// Branch, tag, or commit. Only valid with a `git` source.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#ref: Option<String>,
    /// Group name used as the CLI subcommand (vibe <name> <action>).
    pub name: String,
    /// Short description for help.
    pub about: String,
}

impl GroupConfig {
    /// Cache dir key: git URL + ref. Groups sharing a repo and ref share one
    /// clone; a URL or `ref` change produces a new dir and a fresh clone.
    /// `path` is a subfolder inside the clone — must not affect the key.
    pub fn cache_key(&self) -> String {
        let mut input = self.git.clone().unwrap_or_default();
        if let Some(r) = &self.r#ref {
            input.push(':');
            input.push_str(r);
        }
        format!("group-{:x}", md5::compute(input.as_bytes()))
    }
}

impl Default for GroupConfig {
    fn default() -> Self {
        Self {
            name: String::new(),
            about: String::new(),
            git: Some("https://github.com/keygenqt/vibe-action-groups.git".to_string()),
            path: None,
            r#ref: None,
        }
    }
}
