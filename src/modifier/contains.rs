//! Contains modifier — checks if a string or list contains a substring.
//! Supports :not to invert.

use anyhow::Result;

use super::modifier::Modifier;
use super::modifier::ModifierKey;
use super::modifier::invert;
use crate::models::context::ContextModel;

pub struct ContainsModifier;

impl Modifier for ContainsModifier {
    fn key(&self) -> ModifierKey {
        ModifierKey::Contains
    }

    fn apply(&self, value: &ContextModel, arg: &str) -> Result<ContextModel> {
        let (pattern, invert_flag) = if let Some(p) = arg.strip_suffix(":not") {
            (p, true)
        } else {
            (arg, false)
        };

        let result = match value {
            ContextModel::String(s) => ContextModel::String((s.contains(pattern)).to_string()),
            ContextModel::List(items) => {
                let results: Vec<String> = items
                    .iter()
                    .map(|i| (i.contains(pattern)).to_string())
                    .collect();
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
