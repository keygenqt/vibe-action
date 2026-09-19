//! Fetch operator — download URLs to temp files or resolve local paths.

use crate::operator::operator::Operator;
use crate::operator::operator::OperatorKey;
use crate::operator::operator::map_items;
use crate::operator::read::read::ReadKey;
use crate::utils;
use anyhow::Result;
use std::path::Path;

pub struct FetchOperator;

impl FetchOperator {
    /// Process a single value: URL → download, existing file → resolve, else pass through.
    fn process_one(s: &str) -> Result<String> {
        if s.starts_with("http://") || s.starts_with("https://") {
            let path = utils::fetch::download_to_temp(s)?;
            return Ok(path.to_string_lossy().to_string());
        }
        if let Ok(path) = utils::path::resolve(Path::new(s)) {
            if path.exists() && !path.is_dir() {
                return Ok(path.to_string_lossy().to_string());
            }
        }
        Ok(s.to_string())
    }
}

impl Operator for FetchOperator {
    fn key(&self) -> OperatorKey {
        ReadKey::Fetch.key()
    }

    fn apply(&self, value: &str, _arg: &str) -> Result<String> {
        map_items(value, Self::process_one)
    }
}
