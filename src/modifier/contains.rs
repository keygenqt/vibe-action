//! Contains modifier — checks if a string or list contains a substring.
//! Returns true if found, false otherwise.

use anyhow::Result;

use super::modifier::{Modifier, ModifierKey};
use crate::models::context::ContextModel;

pub struct ContainsModifier;

impl Modifier for ContainsModifier {
    fn key(&self) -> ModifierKey {
        ModifierKey::Contains
    }

    fn apply(&self, value: &ContextModel, arg: &str) -> Result<ContextModel> {
        let result = match value {
            ContextModel::String(s) => s.contains(arg),
            ContextModel::List(items) => items.iter().any(|i| i.to_string().contains(arg)),
            _ => anyhow::bail!("Modifier 'contains' expects a string or list"),
        };
        Ok(ContextModel::Bool(result))
    }
}
