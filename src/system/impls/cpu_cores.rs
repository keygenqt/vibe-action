//! System provider for `system_cpu_cores` — logical CPU core count.

use crate::system::system::SystemKey;
use crate::system::system::SystemProvider;
use anyhow::Result;

pub struct SystemCpuCoresProvider;

impl SystemProvider for SystemCpuCoresProvider {
    fn key(&self) -> SystemKey {
        SystemKey::CpuCores
    }

    fn resolve(&self) -> Result<String> {
        Ok(std::thread::available_parallelism()
            .map(|n| n.get().to_string())
            .unwrap_or_default())
    }
}
