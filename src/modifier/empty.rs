//! Empty modifier — checks if a string or list is empty.
//! Supports :not to invert.

use anyhow::Result;

use super::modifier::Modifier;
use super::modifier::ModifierKey;
use super::modifier::invert;
use crate::models::context::ContextModel;

pub struct EmptyModifier;

impl Modifier for EmptyModifier {
    fn key(&self) -> ModifierKey {
        ModifierKey::Empty
    }

    fn apply(&self, value: &ContextModel, arg: &str) -> Result<ContextModel> {
        let invert_flag = arg == "not";
        let result = match value {
            ContextModel::String(s) => ContextModel::String(s.is_empty().to_string()),
            ContextModel::List(items) => {
                let results: Vec<String> = items.iter().map(|i| i.is_empty().to_string()).collect();
                ContextModel::List(results)
            }
        };
        if invert_flag {
            Ok(invert(result))
        } else {
            Ok(result)
        }
    }
}
