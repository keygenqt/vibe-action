//! System provider for `{system_shell}` — current shell from SHELL env.

use crate::models::context::ContextModel;
use crate::system::system::{SystemKey, SystemProvider};
use anyhow::Result;

pub struct SystemShellProvider;

impl SystemProvider for SystemShellProvider {
    fn key(&self) -> SystemKey {
        SystemKey::Shell
    }

    fn resolve(&self) -> Result<ContextModel> {
        let shell = std::env::var("SHELL")
            .unwrap_or_default()
            .split('/')
            .last()
            .unwrap_or("unknown")
            .to_string();
        Ok(ContextModel::String(shell))
    }
}
