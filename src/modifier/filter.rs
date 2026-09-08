//! Filter modifier — removes items containing the arg substring.
//! Prefixes: eq: exact-match, not: keep-only-matching.

use anyhow::Result;

use crate::modifier::modifier::ITEM_SEP;

use super::modifier::Modifier;
use super::modifier::ModifierKey;

pub struct FilterModifier;

impl Modifier for FilterModifier {
    fn key(&self) -> ModifierKey {
        ModifierKey::Filter
    }

    fn apply(&self, value: &str, arg: &str) -> Result<String> {
        let (mode, pattern, keep_matching) = match arg.strip_prefix("not:") {
            Some(rest) => ("sub", rest, true),
            None => match arg.strip_prefix("eq:") {
                Some(rest) => ("eq", rest, false),
                None => ("sub", arg, false),
            },
        };
        let items: Vec<&str> = value
            .split(ITEM_SEP)
            .filter(|i| {
                let matched = if pattern.is_empty() {
                    false
                } else if mode == "eq" {
                    *i == pattern
                } else {
                    i.contains(pattern)
                };
                if keep_matching { matched } else { !matched }
            })
            .collect();
        Ok(items.join(ITEM_SEP))
    }
}
