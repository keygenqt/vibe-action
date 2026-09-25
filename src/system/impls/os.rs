//! System provider for `system_os` — operating system name.

use crate::system::system::SystemKey;
use crate::system::system::SystemProvider;
use anyhow::Result;

pub struct SystemOsProvider;

impl SystemProvider for SystemOsProvider {
    fn key(&self) -> SystemKey {
        SystemKey::Os
    }

    fn resolve(&self) -> Result<String> {
        Ok(std::env::consts::OS.to_string())
    }
}
