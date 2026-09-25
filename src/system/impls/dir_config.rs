//! System provider for `system_dir_config` — user configuration directory.

use crate::system::system::SystemKey;
use crate::system::system::SystemProvider;
use anyhow::Result;

pub struct SystemDirConfigProvider;

impl SystemProvider for SystemDirConfigProvider {
    fn key(&self) -> SystemKey {
        SystemKey::DirConfig
    }

    fn resolve(&self) -> Result<String> {
        Ok(dirs::config_dir()
            .map(|p| p.display().to_string())
            .unwrap_or_default())
    }
}
