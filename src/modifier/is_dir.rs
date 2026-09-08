//! IsDir modifier — checks if a path exists and is a directory.
//! Supports :not to invert.

use anyhow::Result;

use crate::modifier::modifier::ITEM_SEP;

use super::modifier::Modifier;
use super::modifier::ModifierKey;
use super::modifier::invert;

pub struct IsDirModifier;

impl Modifier for IsDirModifier {
    fn key(&self) -> ModifierKey {
        ModifierKey::IsDir
    }

    fn apply(&self, value: &str, arg: &str) -> Result<String> {
        let invert_flag = arg == "not";
        let results: Vec<String> = value
            .split(ITEM_SEP)
            .map(|i| {
                let val = std::path::Path::new(i).is_dir().to_string();
                if invert_flag { invert(&val) } else { val }
            })
            .collect();
        Ok(results.join(ITEM_SEP))
    }
}
