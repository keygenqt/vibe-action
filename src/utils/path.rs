//! Application directory layout, env overrides, and path resolution.
//! See [`crate::utils`] module-level docs for summary.

use anyhow::Result;
use dirs::home_dir;
use std::env;
use std::path::Path;
use std::path::PathBuf;

use crate::utils::constants::ACTIONS_DIR_NAME;
use crate::utils::constants::CACHE_VERSION;
use crate::utils::constants::CONFIG_DIR_NAME;
use crate::utils::constants::CONFIG_FILE_NAME;

/// True when running under tests (VIBE_TEST=1).
pub fn is_test() -> bool {
    std::env::var("VIBE_TEST").unwrap_or_default() == "1"
}

/// Get config directory path
pub fn config_dir() -> PathBuf {
    let home = home_dir().expect("Failed to get home directory");
    home.join(CONFIG_DIR_NAME)
}

/// Get var directory for runtime data (cache, contexts).
pub fn cache_dir() -> PathBuf {
    if is_test() {
        // Per-process temp cache: each spawned binary gets its own PID-keyed
        // snapshot, so tests never leak pipelines across invocations.
        return std::env::temp_dir()
            .join(format!("vibe-test-{}", std::process::id()))
            .join(CACHE_VERSION);
    }
    config_dir().join("cache").join(CACHE_VERSION)
}

/// Get actions directory path (env VIBE_ACTION_PATH or default).
pub fn actions_dir() -> PathBuf {
    std::env::var("VIBE_ACTION_PATH")
        .ok()
        .and_then(|p| resolve(p).ok())
        .unwrap_or_else(|| config_dir().join(ACTIONS_DIR_NAME))
}

/// Get config file path (env VIBE_CONFIG or default).
pub fn config_path() -> PathBuf {
    std::env::var("VIBE_CONFIG")
        .ok()
        .and_then(|p| resolve(p).ok())
        .unwrap_or_else(|| config_dir().join(CONFIG_FILE_NAME))
}

/// Resolve path to absolute form, expanding ~, . and ..
pub fn resolve(path: impl AsRef<Path>) -> Result<PathBuf> {
    let path = path.as_ref().to_string_lossy().trim().to_string();
    if path.is_empty() {
        anyhow::bail!("Empty path");
    }

    if path == "." {
        return env::current_dir().map_err(|e| anyhow::anyhow!(e));
    }
    if path == ".." {
        return env::current_dir()
            .ok()
            .and_then(|d| d.parent().map(|p| p.to_path_buf()))
            .ok_or_else(|| anyhow::anyhow!("Cannot resolve parent directory"));
    }

    let path = if path.starts_with("~/") {
        env::var("HOME")
            .ok()
            .map(|home| path.replacen("~/", &format!("{}/", home), 1))
            .unwrap_or(path)
    } else {
        path
    };

    let path = if path.starts_with("./") {
        env::current_dir()
            .ok()
            .map(|cwd| path.replacen("./", &format!("{}/", cwd.display()), 1))
            .unwrap_or(path)
    } else if path.starts_with("../") {
        env::current_dir()
            .ok()
            .map(|cwd| format!("{}/{}", cwd.display(), path))
            .unwrap_or(path)
    } else {
        path
    };

    let path = PathBuf::from(&path);

    if path.exists() {
        Ok(dunce::canonicalize(&path).unwrap_or(path))
    } else if path.is_relative() {
        env::current_dir()
            .ok()
            .and_then(|cwd| {
                let full = cwd.join(&path);
                if full.exists() {
                    Some(dunce::canonicalize(&full).unwrap_or(full))
                } else {
                    None
                }
            })
            .ok_or_else(|| anyhow::anyhow!("Path not found: {}", path.display()))
    } else {
        Ok(path)
    }
}
