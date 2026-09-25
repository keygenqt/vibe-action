//! System provider for `system_dir_temp` — temporary directory.

use crate::system::system::SystemKey;
use crate::system::system::SystemProvider;
use anyhow::Result;

pub struct SystemDirTempProvider;

impl SystemProvider for SystemDirTempProvider {
    fn key(&self) -> SystemKey {
        SystemKey::DirTemp
    }

    fn resolve(&self) -> Result<String> {
        Ok(std::env::temp_dir().display().to_string())
    }
}
