//! Size modifier — returns the length of a string or list as a string.

use anyhow::Result;

use super::modifier::Modifier;
use super::modifier::ModifierKey;
use crate::models::context::ContextModel;

pub struct SizeModifier;

impl Modifier for SizeModifier {
    fn key(&self) -> ModifierKey {
        ModifierKey::Size
    }

    fn apply(&self, value: &ContextModel, _arg: &str) -> Result<ContextModel> {
        let count = match value {
            ContextModel::String(s) => s.len(),
            ContextModel::List(items) => items.len(),
        };
        Ok(ContextModel::String(count.to_string()))
    }
}
