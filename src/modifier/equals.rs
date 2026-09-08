//! Equals modifier — checks if a string or list equals a value.
//! Supports :not to invert.

use anyhow::Result;

use crate::modifier::modifier::ITEM_SEP;

use super::modifier::Modifier;
use super::modifier::ModifierKey;
use super::modifier::invert;

pub struct EqualsModifier;

impl Modifier for EqualsModifier {
    fn key(&self) -> ModifierKey {
        ModifierKey::Equals
    }

    fn apply(&self, value: &str, arg: &str) -> Result<String> {
        let (pattern, invert_flag) = if let Some(p) = arg.strip_suffix(":not") {
            (p, true)
        } else {
            (arg, false)
        };

        let results: Vec<String> = value
            .split(ITEM_SEP)
            .map(|i| {
                let val = (i == pattern).to_string();
                if invert_flag { invert(&val) } else { val }
            })
            .collect();
        Ok(results.join(ITEM_SEP))
    }
}
