//! IsDir modifier — checks if a path exists and is a directory.
//! Supports :not to invert.

use anyhow::Result;

use super::modifier::{Modifier, ModifierKey, invert_bool};
use crate::models::context::ContextModel;

pub struct IsDirModifier;

impl Modifier for IsDirModifier {
    fn key(&self) -> ModifierKey {
        ModifierKey::IsDir
    }

    fn apply(&self, value: &ContextModel, arg: &str) -> Result<ContextModel> {
        let invert = arg == "not";
        let result = match value {
            ContextModel::String(s) => ContextModel::Bool(std::path::Path::new(s).is_dir()),
            ContextModel::List(items) => {
                let results: Vec<ContextModel> = items
                    .iter()
                    .map(|i| ContextModel::Bool(std::path::Path::new(&i.to_string()).is_dir()))
                    .collect();
                ContextModel::List(results)
            }
            _ => anyhow::bail!("Modifier 'is_dir' expects a string or list of paths"),
        };
        if invert {
            Ok(invert_bool(result))
        } else {
            Ok(result)
        }
    }
}
