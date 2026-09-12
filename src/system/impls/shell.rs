//! System provider for `{system_shell}` — current shell from SHELL env.

use crate::system::system::SystemKey;
use crate::system::system::SystemProvider;
use anyhow::Result;

pub struct SystemShellProvider;

impl SystemProvider for SystemShellProvider {
    fn key(&self) -> SystemKey {
        SystemKey::Shell
    }

    fn resolve(&self) -> Result<String> {
        let shell = std::env::var("SHELL")
            .unwrap_or_default()
            .split('/')
            .last()
            .unwrap_or("unknown")
            .to_string();
        Ok(shell)
    }
}
