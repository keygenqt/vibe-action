//! Application path utilities.
//! Provides config and data directory paths.

use anyhow::Result;
use dirs::home_dir;
use std::env;
use std::path::Path;
use std::path::PathBuf;

use crate::utils::constants::ACTIONS_DIR_NAME;
use crate::utils::constants::CONFIG_DIR_NAME;
use crate::utils::constants::CONFIG_FILE_NAME;

/// Get config directory path
pub fn config_dir() -> PathBuf {
    let home = home_dir().expect("Failed to get home directory");
    home.join(CONFIG_DIR_NAME)
}

/// Get default actions directory path
pub fn actions_dir() -> PathBuf {
    config_dir().join(ACTIONS_DIR_NAME)
}

/// Get default config file path
pub fn config_default_path() -> PathBuf {
    config_dir().join(CONFIG_FILE_NAME)
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
