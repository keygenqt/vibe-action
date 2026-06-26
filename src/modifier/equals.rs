//! Equals modifier — checks if a string or list equals a value.
//! Supports :not to invert.

use anyhow::Result;

use super::modifier::Modifier;
use super::modifier::ModifierKey;
use super::modifier::invert_bool;
use crate::models::context::ContextModel;

pub struct EqualsModifier;

impl Modifier for EqualsModifier {
    fn key(&self) -> ModifierKey {
        ModifierKey::Equals
    }

    fn apply(&self, value: &ContextModel, arg: &str) -> Result<ContextModel> {
        let (pattern, invert) = if let Some(p) = arg.strip_suffix(":not") {
            (p, true)
        } else {
            (arg, false)
        };
        let result = match value {
            ContextModel::String(s) => ContextModel::Bool(s == pattern),
            ContextModel::List(items) => {
                let results: Vec<ContextModel> = items
                    .iter()
                    .map(|i| ContextModel::Bool(i.to_string() == pattern))
                    .collect();
                ContextModel::List(results)
            }
            _ => anyhow::bail!("Modifier 'equals' expects a string or list"),
        };
        if invert {
            Ok(invert_bool(result))
        } else {
            Ok(result)
        }
    }
}
