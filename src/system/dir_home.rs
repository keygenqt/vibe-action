//! System provider for `{system_home}` — user home directory.

use crate::models::context::ContextModel;
use crate::system::system::SystemKey;
use crate::system::system::SystemProvider;
use anyhow::Result;

pub struct SystemDirHomeProvider;

impl SystemProvider for SystemDirHomeProvider {
    fn key(&self) -> SystemKey {
        SystemKey::DirHome
    }

    fn resolve(&self) -> Result<ContextModel> {
        let home = std::env::var("HOME").unwrap_or_default();
        Ok(ContextModel::String(home))
    }
}
