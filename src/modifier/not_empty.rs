//! NotEmpty modifier — checks if a string or list is not empty.
//! Returns true if not empty, false otherwise.

use anyhow::Result;

use super::modifier::{Modifier, ModifierKey};
use crate::models::context::ContextModel;

pub struct NotEmptyModifier;

impl Modifier for NotEmptyModifier {
    fn key(&self) -> ModifierKey {
        ModifierKey::NotEmpty
    }

    fn apply(&self, value: &ContextModel, _arg: &str) -> Result<ContextModel> {
        let result = match value {
            ContextModel::String(s) => !s.is_empty(),
            ContextModel::List(items) => !items.is_empty(),
            _ => anyhow::bail!("Modifier 'not_empty' expects a string or list"),
        };
        Ok(ContextModel::Bool(result))
    }
}
