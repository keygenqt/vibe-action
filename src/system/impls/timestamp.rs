//! System provider for `system_timestamp` — current unix epoch seconds.

use crate::system::system::SystemKey;
use crate::system::system::SystemProvider;
use anyhow::Result;

pub struct SystemTimestampProvider;

impl SystemProvider for SystemTimestampProvider {
    fn key(&self) -> SystemKey {
        SystemKey::Timestamp
    }

    fn resolve(&self) -> Result<String> {
        Ok(chrono::Local::now().timestamp().to_string())
    }
}
