//! System provider for `system_language` — system language code from `LANG` (e.g. `en`).

use crate::system::system::SystemKey;
use crate::system::system::SystemProvider;
use anyhow::Result;

pub struct SystemLanguageProvider;

impl SystemProvider for SystemLanguageProvider {
    fn key(&self) -> SystemKey {
        SystemKey::Language
    }

    fn resolve(&self) -> Result<String> {
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

        Ok(lang.to_string())
    }
}
