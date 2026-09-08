//! IsFile modifier — checks if a path exists and is a file.
//! Supports :not to invert.

use anyhow::Result;

use crate::modifier::modifier::ITEM_SEP;

use super::modifier::Modifier;
use super::modifier::ModifierKey;
use super::modifier::invert;

pub struct IsFileModifier;

impl Modifier for IsFileModifier {
    fn key(&self) -> ModifierKey {
        ModifierKey::IsFile
    }

    fn apply(&self, value: &str, arg: &str) -> Result<String> {
        let invert_flag = arg == "not";
        let results: Vec<String> = value
            .split(ITEM_SEP)
            .map(|i| {
                let val = std::path::Path::new(i).is_file().to_string();
                if invert_flag { invert(&val) } else { val }
            })
            .collect();
        Ok(results.join(ITEM_SEP))
    }
}
