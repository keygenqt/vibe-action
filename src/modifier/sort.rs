//! Sort modifier — sorts a string (characters) or list alphabetically.

use anyhow::Result;

use crate::modifier::modifier::ITEM_SEP;

use super::modifier::Modifier;
use super::modifier::ModifierKey;

pub struct SortModifier;

impl SortModifier {
    fn direction_from_arg(arg: &str) -> Result<bool> {
        match arg {
            "" | "asc" => Ok(true),
            "desc" => Ok(false),
            _ => anyhow::bail!("Unknown sort direction: {}", arg),
        }
    }
}

impl Modifier for SortModifier {
    fn key(&self) -> ModifierKey {
        ModifierKey::Sort
    }

    fn apply(&self, value: &str, arg: &str) -> Result<String> {
        let ascending = Self::direction_from_arg(arg)?;
        if value.contains(ITEM_SEP) {
            let mut items: Vec<&str> = value.split(ITEM_SEP).collect();
            items.sort();
            if !ascending {
                items.reverse();
            }
            Ok(items.join(ITEM_SEP))
        } else {
            let mut chars: Vec<char> = value.chars().collect();
            chars.sort();
            if !ascending {
                chars.reverse();
            }
            Ok(chars.into_iter().collect())
        }
    }
}
