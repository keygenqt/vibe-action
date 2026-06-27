//! Trim modifier — strips characters from ends of strings, or filters list elements.

use anyhow::Result;

use super::modifier::Modifier;
use super::modifier::ModifierKey;
use crate::models::context::ContextModel;

pub struct TrimModifier;

impl Modifier for TrimModifier {
    fn key(&self) -> ModifierKey {
        ModifierKey::Trim
    }

    fn apply(&self, value: &ContextModel, arg: &str) -> Result<ContextModel> {
        let chars: Vec<char> = arg.chars().collect();
        match value {
            ContextModel::String(s) => {
                let trimmed = s
                    .trim_matches(|c: char| c.is_whitespace() || chars.contains(&c))
                    .to_string();
                if !arg.is_empty() && trimmed == arg {
                    Ok(ContextModel::String(String::new()))
                } else {
                    Ok(ContextModel::String(trimmed))
                }
            }
            ContextModel::List(items) => {
                let filtered: Vec<String> = items
                    .iter()
                    .filter_map(|i| {
                        let trimmed = i
                            .trim_matches(|c: char| c.is_whitespace() || chars.contains(&c))
                            .to_string();
                        if arg.is_empty() && trimmed.is_empty() {
                            None
                        } else if !arg.is_empty() && trimmed == arg {
                            None
                        } else {
                            Some(trimmed)
                        }
                    })
                    .collect();
                Ok(ContextModel::List(filtered))
            }
        }
    }
}
