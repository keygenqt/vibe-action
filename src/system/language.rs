//! System provider for `{system_language}` — system language from LANG env.

use crate::models::context::ContextModel;
use crate::system::system::{SystemKey, SystemProvider};
use anyhow::Result;

pub struct SystemLanguageProvider;

impl SystemProvider for SystemLanguageProvider {
    fn key(&self) -> SystemKey {
        SystemKey::Language
    }

    fn resolve(&self) -> Result<ContextModel> {
        let raw = std::env::var("LANG").unwrap_or_default();
        let lang = if raw.is_empty() || raw.starts_with("C.") || raw == "C" || raw == "POSIX" {
            "en"
        } else {
            raw.split('.')
                .next()
                .unwrap_or("en")
                .split('_')
                .next()
                .unwrap_or("en")
        };

        Ok(ContextModel::String(lang.to_string()))
    }
}
