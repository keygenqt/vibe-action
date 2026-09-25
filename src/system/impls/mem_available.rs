//! System provider for `system_mem_available` — available memory in bytes.

use crate::system::system::SystemKey;
use crate::system::system::SystemProvider;
use anyhow::Result;

pub struct SystemMemAvailableProvider;

impl SystemProvider for SystemMemAvailableProvider {
    fn key(&self) -> SystemKey {
        SystemKey::MemAvailable
    }

    fn resolve(&self) -> Result<String> {
        let mut sys = sysinfo::System::new();
        sys.refresh_memory();
        Ok(sys.available_memory().to_string())
    }
}
