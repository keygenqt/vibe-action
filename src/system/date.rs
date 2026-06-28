//! System provider for `{system_date}` — current date in ISO 8601 format.

use crate::models::context::ContextModel;
use crate::system::system::{SystemKey, SystemProvider};
use anyhow::Result;

pub struct SystemDateProvider;

impl SystemProvider for SystemDateProvider {
    fn key(&self) -> SystemKey {
        SystemKey::Date
    }

    fn resolve(&self) -> Result<ContextModel> {
        let date = chrono::Local::now().format("%Y-%m-%d").to_string();
        Ok(ContextModel::String(date))
    }
}
