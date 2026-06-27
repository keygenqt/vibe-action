//! Uniq modifier — removes duplicate characters from a string or duplicate elements from a list.

use anyhow::Result;
use std::collections::HashSet;

use super::modifier::Modifier;
use super::modifier::ModifierKey;
use crate::models::context::ContextModel;

pub struct UniqModifier;

impl Modifier for UniqModifier {
    fn key(&self) -> ModifierKey {
        ModifierKey::Uniq
    }

    fn apply(&self, value: &ContextModel, _arg: &str) -> Result<ContextModel> {
        match value {
            ContextModel::String(s) => {
                let mut seen = HashSet::new();
                let result: String = s.chars().filter(|c| seen.insert(*c)).collect();
                Ok(ContextModel::String(result))
            }
            ContextModel::List(items) => {
                let mut seen = HashSet::new();
                let result: Vec<String> = items
                    .iter()
                    .filter(|&i| seen.insert(i.clone()))
                    .cloned()
                    .collect();
                Ok(ContextModel::List(result))
            }
        }
    }
}
