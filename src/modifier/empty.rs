//! Empty modifier — checks if a string or list is empty.
//! Supports :not to invert.

use anyhow::Result;

use crate::modifier::modifier::ITEM_SEP;

use super::modifier::Modifier;
use super::modifier::ModifierKey;
use super::modifier::invert;

pub struct EmptyModifier;

impl Modifier for EmptyModifier {
    fn key(&self) -> ModifierKey {
        ModifierKey::Empty
    }

    fn apply(&self, value: &str, arg: &str) -> Result<String> {
        let invert_flag = arg == "not";
        let results: Vec<String> = value
            .split(ITEM_SEP)
            .map(|i| {
                let val = i.is_empty().to_string();
                if invert_flag { invert(&val) } else { val }
            })
            .collect();
        Ok(results.join(ITEM_SEP))
    }
}
