//! Lower modifier — transforms text to lowercase.
//! Works on strings and lists of strings.

use anyhow::Result;

use crate::modifier::modifier::ITEM_SEP;

use super::modifier::Modifier;
use super::modifier::ModifierKey;

pub struct LowerModifier;

impl Modifier for LowerModifier {
    fn key(&self) -> ModifierKey {
        ModifierKey::Lower
    }

    fn apply(&self, value: &str, _arg: &str) -> Result<String> {
        Ok(value
            .split(ITEM_SEP)
            .map(|i| i.to_lowercase())
            .collect::<Vec<_>>()
            .join(ITEM_SEP))
    }
}
