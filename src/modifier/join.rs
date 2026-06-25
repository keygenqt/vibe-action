//! Join modifier — collapses a list into a single string with specified separators.
//! Supports arg "uniq" for deduplication and unescapes literal sequences like "\n" or "\s".
use super::modifier::{Modifier, ModifierKey};
use crate::models::context::ContextModel;
use anyhow::Result;
use std::collections::HashSet;

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

                // Extract custom separator context or default to newline layout
                let (is_uniq, raw_separator) = if arg.starts_with("uniq") {
                    // Safe index accessor: checked via short-circuit length validation boundary
                    if arg.len() > 4 && arg.as_bytes()[4] == b':' {
                        (true, &arg[5..])
                    } else {
                        (true, "\n")
                    }
                } else if arg.is_empty() {
                    (false, "\n")
                } else {
                    (false, arg)
                };

                // Smart Unescape Pipeline: map human-readable mnemonics to control bytes.
                // Bypasses any rigid YAML trailing-space trimming limits entirely.
                let separator_string = raw_separator
                    .replace("\\n", "\n")
                    .replace("\\t", "\t")
                    .replace("\\s", " "); // Clear, tight space marker shorthand

                for item in items {
                    let s = item.to_string();
                    if s.is_empty() {
                        continue;
                    }
                    if is_uniq && !seen.insert(s.clone()) {
                        continue;
                    }
                    if !buffer.is_empty() {
                        buffer.push_str(&separator_string);
                    }
                    buffer.push_str(&s);
                }
                Ok(ContextModel::String(buffer))
            }
            _ => anyhow::bail!("Modifier 'join' expects a list, but got a scalar value"),
        }
    }
}
