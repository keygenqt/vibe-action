//! Uniq modifier — removes duplicate characters from a string or duplicate elements from a list.

use anyhow::Result;
use std::collections::HashSet;

use crate::modifier::modifier::ITEM_SEP;

use super::modifier::Modifier;
use super::modifier::ModifierKey;

pub struct UniqModifier;

impl Modifier for UniqModifier {
    fn key(&self) -> ModifierKey {
        ModifierKey::Uniq
    }

    fn apply(&self, value: &str, _arg: &str) -> Result<String> {
        if value.contains(ITEM_SEP) {
            let mut seen = HashSet::new();
            let result: Vec<&str> = value.split(ITEM_SEP).filter(|i| seen.insert(*i)).collect();
            Ok(result.join(ITEM_SEP))
        } else {
            let mut seen = HashSet::new();
            let result: String = value.chars().filter(|c| seen.insert(*c)).collect();
            Ok(result)
        }
    }
}
