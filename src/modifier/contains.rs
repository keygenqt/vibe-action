//! Contains modifier — checks if a string or list contains a substring.
//! For strings: returns Bool. For lists: returns List<Bool> per element.

use anyhow::Result;

use super::modifier::Modifier;
use super::modifier::ModifierKey;
use crate::models::context::ContextModel;

pub struct ContainsModifier;

impl Modifier for ContainsModifier {
    fn key(&self) -> ModifierKey {
        ModifierKey::Contains
    }

    fn apply(&self, value: &ContextModel, arg: &str) -> Result<ContextModel> {
        match value {
            ContextModel::String(s) => Ok(ContextModel::Bool(s.contains(arg))),
            ContextModel::List(items) => {
                let results: Vec<ContextModel> = items
                    .iter()
                    .map(|i| ContextModel::Bool(i.to_string().contains(arg)))
                    .collect();
                Ok(ContextModel::List(results))
            }
            _ => anyhow::bail!("Modifier 'contains' expects a string or list"),
        }
    }
}
