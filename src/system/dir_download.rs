//! System provider for `{system_dir_download}` — user downloads directory.

use crate::models::context::ContextModel;
use crate::system::system::SystemKey;
use crate::system::system::SystemProvider;
use anyhow::Result;

pub struct SystemDirDownloadProvider;

impl SystemProvider for SystemDirDownloadProvider {
    fn key(&self) -> SystemKey {
        SystemKey::DirDownload
    }

    fn resolve(&self) -> Result<ContextModel> {
        let dir = dirs::download_dir()
            .map(|p| p.display().to_string())
            .unwrap_or_default();
        Ok(ContextModel::String(dir))
    }
}
