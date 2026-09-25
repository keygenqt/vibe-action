//! Resolve operator — resolves paths to absolute form.
//! Bare: resolve any path. :dir — only if directory. :file — only if file.
//! Returns empty string if the path doesn't exist or doesn't match.

use crate::operator::operator::Operator;
use crate::operator::operator::OperatorKey;
use crate::operator::operator::map_items;
use crate::operator::read::read::ReadKey;
use crate::utils;
use anyhow::Result;

pub struct ResolveOperator;

impl Operator for ResolveOperator {
    fn key(&self) -> OperatorKey {
        ReadKey::Resolve.key()
    }

    fn apply(&self, value: &str, arg: &str) -> Result<String> {
        map_items(value, |s| {
            let resolved = utils::path::resolve(s).ok().filter(|p| p.exists());
            let result = match arg {
                "dir" => resolved.filter(|p| p.is_dir()),
                "file" => resolved.filter(|p| p.is_file()),
                _ => resolved,
            };
            Ok(result.map(|p| p.display().to_string()).unwrap_or_default())
        })
    }
}
