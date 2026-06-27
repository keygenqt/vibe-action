//! Sort modifier — sorts a string (characters) or list alphabetically.

use anyhow::Result;

use super::modifier::Modifier;
use super::modifier::ModifierKey;
use crate::models::context::ContextModel;

pub struct SortModifier;

impl SortModifier {
    fn direction_from_arg(arg: &str) -> Result<bool> {
        match arg {
            "" | "asc" => Ok(true),
            "desc" => Ok(false),
            _ => anyhow::bail!("Unknown sort direction: {}", arg),
        }
    }
}

impl Modifier for SortModifier {
    fn key(&self) -> ModifierKey {
        ModifierKey::Sort
    }

    fn apply(&self, value: &ContextModel, arg: &str) -> Result<ContextModel> {
        let ascending = Self::direction_from_arg(arg)?;
        match value {
            ContextModel::String(s) => {
                let mut chars: Vec<char> = s.chars().collect();
                chars.sort();
                if !ascending {
                    chars.reverse();
                }
                Ok(ContextModel::String(chars.into_iter().collect()))
            }
            ContextModel::List(items) => {
                let mut sorted = items.clone();
                sorted.sort();
                if !ascending {
                    sorted.reverse();
                }
                Ok(ContextModel::List(sorted))
            }
        }
    }
}
