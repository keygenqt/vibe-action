//! System provider for `system_datetime` — current date and time, ISO 8601 (`YYYY-MM-DDTHH:MM:SS`).

use crate::system::system::SystemKey;
use crate::system::system::SystemProvider;
use anyhow::Result;

pub struct SystemDatetimeProvider;

impl SystemProvider for SystemDatetimeProvider {
    fn key(&self) -> SystemKey {
        SystemKey::Datetime
    }

    fn resolve(&self) -> Result<String> {
        Ok(chrono::Local::now().format("%Y-%m-%dT%H:%M:%S").to_string())
    }
}
