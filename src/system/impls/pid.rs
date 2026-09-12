//! System provider for `{system_pid}` — current process ID.

use crate::system::system::SystemKey;
use crate::system::system::SystemProvider;
use anyhow::Result;

pub struct SystemPidProvider;

impl SystemProvider for SystemPidProvider {
    fn key(&self) -> SystemKey {
        SystemKey::Pid
    }

    fn resolve(&self) -> Result<String> {
        Ok(std::process::id().to_string())
    }
}
