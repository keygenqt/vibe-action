//! Contains modifier — checks if a string or list contains a substring.
//! Supports :not to invert.

use anyhow::Result;

use crate::modifier::modifier::ITEM_SEP;

use super::modifier::Modifier;
use super::modifier::ModifierKey;
use super::modifier::invert;

pub struct ContainsModifier;

impl Modifier for ContainsModifier {
    fn key(&self) -> ModifierKey {
        ModifierKey::Contains
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
                let val = i.contains(pattern).to_string();
                if invert_flag { invert(&val) } else { val }
            })
            .collect();

        Ok(results.join(ITEM_SEP))
    }
}
