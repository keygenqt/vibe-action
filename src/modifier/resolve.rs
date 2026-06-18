//! Resolve modifier — resolves paths to absolute form.
//! Expands ~, ./, ../ to absolute paths.

use anyhow::Result;

use super::modifier::{Modifier, ModifierKey};
use crate::models::context::ContextModel;
use crate::utils;

pub struct ResolveModifier;

impl Modifier for ResolveModifier {
    fn key(&self) -> ModifierKey {
        ModifierKey::Resolve
    }

    fn apply(&self, value: &ContextModel, _arg: &str) -> Result<ContextModel> {
        match value {
            ContextModel::String(s) => {
                let resolved = utils::path::resolve(s)
                    .map(|p| p.display().to_string())
                    .unwrap_or_else(|_| s.clone());
                Ok(ContextModel::String(resolved))
            }
            ContextModel::List(items) => {
                let resolved: Vec<ContextModel> = items
                    .iter()
                    .map(|i| {
                        let s = i.to_string();
                        let resolved = utils::path::resolve(&s)
                            .map(|p| p.display().to_string())
                            .unwrap_or(s);
                        ContextModel::String(resolved)
                    })
                    .collect();
                Ok(ContextModel::List(resolved))
            }
            _ => anyhow::bail!("Modifier 'resolve' expects a string or list of paths"),
        }
    }
}
