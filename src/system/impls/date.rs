//! System provider for `{system_date}` — current date in ISO 8601 format.

use crate::system::system::SystemKey;
use crate::system::system::SystemProvider;
use anyhow::Result;

pub struct SystemDateProvider;

impl SystemProvider for SystemDateProvider {
    fn key(&self) -> SystemKey {
        SystemKey::Date
    }

    fn resolve(&self) -> Result<String> {
        Ok(chrono::Local::now().format("%Y-%m-%d").to_string())
    }
}
