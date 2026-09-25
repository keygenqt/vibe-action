//! System provider for `system_uid` — current user ID.

use crate::system::system::SystemKey;
use crate::system::system::SystemProvider;
use anyhow::Result;

pub struct SystemUidProvider;

impl SystemProvider for SystemUidProvider {
    fn key(&self) -> SystemKey {
        SystemKey::Uid
    }

    fn resolve(&self) -> Result<String> {
        let uid = unsafe { libc::getuid() };
        Ok(uid.to_string())
    }
}
