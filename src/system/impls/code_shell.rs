//! System provider for `system_code_shell` — extensions of supported
//! shell languages (single source of truth: `crate::langs`).

use crate::system::system::SystemKey;
use crate::system::system::SystemProvider;
use crate::utils;
use anyhow::Result;

pub struct SystemCodeShellProvider;

impl SystemProvider for SystemCodeShellProvider {
    fn key(&self) -> SystemKey {
        SystemKey::CodeShell
    }

    fn resolve(&self) -> Result<String> {
        Ok(utils::langs::CODE_SHELL
            .iter()
            .map(|(e, _)| *e)
            .collect::<Vec<_>>()
            .join("\n"))
    }
}
