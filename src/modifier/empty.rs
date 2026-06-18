//! Empty modifier — checks if a string or list is empty.
//! Returns true if empty, false otherwise.

use anyhow::Result;

use super::modifier::{Modifier, ModifierKey};
use crate::models::context::ContextModel;

pub struct EmptyModifier;

impl Modifier for EmptyModifier {
    fn key(&self) -> ModifierKey {
        ModifierKey::Empty
    }

    fn apply(&self, value: &ContextModel, _arg: &str) -> Result<ContextModel> {
        let result = match value {
            ContextModel::String(s) => s.is_empty(),
            ContextModel::List(items) => items.is_empty(),
            _ => anyhow::bail!("Modifier 'empty' expects a string or list"),
        };
        Ok(ContextModel::Bool(result))
    }
}
