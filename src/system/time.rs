//! System provider for `{system_time}` — current time.

use crate::system::system::SystemKey;
use crate::system::system::SystemProvider;
use anyhow::Result;

pub struct SystemTimeProvider;

impl SystemProvider for SystemTimeProvider {
    fn key(&self) -> SystemKey {
        SystemKey::Time
    }

    fn resolve(&self) -> Result<String> {
        Ok(chrono::Local::now().format("%H:%M:%S").to_string())
    }
}
