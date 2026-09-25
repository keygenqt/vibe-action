//! System provider for `system_user` — current user name.

use crate::system::system::SystemKey;
use crate::system::system::SystemProvider;
use anyhow::Result;

pub struct SystemUserProvider;

impl SystemProvider for SystemUserProvider {
    fn key(&self) -> SystemKey {
        SystemKey::User
    }

    fn resolve(&self) -> Result<String> {
        Ok(std::env::var("USER").unwrap_or_default())
    }
}
