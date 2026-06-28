//! System provider for `{system_hostname}` — machine hostname.

use crate::models::context::ContextModel;
use crate::system::system::{SystemKey, SystemProvider};
use anyhow::Result;

pub struct SystemHostnameProvider;

impl SystemProvider for SystemHostnameProvider {
    fn key(&self) -> SystemKey {
        SystemKey::Hostname
    }

    fn resolve(&self) -> Result<ContextModel> {
        let hostname = hostname::get()
            .map(|h| h.to_string_lossy().to_string())
            .unwrap_or_default();
        Ok(ContextModel::String(hostname))
    }
}
