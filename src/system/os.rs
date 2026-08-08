//! System provider for `{system_os}` — operating system name.

use crate::models::context::ContextModel;
use crate::system::system::SystemKey;
use crate::system::system::SystemProvider;
use anyhow::Result;

pub struct SystemOsProvider;

impl SystemProvider for SystemOsProvider {
    fn key(&self) -> SystemKey {
        SystemKey::Os
    }

    fn resolve(&self) -> Result<ContextModel> {
        Ok(ContextModel::String(std::env::consts::OS.to_string()))
    }
}
