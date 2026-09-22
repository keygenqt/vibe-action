//! System provider for `system_dir_cache` — user cache directory.

use crate::system::system::SystemKey;
use crate::system::system::SystemProvider;
use anyhow::Result;

pub struct SystemDirCacheProvider;

impl SystemProvider for SystemDirCacheProvider {
    fn key(&self) -> SystemKey {
        SystemKey::DirCache
    }

    fn resolve(&self) -> Result<String> {
        Ok(dirs::cache_dir()
            .map(|p| p.display().to_string())
            .unwrap_or_default())
    }
}
