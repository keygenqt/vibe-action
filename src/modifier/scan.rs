//! Scan modifier — scan directory and return list of file paths via vibe-fs.

use super::modifier::{Modifier, ModifierKey};
use crate::models::context::ContextModel;
use anyhow::Result;

pub struct ScanModifier;

impl Modifier for ScanModifier {
    fn key(&self) -> ModifierKey {
        ModifierKey::Scan
    }

    fn apply(&self, value: &ContextModel, _arg: &str) -> Result<ContextModel> {
        match value {
            ContextModel::String(path) => {
                let resolved = crate::utils::path::resolve(path)?;
                if !resolved.is_dir() {
                    anyhow::bail!("Not a directory: '{}'", path);
                }
                let result = vibe_fs::scan(path, true, None, None)
                    .map_err(|e| anyhow::anyhow!("Failed to scan '{}': {}", path, e))?;
                let files: Vec<String> = result
                    .changed
                    .into_iter()
                    .map(|p| p.display().to_string())
                    .collect();
                Ok(ContextModel::List(files))
            }
            ContextModel::List(items) => {
                let mut results = Vec::new();
                for item in items {
                    match self.apply(&ContextModel::String(item.clone()), "")? {
                        ContextModel::List(files) => results.extend(files),
                        _ => unreachable!(),
                    }
                }
                Ok(ContextModel::List(results))
            }
        }
    }
}
