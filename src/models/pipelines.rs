//! Aggregated pipelines model with file-system cache and default seeding.
//! See [`crate::models`] module-level docs for context.

use anyhow::Result;
use fs2::FileExt;
use std::collections::HashSet;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

use crate::configs::group::GroupConfig;
use crate::default::default::default_pipelines;
use crate::models::pipeline::PipelineModel;
use crate::utils::clap::SYSTEM_COMMANDS;
use crate::utils::constants;
use crate::utils::{self};
use crate::validate::ValidateTrait;

/// Aggregated actions from all sources.
#[derive(Debug, Clone)]
pub struct PipelinesModel {
    /// All loaded pipelines.
    pub pipelines: Vec<PipelineModel>,
}

impl PipelinesModel {
    /// Find a top-level pipeline by name (exact match, no group).
    pub fn find(&self, name: &str) -> Option<&PipelineModel> {
        self.pipelines
            .iter()
            .find(|f| f.group.is_none() && f.name == name)
    }

    /// Find a pipeline by name within a group (exact match).
    pub fn find_in_group(&self, group: &str, name: &str) -> Option<&PipelineModel> {
        self.pipelines
            .iter()
            .find(|f| f.group.as_deref() == Some(group) && f.name == name)
    }

    /// Load pipelines from actions directory.
    /// Uses cache for validated files, falls back to defaults on first run.
    /// Acquires an exclusive file lock to prevent concurrent cache corruption.
    pub fn load(groups: &[GroupConfig]) -> Result<Self> {
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
        let mut model = Self::load_inner(&path, &cache_dir).map_err(|e| {
            // Wipe cache on any error to force clean reload next time
            vibe_fs::clean(&path, Some(&cache_dir)).ok();
            e
        })?;

        // External groups — after the default load; a group error must not
        // wipe the default snapshot cache.
        Self::load_groups(groups, &cache_dir, &mut model)?;
        Self::validate_groups(groups, &model)?;
        Ok(model)
    }

    /// Inner loader — lock is already held by caller.
    fn load_inner(path: &PathBuf, cache_dir: &PathBuf) -> Result<Self> {
        // Always ensure default files exist (recreates deleted defaults)
        Self::save_defaults(path)?;

        let scan = vibe_fs::scan(path, false, Some(cache_dir), Some(&["yaml", "yml"]))?;

        // No files on disk and nothing in snapshot — first run
        if scan.changed.is_empty() && scan.unchanged.is_empty() {
            // In test mode, just return empty if no fixtures found
            if crate::utils::path::is_test() {
                return Ok(Self { pipelines: vec![] });
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
                let pipeline = PipelineModel::load(file_path)?;
                pipeline.validate().map_err(|e| {
                    let mut msg = e.to_string();
                    if let Some(first) = msg.get_mut(0..1) {
                        first.make_ascii_lowercase();
                    }
                    anyhow::anyhow!("failed to parse {}: {}", file_path.display(), msg)
                })?;
            }
        }

        // Load all valid files (changed just validated, unchanged already in cache)
        let mut actions = Self { pipelines: vec![] };
        for file_path in scan.changed.iter().chain(scan.unchanged.iter()) {
            // Skip deleted files
            if !file_path.exists() {
                continue;
            }
            let pipeline = PipelineModel::load(file_path)?;
            actions.pipelines.push(pipeline);
        }

        // Re-validate whole model if new files were added
        if has_changes {
            actions.validate()?;
        }

        Ok(actions)
    }

    /// Load pipelines from configured groups (git repos and local dirs).
    /// No snapshot cache — group YAML sets are small, validated every run.
    fn load_groups(groups: &[GroupConfig], cache_dir: &Path, model: &mut Self) -> Result<()> {
        for group in groups {
            let src_dir = if let Some(git) = &group.git {
                let dest = cache_dir.join("groups").join(group.cache_key());
                utils::git::clone_once(git, group.r#ref.as_deref(), &dest)
                    .map_err(|e| anyhow::anyhow!("Group '{}': {}", group.name, e))?;
                match group.path.as_deref().map(|p| p.trim_matches('/')) {
                    Some(sub) if !sub.is_empty() => dest.join(sub),
                    _ => dest,
                }
            } else if let Some(dir) = &group.path {
                utils::path::resolve(dir)
                    .map_err(|e| anyhow::anyhow!("Group '{}': {}", group.name, e))?
            } else {
                unreachable!("validated: either git or path is set")
            };

            if !src_dir.is_dir() {
                anyhow::bail!(
                    "Group '{}': source directory '{}' not found.",
                    group.name,
                    src_dir.display()
                );
            }

            let mut loaded = Self::load_dir(&src_dir)
                .map_err(|e| anyhow::anyhow!("Group '{}': {}", group.name, e))?;
            if loaded.is_empty() {
                anyhow::bail!(
                    "Group '{}': no YAML actions found in '{}'.",
                    group.name,
                    src_dir.display()
                );
            }

            // Tag with group name, check in-group duplicates.
            let mut names: HashSet<&str> = HashSet::new();
            for pipeline in &mut loaded {
                if pipeline.name == group.name {
                    anyhow::bail!(
                        "Group '{}': action '{}' has the same name as the group.",
                        group.name,
                        pipeline.name
                    );
                }
                if SYSTEM_COMMANDS.contains(&pipeline.name.as_str()) {
                    anyhow::bail!(
                        "Group '{}': action name '{}' conflicts with a built-in command.",
                        group.name,
                        pipeline.name
                    );
                }
                if !names.insert(pipeline.name.as_str()) {
                    anyhow::bail!(
                        "Group '{}': duplicate action name '{}'.",
                        group.name,
                        pipeline.name
                    );
                }
                pipeline.group = Some(group.name.clone());
            }
            model.pipelines.extend(loaded);
        }
        Ok(())
    }

    /// Recursively load and validate all YAML pipelines in a directory.
    /// Skips the `.git` directory.
    fn load_dir(dir: &Path) -> Result<Vec<PipelineModel>> {
        let mut out = Vec::new();
        for entry in walkdir::WalkDir::new(dir)
            .sort_by_file_name()
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            if path.is_dir() || path.components().any(|c| c.as_os_str() == ".git") {
                continue;
            }
            let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
            if ext != "yaml" && ext != "yml" {
                continue;
            }
            let pipeline = PipelineModel::load(&path.to_path_buf())
                .map_err(|e| anyhow::anyhow!("failed to parse {}: {}", path.display(), e))?;
            pipeline.validate().map_err(|e| {
                let mut msg = e.to_string();
                if let Some(first) = msg.get_mut(0..1) {
                    first.make_ascii_lowercase();
                }
                anyhow::anyhow!("invalid action {}: {}", path.display(), msg)
            })?;
            out.push(pipeline);
        }
        Ok(out)
    }

    /// Cross-validation: unique group names, no collision with flat actions.
    fn validate_groups(groups: &[GroupConfig], model: &Self) -> Result<()> {
        let mut group_names: HashSet<&str> = HashSet::new();
        for group in groups {
            if !group_names.insert(group.name.as_str()) {
                anyhow::bail!("Duplicate group name '{}'.", group.name);
            }
            if model
                .pipelines
                .iter()
                .any(|p| p.group.is_none() && p.name == group.name)
            {
                anyhow::bail!("Group name '{}' conflicts with an action name.", group.name);
            }
        }
        Ok(())
    }

    /// Save default actions to a directory. Overwrites files whose version
    /// is missing or doesn't match PIPELINE_VERSION; creates missing files.
    fn save_defaults(path: &PathBuf) -> Result<()> {
        // Skip in test mode to avoid polluting fixtures
        if crate::utils::path::is_test() {
            return Ok(());
        }
        if !path.exists() {
            fs::create_dir_all(path)?;
        }
        for pipeline in default_pipelines() {
            let yaml = pipeline.pipeline()?;
            let name = pipeline.name()?;
            let file_path = path.join(format!("{}.yaml", name));
            if file_path.exists() {
                match yaml_serde::from_str::<PipelineModel>(&fs::read_to_string(&file_path)?) {
                    Ok(existing) if existing.version == constants::PIPELINE_VERSION => continue,
                    _ => {}
                }
            }
            fs::write(&file_path, &yaml)?;
        }
        Ok(())
    }
}
