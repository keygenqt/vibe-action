//! System provider for `system_code_langs` — extensions of supported
//! code languages (single source of truth: `crate::langs`).

use crate::system::system::SystemKey;
use crate::system::system::SystemProvider;
use crate::utils;
use anyhow::Result;

pub struct SystemCodeLangsProvider;

impl SystemProvider for SystemCodeLangsProvider {
    fn key(&self) -> SystemKey {
        SystemKey::CodeLangs
    }

    fn resolve(&self) -> Result<String> {
        Ok(utils::langs::CODE_LANGS
            .iter()
            .map(|(e, _)| *e)
            .collect::<Vec<_>>()
            .join("\n"))
    }
}
