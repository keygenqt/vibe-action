//! Join modifier — collapses a list into a single string with newline separator.
//! Supports arg "uniq" for deduplication.

use anyhow::Result;
use std::collections::HashSet;

use super::modifier::{Modifier, ModifierKey};
use crate::models::context::ContextModel;

pub struct JoinModifier;

impl Modifier for JoinModifier {
    fn key(&self) -> ModifierKey {
        ModifierKey::Join
    }

    fn apply(&self, value: &ContextModel, arg: &str) -> Result<ContextModel> {
        match value {
            ContextModel::List(items) => {
                let mut buffer = String::new();
                let mut seen = HashSet::new();
                let is_uniq = arg == "uniq";
                for item in items {
                    let s = item.to_string();
                    if s.is_empty() {
                        continue;
                    }
                    if is_uniq && !seen.insert(s.clone()) {
                        continue;
                    }
                    if !buffer.is_empty() {
                        buffer.push('\n');
                    }
                    buffer.push_str(&s);
                }
                Ok(ContextModel::String(buffer))
            }
            _ => anyhow::bail!("Modifier 'join' expects a list, but got a scalar value"),
        }
    }
}
