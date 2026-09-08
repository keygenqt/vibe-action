//! System provider for `{system_pwd}` — current working directory.

use crate::system::system::SystemKey;
use crate::system::system::SystemProvider;
use anyhow::Result;

pub struct SystemDirPwdProvider;

impl SystemProvider for SystemDirPwdProvider {
    fn key(&self) -> SystemKey {
        SystemKey::DirPwd
    }

    fn resolve(&self) -> Result<String> {
        Ok(std::env::current_dir()
            .map(|p| p.display().to_string())
            .unwrap_or_default())
    }
}
