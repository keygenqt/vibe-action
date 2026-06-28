//! System provider for `{system_arch}` — CPU architecture.

use crate::models::context::ContextModel;
use crate::system::system::{SystemKey, SystemProvider};
use anyhow::Result;

pub struct SystemArchProvider;

impl SystemProvider for SystemArchProvider {
    fn key(&self) -> SystemKey {
        SystemKey::Arch
    }

    fn resolve(&self) -> Result<ContextModel> {
        Ok(ContextModel::String(std::env::consts::ARCH.to_string()))
    }
}
