//! System provider for `{system_hostname}` — machine hostname.

use crate::system::system::SystemKey;
use crate::system::system::SystemProvider;
use anyhow::Result;

pub struct SystemHostnameProvider;

impl SystemProvider for SystemHostnameProvider {
    fn key(&self) -> SystemKey {
        SystemKey::Hostname
    }

    fn resolve(&self) -> Result<String> {
        Ok(hostname::get()
            .map(|h| h.to_string_lossy().to_string())
            .unwrap_or_default())
    }
}
