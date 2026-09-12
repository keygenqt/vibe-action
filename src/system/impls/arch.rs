//! System provider for `{system_arch}` — CPU architecture.

use crate::system::system::SystemKey;
use crate::system::system::SystemProvider;
use anyhow::Result;

pub struct SystemArchProvider;

impl SystemProvider for SystemArchProvider {
    fn key(&self) -> SystemKey {
        SystemKey::Arch
    }

    fn resolve(&self) -> Result<String> {
        Ok(std::env::consts::ARCH.to_string())
    }
}
