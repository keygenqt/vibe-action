//! Resolve modifier — resolves paths to absolute form.
//! Expands ~, ./, ../ to absolute paths.

use anyhow::Result;

use super::modifier::Modifier;
use super::modifier::ModifierKey;
use crate::modifier::modifier::ITEM_SEP;
use crate::utils;

pub struct ResolveModifier;

impl Modifier for ResolveModifier {
    fn key(&self) -> ModifierKey {
        ModifierKey::Resolve
    }

    fn apply(&self, value: &str, _arg: &str) -> Result<String> {
        Ok(value
            .split(ITEM_SEP)
            .map(|i| {
                utils::path::resolve(i)
                    .map(|p| p.display().to_string())
                    .unwrap_or_else(|_| i.to_string())
            })
            .collect::<Vec<_>>()
            .join(ITEM_SEP))
    }
}
