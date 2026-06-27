//! IsDir modifier — checks if a path exists and is a directory.
//! Supports :not to invert.

use anyhow::Result;

use super::modifier::Modifier;
use super::modifier::ModifierKey;
use super::modifier::invert;
use crate::models::context::ContextModel;

pub struct IsDirModifier;

impl Modifier for IsDirModifier {
    fn key(&self) -> ModifierKey {
        ModifierKey::IsDir
    }

    fn apply(&self, value: &ContextModel, arg: &str) -> Result<ContextModel> {
        let invert_flag = arg == "not";
        let result = match value {
            ContextModel::String(s) => {
                ContextModel::String(std::path::Path::new(s).is_dir().to_string())
            }
            ContextModel::List(items) => {
                let results: Vec<String> = items
                    .iter()
                    .map(|i| std::path::Path::new(i).is_dir().to_string())
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
