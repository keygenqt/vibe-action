//! Join modifier — collapses a list into a single string with separator.
//! For strings: returns unchanged. Supports escape mnemonics \n, \t, \s.

use super::modifier::Modifier;
use super::modifier::ModifierKey;
use crate::models::context::ContextModel;
use anyhow::Result;

pub struct JoinModifier;

impl Modifier for JoinModifier {
    fn key(&self) -> ModifierKey {
        ModifierKey::Join
    }

    fn apply(&self, value: &ContextModel, arg: &str) -> Result<ContextModel> {
        match value {
            ContextModel::String(s) => Ok(ContextModel::String(s.clone())),
            ContextModel::List(items) => {
                let separator = if arg.is_empty() {
                    "\n".to_string()
                } else {
                    arg.replace("\\n", "\n")
                        .replace("\\t", "\t")
                        .replace("\\s", " ")
                };

                Ok(ContextModel::String(items.join(&separator)))
            }
        }
    }
}
