//! System provider for `{system_pwd}` — current working directory.

use crate::models::context::ContextModel;
use crate::system::system::{SystemKey, SystemProvider};
use anyhow::Result;

pub struct SystemPwdProvider;

impl SystemProvider for SystemPwdProvider {
    fn key(&self) -> SystemKey {
        SystemKey::Pwd
    }

    fn resolve(&self) -> Result<ContextModel> {
        let pwd = std::env::current_dir()
            .map(|p| p.display().to_string())
            .unwrap_or_default();
        Ok(ContextModel::String(pwd))
    }
}
