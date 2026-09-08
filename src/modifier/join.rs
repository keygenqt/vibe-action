//! Join modifier — collapses a list into a single string with separator.
//! For strings: returns unchanged. Supports escape mnemonics \n, \t, \s.

use crate::modifier::modifier::ITEM_SEP;

use super::modifier::Modifier;
use super::modifier::ModifierKey;
use anyhow::Result;

pub struct JoinModifier;

impl Modifier for JoinModifier {
    fn key(&self) -> ModifierKey {
        ModifierKey::Join
    }

    fn apply(&self, value: &str, arg: &str) -> Result<String> {
        let separator = if arg.is_empty() {
            "\n".to_string()
        } else {
            arg.replace("\\n", "\n")
                .replace("\\t", "\t")
                .replace("\\s", " ")
        };
        Ok(value.split(ITEM_SEP).collect::<Vec<_>>().join(&separator))
    }
}
