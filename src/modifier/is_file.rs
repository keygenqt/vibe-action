//! IsFile modifier — checks if a path exists and is a file.

use anyhow::Result;

use super::modifier::{Modifier, ModifierKey};
use crate::models::context::ContextModel;

pub struct IsFileModifier;

impl Modifier for IsFileModifier {
    fn key(&self) -> ModifierKey {
        ModifierKey::IsFile
    }

    fn apply(&self, value: &ContextModel, _arg: &str) -> Result<ContextModel> {
        match value {
            ContextModel::String(s) => Ok(ContextModel::Bool(std::path::Path::new(s).is_file())),
            ContextModel::List(items) => {
                let results: Vec<ContextModel> = items
                    .iter()
                    .map(|i| ContextModel::Bool(std::path::Path::new(&i.to_string()).is_file()))
                    .collect();
                Ok(ContextModel::List(results))
            }
            _ => anyhow::bail!("Modifier 'is_file' expects a string or list of paths"),
        }
    }
}
