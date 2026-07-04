//! Aggregated actions model.
//! Loads all flows from configured directories and provides lookup.

use anyhow::Result;
use fs2::FileExt;
use std::fs;
use std::path::PathBuf;

use crate::default::default::default_flows;
use crate::models::flow::FlowModel;
use crate::utils;
use crate::validate::ValidateTrait;

/// Aggregated actions from all sources.
#[derive(Debug, Clone)]
pub struct FlowsModel {
    /// All loaded flows.
    pub flows: Vec<FlowModel>,
}

impl FlowsModel {
    /// Find a flow by name (exact match).
    pub fn find(&self, name: &str) -> Option<&FlowModel> {
        self.flows.iter().find(|f| f.name == name)
    }

    /// Load flows from actions directory.
    /// Uses cache for validated files, falls back to defaults on first run.
    /// Acquires an exclusive file lock to prevent concurrent cache corruption.
    pub fn load() -> Result<Self> {
        let path = utils::path::actions_dir();
        let cache_dir = utils::path::cache_dir();

        // Ensure directories exist before any operations
        fs::create_dir_all(&path)?;
        fs::create_dir_all(&cache_dir)?;

        // Prevent concurrent scans from corrupting the snapshot
        let lock_file = cache_dir.join(".lock");
        let _lock = std::fs::File::create(&lock_file)?;
        _lock.lock_exclusive()?;

        // Delegate to inner to keep lock held during recursion
        Self::load_inner(&path, &cache_dir).map_err(|e| {
            // Wipe cache on any error to force clean reload next time
            vibe_fs::clean(&path, Some(&cache_dir)).ok();
            e
        })
    }

    /// Inner loader — lock is already held by caller.
    fn load_inner(path: &PathBuf, cache_dir: &PathBuf) -> Result<Self> {
        let scan = vibe_fs::scan(path, false, Some(cache_dir), Some(&["yaml", "yml"]))?;

        // No files on disk and nothing in snapshot — first run
        if scan.changed.is_empty() && scan.unchanged.is_empty() {
            Self::save_defaults(path)?;
            // Force scan to populate snapshot with defaults
            vibe_fs::scan(path, true, Some(cache_dir), Some(&["yaml", "yml"]))?;
            return Self::load_inner(path, cache_dir);
        }

        // Validate only the files that changed since last snapshot
        let has_changes = !scan.changed.is_empty();
        if has_changes {
            for file_path in &scan.changed {
                let flow = FlowModel::load(file_path)?;
                flow.validate().map_err(|e| {
                    anyhow::anyhow!("Validation failed for {}: {}", file_path.display(), e)
                })?;
            }
        }

        // Load all valid files (changed just validated, unchanged already in cache)
        let mut actions = Self { flows: vec![] };
        for file_path in scan.changed.iter().chain(scan.unchanged.iter()) {
            let flow = FlowModel::load(file_path)?;
            actions.flows.push(flow);
        }

        // Re-validate whole model if new files were added
        if has_changes {
            actions.validate()?;
        }

        Ok(actions)
    }

    /// Save default actions to a directory. Only creates files that don't already exist.
    fn save_defaults(path: &PathBuf) -> Result<()> {
        if !path.exists() {
            fs::create_dir_all(path)?;
        }
        for flow in default_flows() {
            let yaml = flow.flow()?;
            let name = flow.name()?;
            let file_path = path.join(format!("{}.yaml", name));
            if file_path.exists() {
                continue;
            }
            fs::write(&file_path, &yaml)?;
        }
        Ok(())
    }
}
