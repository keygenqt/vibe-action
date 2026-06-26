//! Trim modifier — strips characters from ends of strings, or filters list elements.
//! Without arg: trims whitespace, removes empty strings from list.
//! With arg: trims specified characters, removes elements equal to arg.

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
                if arg.is_empty() {
                    Ok(ContextModel::String(trimmed))
                } else if trimmed == arg {
                    Ok(ContextModel::String(String::new()))
                } else {
                    Ok(ContextModel::String(trimmed))
                }
            }
            ContextModel::List(items) => {
                let filtered: Vec<ContextModel> = items
                    .iter()
                    .filter(|i| {
                        let s = i
                            .to_string()
                            .trim_matches(|c: char| c.is_whitespace() || chars.contains(&c))
                            .to_string();
                        if arg.is_empty() {
                            !s.is_empty()
                        } else {
                            s != arg
                        }
                    })
                    .map(|i| {
                        let s = i
                            .to_string()
                            .trim_matches(|c: char| c.is_whitespace() || chars.contains(&c))
                            .to_string();
                        ContextModel::String(s)
                    })
                    .collect();
                Ok(ContextModel::List(filtered))
            }
            _ => anyhow::bail!("Modifier 'trim' expects a string or list"),
        }
    }
}
