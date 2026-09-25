//! Scan operator — scan directory and return list of file paths via vibe-fs.

use crate::operator::operator::ITEM_SEP;
use crate::operator::operator::Operator;
use crate::operator::operator::OperatorKey;
use crate::operator::read::read::ReadKey;
use crate::utils;
use anyhow::Result;

pub struct ScanOperator;

impl Operator for ScanOperator {
    fn key(&self) -> OperatorKey {
        ReadKey::Scan.key()
    }

    fn apply(&self, value: &str, _arg: &str) -> Result<String> {
        let mut results = Vec::new();

        for path in value.split(ITEM_SEP) {
            let resolved = utils::path::resolve(path)?;
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
