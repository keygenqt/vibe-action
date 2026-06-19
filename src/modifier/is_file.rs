//! IsFile modifier — checks if a path exists and is a file.
//! Supports :not to invert.

use anyhow::Result;

use super::modifier::{Modifier, ModifierKey, invert_bool};
use crate::models::context::ContextModel;

pub struct IsFileModifier;

impl Modifier for IsFileModifier {
    fn key(&self) -> ModifierKey {
        ModifierKey::IsFile
    }

    fn apply(&self, value: &ContextModel, arg: &str) -> Result<ContextModel> {
        let invert = arg == "not";
        let result = match value {
            ContextModel::String(s) => ContextModel::Bool(std::path::Path::new(s).is_file()),
            ContextModel::List(items) => {
                let results: Vec<ContextModel> = items
                    .iter()
                    .map(|i| ContextModel::Bool(std::path::Path::new(&i.to_string()).is_file()))
                    .collect();
                ContextModel::List(results)
            }
            _ => anyhow::bail!("Modifier 'is_file' expects a string or list of paths"),
        };
        if invert {
            Ok(invert_bool(result))
        } else {
            Ok(result)
        }
    }
}
