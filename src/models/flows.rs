//! Aggregated actions model.
//! Loads all flows from configured directories and provides lookup.

use anyhow::Result;
use fs2::FileExt;
use std::fs;
use std::path::PathBuf;

use crate::default::default::default_flows;
use crate::models::flow::FlowModel;
use crate::utils::constants;
use crate::utils::{self};
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
        // Always ensure default files exist (recreates deleted defaults)
        Self::save_defaults(path)?;

        let scan = vibe_fs::scan(path, false, Some(cache_dir), Some(&["yaml", "yml"]))?;

        // No files on disk and nothing in snapshot — first run
        if scan.changed.is_empty() && scan.unchanged.is_empty() {
            // In test mode, just return empty if no fixtures found
            if std::env::var("VIBE_LOG_TYPE").unwrap_or_default() == "test" {
                return Ok(Self { flows: vec![] });
            }
            // Force scan to populate snapshot with defaults
            vibe_fs::scan(path, true, Some(cache_dir), Some(&["yaml", "yml"]))?;
            return Self::load_inner(path, cache_dir);
        }

        // Validate only the files that changed since last snapshot
        let has_changes = !scan.changed.is_empty();
        if has_changes {
            for file_path in &scan.changed {
                // Skip deleted files (they don't exist anymore)
                if !file_path.exists() {
                    continue;
                }
                let flow = FlowModel::load(file_path)?;
                flow.validate().map_err(|e| {
                    let mut msg = e.to_string();
                    if let Some(first) = msg.get_mut(0..1) {
                        first.make_ascii_lowercase();
                    }
                    anyhow::anyhow!("failed to parse {}: {}", file_path.display(), msg)
                })?;
            }
        }

        // Load all valid files (changed just validated, unchanged already in cache)
        let mut actions = Self { flows: vec![] };
        for file_path in scan.changed.iter().chain(scan.unchanged.iter()) {
            // Skip deleted files
            if !file_path.exists() {
                continue;
            }
            let flow = FlowModel::load(file_path)?;
            actions.flows.push(flow);
        }

        // Re-validate whole model if new files were added
        if has_changes {
            actions.validate()?;
        }

        Ok(actions)
    }

    /// Save default actions to a directory. Overwrites files whose version
    /// is missing or doesn't match FLOW_VERSION; creates missing files.
    fn save_defaults(path: &PathBuf) -> Result<()> {
        // Skip in test mode to avoid polluting fixtures
        if std::env::var("VIBE_LOG_TYPE").unwrap_or_default() == "test" {
            return Ok(());
        }
        if !path.exists() {
            fs::create_dir_all(path)?;
        }
        for flow in default_flows() {
            let yaml = flow.flow()?;
            let name = flow.name()?;
            let file_path = path.join(format!("{}.yaml", name));
            if file_path.exists() {
                match yaml_serde::from_str::<FlowModel>(&fs::read_to_string(&file_path)?) {
                    Ok(existing) if existing.version == constants::FLOW_VERSION => continue,
                    _ => {}
                }
            }
            fs::write(&file_path, &yaml)?;
        }
        Ok(())
    }
}
