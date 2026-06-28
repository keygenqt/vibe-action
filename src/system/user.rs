//! System provider for `{system_user}` — current user name.

use crate::models::context::ContextModel;
use crate::system::system::{SystemKey, SystemProvider};
use anyhow::Result;

pub struct SystemUserProvider;

impl SystemProvider for SystemUserProvider {
    fn key(&self) -> SystemKey {
        SystemKey::User
    }

    fn resolve(&self) -> Result<ContextModel> {
        let user = std::env::var("USER").unwrap_or_default();
        Ok(ContextModel::String(user))
    }
}
