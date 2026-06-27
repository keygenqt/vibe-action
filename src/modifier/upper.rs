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
                let transformed: Vec<String> = items.iter().map(|i| i.to_uppercase()).collect();
                Ok(ContextModel::List(transformed))
            }
        }
    }
}
