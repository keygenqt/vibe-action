//! Upper modifier — transforms text to UPPERCASE.
//! Works on strings and lists of strings.

use anyhow::Result;

use crate::modifier::modifier::ITEM_SEP;

use super::modifier::Modifier;
use super::modifier::ModifierKey;

pub struct UpperModifier;

impl Modifier for UpperModifier {
    fn key(&self) -> ModifierKey {
        ModifierKey::Upper
    }

    fn apply(&self, value: &str, _arg: &str) -> Result<String> {
        Ok(value
            .split(ITEM_SEP)
            .map(|i| i.to_uppercase())
            .collect::<Vec<_>>()
            .join(ITEM_SEP))
    }
}
