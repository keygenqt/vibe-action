//! Size modifier — returns the length of a string or list as a number.

use anyhow::Result;

use super::modifier::{Modifier, ModifierKey};
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
            _ => anyhow::bail!("Modifier 'size' expects a string or list"),
        };
        Ok(ContextModel::Number(count as f64))
    }
}
