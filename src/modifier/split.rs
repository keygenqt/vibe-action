//! Split modifier — splits a string into a list by separator.
//! Default separator is newline. Supports escape mnemonics \n, \t, \s.
//! For list: returns unchanged.

use anyhow::Result;

use crate::modifier::modifier::ITEM_SEP;

use super::modifier::Modifier;
use super::modifier::ModifierKey;

pub struct SplitModifier;

impl Modifier for SplitModifier {
    fn key(&self) -> ModifierKey {
        ModifierKey::Split
    }

    fn apply(&self, value: &str, arg: &str) -> Result<String> {
        if arg.contains(ITEM_SEP) {
            anyhow::bail!("split separator contains array delimiter");
        }
        let separator = if arg.is_empty() {
            "\n".to_string()
        } else {
            arg.replace("\\n", "\n")
                .replace("\\t", "\t")
                .replace("\\s", " ")
        };
        let items: Vec<&str> = value.split(&separator).collect();
        Ok(items.join(ITEM_SEP))
    }
}
