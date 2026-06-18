//! IsDir modifier — checks if a path exists and is a directory.

use anyhow::Result;

use super::modifier::{Modifier, ModifierKey};
use crate::models::context::ContextModel;

pub struct IsDirModifier;

impl Modifier for IsDirModifier {
    fn key(&self) -> ModifierKey {
        ModifierKey::IsDir
    }

    fn apply(&self, value: &ContextModel, _arg: &str) -> Result<ContextModel> {
        match value {
            ContextModel::String(s) => Ok(ContextModel::Bool(std::path::Path::new(s).is_dir())),
            ContextModel::List(items) => {
                let results: Vec<ContextModel> = items
                    .iter()
                    .map(|i| ContextModel::Bool(std::path::Path::new(&i.to_string()).is_dir()))
                    .collect();
                Ok(ContextModel::List(results))
            }
            _ => anyhow::bail!("Modifier 'is_dir' expects a string or list of paths"),
        }
    }
}
