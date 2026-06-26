//! Empty modifier — checks if a string or list is empty.
//! Supports :not to invert.

use anyhow::Result;

use super::modifier::Modifier;
use super::modifier::ModifierKey;
use super::modifier::invert_bool;
use crate::models::context::ContextModel;

pub struct EmptyModifier;

impl Modifier for EmptyModifier {
    fn key(&self) -> ModifierKey {
        ModifierKey::Empty
    }

    fn apply(&self, value: &ContextModel, arg: &str) -> Result<ContextModel> {
        let invert = arg == "not";
        let result = match value {
            ContextModel::String(s) => ContextModel::Bool(s.is_empty()),
            ContextModel::List(items) => {
                let results: Vec<ContextModel> = items
                    .iter()
                    .map(|i| ContextModel::Bool(i.to_string().is_empty()))
                    .collect();
                ContextModel::List(results)
            }
            _ => anyhow::bail!("Modifier 'empty' expects a string or list"),
        };
        if invert {
            Ok(invert_bool(result))
        } else {
            Ok(result)
        }
    }
}
