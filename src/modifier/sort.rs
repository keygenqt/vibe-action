//! Sort modifier — sorts a list alphabetically.

use anyhow::Result;

use super::modifier::{Modifier, ModifierKey};
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
            ContextModel::List(items) => {
                let mut sorted: Vec<String> = items.iter().map(|i| i.to_string()).collect();
                sorted.sort();
                if !ascending {
                    sorted.reverse();
                }
                let sorted: Vec<ContextModel> =
                    sorted.into_iter().map(ContextModel::String).collect();
                Ok(ContextModel::List(sorted))
            }
            _ => anyhow::bail!("Modifier 'sort' expects a list"),
        }
    }
}
