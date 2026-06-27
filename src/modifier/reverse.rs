//! Reverse modifier — reverses a string or list.

use anyhow::Result;

use super::modifier::Modifier;
use super::modifier::ModifierKey;
use crate::models::context::ContextModel;

pub struct ReverseModifier;

impl Modifier for ReverseModifier {
    fn key(&self) -> ModifierKey {
        ModifierKey::Reverse
    }

    fn apply(&self, value: &ContextModel, _arg: &str) -> Result<ContextModel> {
        match value {
            ContextModel::String(s) => {
                let reversed: String = s.chars().rev().collect();
                Ok(ContextModel::String(reversed))
            }
            ContextModel::List(items) => {
                let mut reversed = items.clone();
                reversed.reverse();
                Ok(ContextModel::List(reversed))
            }
        }
    }
}
