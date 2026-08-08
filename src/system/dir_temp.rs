//! System provider for `{system_temp}` — temporary directory.

use crate::models::context::ContextModel;
use crate::system::system::SystemKey;
use crate::system::system::SystemProvider;
use anyhow::Result;

pub struct SystemDirTempProvider;

impl SystemProvider for SystemDirTempProvider {
    fn key(&self) -> SystemKey {
        SystemKey::DirTemp
    }

    fn resolve(&self) -> Result<ContextModel> {
        let temp = std::env::temp_dir().display().to_string();
        Ok(ContextModel::String(temp))
    }
}
