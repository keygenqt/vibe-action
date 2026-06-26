//! Upper modifier — transforms text to UPPERCASE.
//! Works on strings and lists of strings.

use anyhow::Result;

use super::modifier::Modifier;
use super::modifier::ModifierKey;
use crate::models::context::ContextModel;

pub struct UpperModifier;

impl Modifier for UpperModifier {
    fn key(&self) -> ModifierKey {
        ModifierKey::Upper
    }

    fn apply(&self, value: &ContextModel, _arg: &str) -> Result<ContextModel> {
        match value {
            ContextModel::String(s) => Ok(ContextModel::String(s.to_uppercase())),
            ContextModel::List(items) => {
                let transformed: Vec<ContextModel> = items
                    .iter()
                    .map(|i| ContextModel::String(i.to_string().to_uppercase()))
                    .collect();
                Ok(ContextModel::List(transformed))
            }
            _ => anyhow::bail!("Modifier 'upper' expects a string or list of strings"),
        }
    }
}
