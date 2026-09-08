//! Reverse modifier — reverses a string or list.

use anyhow::Result;

use crate::modifier::modifier::ITEM_SEP;

use super::modifier::Modifier;
use super::modifier::ModifierKey;

pub struct ReverseModifier;

impl Modifier for ReverseModifier {
    fn key(&self) -> ModifierKey {
        ModifierKey::Reverse
    }

    fn apply(&self, value: &str, _arg: &str) -> Result<String> {
        if value.contains(ITEM_SEP) {
            let mut items: Vec<&str> = value.split(ITEM_SEP).collect();
            items.reverse();
            Ok(items.join(ITEM_SEP))
        } else {
            Ok(value.chars().rev().collect())
        }
    }
}
