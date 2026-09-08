//! Scan modifier — scan directory and return list of file paths via vibe-fs.

use crate::modifier::modifier::ITEM_SEP;

use super::modifier::Modifier;
use super::modifier::ModifierKey;
use anyhow::Result;

pub struct ScanModifier;

impl Modifier for ScanModifier {
    fn key(&self) -> ModifierKey {
        ModifierKey::Scan
    }

    fn apply(&self, value: &str, _arg: &str) -> Result<String> {
        let mut results = Vec::new();

        for path in value.split(ITEM_SEP) {
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
            results.extend(files);
        }

        Ok(results.join(ITEM_SEP))
    }
}
