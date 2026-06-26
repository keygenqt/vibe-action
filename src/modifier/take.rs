//! Take modifier — returns first N characters of a string or first N elements of a list.

use anyhow::Result;

use super::modifier::Modifier;
use super::modifier::ModifierKey;
use crate::models::context::ContextModel;

pub struct TakeModifier;

impl Modifier for TakeModifier {
    fn key(&self) -> ModifierKey {
        ModifierKey::Take
    }

    fn apply(&self, value: &ContextModel, arg: &str) -> Result<ContextModel> {
        let n: usize = arg.parse().unwrap_or(0);
        match value {
            ContextModel::String(s) => {
                let chars: Vec<char> = s.chars().take(n).collect();
                Ok(ContextModel::String(chars.into_iter().collect()))
            }
            ContextModel::List(items) => {
                let taken: Vec<ContextModel> = items.iter().take(n).cloned().collect();
                Ok(ContextModel::List(taken))
            }
            _ => anyhow::bail!("Modifier 'take' expects a string or list"),
        }
    }
}
