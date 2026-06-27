//! IsFile modifier — checks if a path exists and is a file.
//! Supports :not to invert.

use anyhow::Result;

use super::modifier::Modifier;
use super::modifier::ModifierKey;
use super::modifier::invert;
use crate::models::context::ContextModel;

pub struct IsFileModifier;

impl Modifier for IsFileModifier {
    fn key(&self) -> ModifierKey {
        ModifierKey::IsFile
    }

    fn apply(&self, value: &ContextModel, arg: &str) -> Result<ContextModel> {
        let invert_flag = arg == "not";
        let result = match value {
            ContextModel::String(s) => {
                ContextModel::String(std::path::Path::new(s).is_file().to_string())
            }
            ContextModel::List(items) => {
                let results: Vec<String> = items
                    .iter()
                    .map(|i| std::path::Path::new(i).is_file().to_string())
                    .collect();
                ContextModel::List(results)
            }
        };
        if invert_flag {
            Ok(invert(result))
        } else {
            Ok(result)
        }
    }
}
