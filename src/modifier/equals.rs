//! Equals modifier — checks if a string or list equals a value.
//! Returns true if equal, false otherwise.

use anyhow::Result;

use super::modifier::{Modifier, ModifierKey};
use crate::models::context::ContextModel;

pub struct EqualsModifier;

impl Modifier for EqualsModifier {
    fn key(&self) -> ModifierKey {
        ModifierKey::Equals
    }

    fn apply(&self, value: &ContextModel, arg: &str) -> Result<ContextModel> {
        let result = match value {
            ContextModel::String(s) => s == arg,
            ContextModel::List(items) => items.len() == 1 && items[0].to_string() == arg,
            _ => anyhow::bail!("Modifier 'equals' expects a string or list"),
        };
        Ok(ContextModel::Bool(result))
    }
}
