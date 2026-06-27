//! Equals modifier — checks if a string or list equals a value.
//! Supports :not to invert.

use anyhow::Result;

use super::modifier::Modifier;
use super::modifier::ModifierKey;
use super::modifier::invert;
use crate::models::context::ContextModel;

pub struct EqualsModifier;

impl Modifier for EqualsModifier {
    fn key(&self) -> ModifierKey {
        ModifierKey::Equals
    }

    fn apply(&self, value: &ContextModel, arg: &str) -> Result<ContextModel> {
        let (pattern, invert_flag) = if let Some(p) = arg.strip_suffix(":not") {
            (p, true)
        } else {
            (arg, false)
        };

        let result = match value {
            ContextModel::String(s) => ContextModel::String((s == pattern).to_string()),
            ContextModel::List(items) => {
                let results: Vec<String> =
                    items.iter().map(|i| (i == pattern).to_string()).collect();
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
