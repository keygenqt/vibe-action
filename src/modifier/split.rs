//! Split modifier — splits a string into a list by separator.
//! Default separator is newline. Supports escape mnemonics \n, \t, \s.
//! For list: returns unchanged.

use anyhow::Result;

use super::modifier::Modifier;
use super::modifier::ModifierKey;
use crate::models::context::ContextModel;

pub struct SplitModifier;

impl Modifier for SplitModifier {
    fn key(&self) -> ModifierKey {
        ModifierKey::Split
    }

    fn apply(&self, value: &ContextModel, arg: &str) -> Result<ContextModel> {
        match value {
            ContextModel::String(s) => {
                let separator = if arg.is_empty() {
                    "\n".to_string()
                } else {
                    arg.replace("\\n", "\n")
                        .replace("\\t", "\t")
                        .replace("\\s", " ")
                };

                let items: Vec<String> = s.split(&separator).map(|line| line.to_string()).collect();
                Ok(ContextModel::List(items))
            }
            ContextModel::List(_) => Ok(value.clone()),
        }
    }
}
