//! System provider for `{system_temp}` — temporary directory.

use crate::models::context::ContextModel;
use crate::system::system::{SystemKey, SystemProvider};
use anyhow::Result;

pub struct SystemTempProvider;

impl SystemProvider for SystemTempProvider {
    fn key(&self) -> SystemKey {
        SystemKey::Temp
    }

    fn resolve(&self) -> Result<ContextModel> {
        let temp = std::env::temp_dir().display().to_string();
        Ok(ContextModel::String(temp))
    }
}
