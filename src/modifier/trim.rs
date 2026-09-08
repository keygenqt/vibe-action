//! Trim modifier — strips characters from ends of strings, or filters list elements.

use anyhow::Result;

use crate::modifier::modifier::ITEM_SEP;

use super::modifier::Modifier;
use super::modifier::ModifierKey;

pub struct TrimModifier;

impl Modifier for TrimModifier {
    fn key(&self) -> ModifierKey {
        ModifierKey::Trim
    }

    fn apply(&self, value: &str, arg: &str) -> Result<String> {
        let chars: Vec<char> = arg.chars().collect();

        if value.contains(ITEM_SEP) {
            let filtered: Vec<String> = value
                .split(ITEM_SEP)
                .filter_map(|i| {
                    let trimmed = i
                        .trim_matches(|c: char| c.is_whitespace() || chars.contains(&c))
                        .to_string();
                    if arg.is_empty() && trimmed.is_empty() {
                        None
                    } else if !arg.is_empty() && trimmed == arg {
                        None
                    } else {
                        Some(trimmed)
                    }
                })
                .collect();
            Ok(filtered.join(ITEM_SEP))
        } else {
            let trimmed = value
                .trim_matches(|c: char| c.is_whitespace() || chars.contains(&c))
                .to_string();
            if !arg.is_empty() && trimmed == arg {
                Ok(String::new())
            } else {
                Ok(trimmed)
            }
        }
    }
}
