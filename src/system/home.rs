//! System provider for `{system_home}` — user home directory.

use crate::models::context::ContextModel;
use crate::system::system::{SystemKey, SystemProvider};
use anyhow::Result;

pub struct SystemHomeProvider;

impl SystemProvider for SystemHomeProvider {
    fn key(&self) -> SystemKey {
        SystemKey::Home
    }

    fn resolve(&self) -> Result<ContextModel> {
        let home = std::env::var("HOME").unwrap_or_default();
        Ok(ContextModel::String(home))
    }
}
