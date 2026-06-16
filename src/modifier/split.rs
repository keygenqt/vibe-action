//! Split modifier — splits a string into a list by newlines.

use anyhow::Result;

use super::modifier::{Modifier, ModifierKey};
use crate::models::context::ContextModel;

pub struct SplitModifier;

impl Modifier for SplitModifier {
    fn key(&self) -> ModifierKey {
        ModifierKey::Split
    }

    fn apply(&self, value: &ContextModel, _arg: &str) -> Result<ContextModel> {
        match value {
            ContextModel::String(s) => {
                let items: Vec<ContextModel> = s
                    .split('\n')
                    .map(|line| ContextModel::String(line.to_string()))
                    .collect();
                Ok(ContextModel::List(items))
            }
            _ => anyhow::bail!("Modifier 'split' expects a string"),
        }
    }
}
