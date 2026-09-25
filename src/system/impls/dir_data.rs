//! System provider for `system_dir_data` — user data directory.

use crate::system::system::SystemKey;
use crate::system::system::SystemProvider;
use anyhow::Result;

pub struct SystemDirDataProvider;

impl SystemProvider for SystemDirDataProvider {
    fn key(&self) -> SystemKey {
        SystemKey::DirData
    }

    fn resolve(&self) -> Result<String> {
        Ok(dirs::data_dir()
            .map(|p| p.display().to_string())
            .unwrap_or_default())
    }
}
