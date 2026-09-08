//! Size modifier — returns the length of a string or list as a string.

use anyhow::Result;

use crate::modifier::modifier::ITEM_SEP;

use super::modifier::Modifier;
use super::modifier::ModifierKey;

pub struct SizeModifier;

impl Modifier for SizeModifier {
    fn key(&self) -> ModifierKey {
        ModifierKey::Size
    }

    fn apply(&self, value: &str, _arg: &str) -> Result<String> {
        let count = if value.contains(ITEM_SEP) {
            value.split(ITEM_SEP).count()
        } else {
            value.len()
        };
        Ok(count.to_string())
    }
}
