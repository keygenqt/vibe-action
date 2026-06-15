//! Lower modifier — transforms text to lowercase.
//! Works on strings and lists of strings.

use anyhow::Result;

use super::modifier::{Modifier, ModifierKey};
use crate::models::context::ContextModel;

pub struct LowerModifier;

impl Modifier for LowerModifier {
    fn key(&self) -> ModifierKey {
        ModifierKey::Lower
    }

    fn apply(&self, value: &ContextModel, _arg: &str) -> Result<ContextModel> {
        match value {
            ContextModel::String(s) => Ok(ContextModel::String(s.to_lowercase())),
            ContextModel::List(items) => {
                let transformed: Vec<ContextModel> = items
                    .iter()
                    .map(|i| ContextModel::String(i.to_string().to_lowercase()))
                    .collect();
                Ok(ContextModel::List(transformed))
            }
            _ => anyhow::bail!("Modifier 'lower' expects a string or list of strings"),
        }
    }
}
