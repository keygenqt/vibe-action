//! System key enum, provider trait, and registry.
//! See [`crate::system`] module-level docs for the provider contract and tag list.

use anyhow::Result;
use std::collections::HashMap;

use crate::system::impls::arch::SystemArchProvider;
use crate::system::impls::cpu_cores::SystemCpuCoresProvider;
use crate::system::impls::date::SystemDateProvider;
use crate::system::impls::datetime::SystemDatetimeProvider;
use crate::system::impls::dir_cache::SystemDirCacheProvider;
use crate::system::impls::dir_config::SystemDirConfigProvider;
use crate::system::impls::dir_data::SystemDirDataProvider;
use crate::system::impls::dir_download::SystemDirDownloadProvider;
use crate::system::impls::dir_home::SystemDirHomeProvider;
use crate::system::impls::dir_pwd::SystemDirPwdProvider;
use crate::system::impls::dir_temp::SystemDirTempProvider;
use crate::system::impls::hostname::SystemHostnameProvider;
use crate::system::impls::language::SystemLanguageProvider;
use crate::system::impls::mem_available::SystemMemAvailableProvider;
use crate::system::impls::os::SystemOsProvider;
use crate::system::impls::pid::SystemPidProvider;
use crate::system::impls::shell::SystemShellProvider;
use crate::system::impls::time::SystemTimeProvider;
use crate::system::impls::timestamp::SystemTimestampProvider;
use crate::system::impls::uid::SystemUidProvider;
use crate::system::impls::user::SystemUserProvider;

/// Enum of all known system tag keys.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SystemKey {
    Arch,
    CpuCores,
    Date,
    Datetime,
    DirCache,
    DirConfig,
    DirData,
    DirDownload,
    DirHome,
    DirPwd,
    DirTemp,
    Hostname,
    Language,
    MemAvailable,
    Os,
    Pid,
    Shell,
    Time,
    Timestamp,
    Uid,
    User,
}

impl SystemKey {
    pub fn as_str(&self) -> &'static str {
        match self {
            SystemKey::Arch => "system_arch",
            SystemKey::CpuCores => "system_cpu_cores",
            SystemKey::Date => "system_date",
            SystemKey::Datetime => "system_datetime",
            SystemKey::DirCache => "system_dir_cache",
            SystemKey::DirConfig => "system_dir_config",
            SystemKey::DirData => "system_dir_data",
            SystemKey::DirDownload => "system_dir_download",
            SystemKey::DirHome => "system_dir_home",
            SystemKey::DirPwd => "system_dir_pwd",
            SystemKey::DirTemp => "system_dir_temp",
            SystemKey::Hostname => "system_hostname",
            SystemKey::Language => "system_language",
            SystemKey::MemAvailable => "system_mem_available",
            SystemKey::Os => "system_os",
            SystemKey::Pid => "system_pid",
            SystemKey::Shell => "system_shell",
            SystemKey::Time => "system_time",
            SystemKey::Timestamp => "system_timestamp",
            SystemKey::Uid => "system_uid",
            SystemKey::User => "system_user",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "system_arch" => Some(SystemKey::Arch),
            "system_cpu_cores" => Some(SystemKey::CpuCores),
            "system_date" => Some(SystemKey::Date),
            "system_datetime" => Some(SystemKey::Datetime),
            "system_dir_cache" => Some(SystemKey::DirCache),
            "system_dir_config" => Some(SystemKey::DirConfig),
            "system_dir_data" => Some(SystemKey::DirData),
            "system_dir_download" => Some(SystemKey::DirDownload),
            "system_dir_home" => Some(SystemKey::DirHome),
            "system_dir_pwd" => Some(SystemKey::DirPwd),
            "system_dir_temp" => Some(SystemKey::DirTemp),
            "system_hostname" => Some(SystemKey::Hostname),
            "system_language" => Some(SystemKey::Language),
            "system_mem_available" => Some(SystemKey::MemAvailable),
            "system_os" => Some(SystemKey::Os),
            "system_pid" => Some(SystemKey::Pid),
            "system_shell" => Some(SystemKey::Shell),
            "system_time" => Some(SystemKey::Time),
            "system_timestamp" => Some(SystemKey::Timestamp),
            "system_uid" => Some(SystemKey::Uid),
            "system_user" => Some(SystemKey::User),
            _ => None,
        }
    }

    pub fn all() -> &'static [SystemKey] {
        &[
            SystemKey::Arch,
            SystemKey::CpuCores,
            SystemKey::Date,
            SystemKey::Datetime,
            SystemKey::DirCache,
            SystemKey::DirConfig,
            SystemKey::DirData,
            SystemKey::DirDownload,
            SystemKey::DirHome,
            SystemKey::DirPwd,
            SystemKey::DirTemp,
            SystemKey::Hostname,
            SystemKey::Language,
            SystemKey::MemAvailable,
            SystemKey::Os,
            SystemKey::Pid,
            SystemKey::Shell,
            SystemKey::Time,
            SystemKey::Timestamp,
            SystemKey::Uid,
            SystemKey::User,
        ]
    }
}

/// Trait for system tag providers.
pub trait SystemProvider: Send + Sync {
    fn key(&self) -> SystemKey;
    fn resolve(&self) -> Result<String>;
}

/// Registry of all system providers.
pub struct SystemRegistry {
    providers: HashMap<SystemKey, Box<dyn SystemProvider>>,
}

impl SystemRegistry {
    /// Create a new registry with all built-in providers.
    pub fn new() -> Self {
        let mut registry = Self {
            providers: HashMap::new(),
        };
        registry.register(Box::new(SystemArchProvider));
        registry.register(Box::new(SystemCpuCoresProvider));
        registry.register(Box::new(SystemDateProvider));
        registry.register(Box::new(SystemDatetimeProvider));
        registry.register(Box::new(SystemDirCacheProvider));
        registry.register(Box::new(SystemDirConfigProvider));
        registry.register(Box::new(SystemDirDataProvider));
        registry.register(Box::new(SystemDirDownloadProvider));
        registry.register(Box::new(SystemDirHomeProvider));
        registry.register(Box::new(SystemDirPwdProvider));
        registry.register(Box::new(SystemDirTempProvider));
        registry.register(Box::new(SystemHostnameProvider));
        registry.register(Box::new(SystemLanguageProvider));
        registry.register(Box::new(SystemMemAvailableProvider));
        registry.register(Box::new(SystemOsProvider));
        registry.register(Box::new(SystemPidProvider));
        registry.register(Box::new(SystemShellProvider));
        registry.register(Box::new(SystemTimeProvider));
        registry.register(Box::new(SystemTimestampProvider));
        registry.register(Box::new(SystemUidProvider));
        registry.register(Box::new(SystemUserProvider));
        registry
    }

    /// Register a new provider.
    pub fn register(&mut self, provider: Box<dyn SystemProvider>) {
        self.providers.insert(provider.key(), provider);
    }

    /// Resolve a system tag by its key.
    pub fn resolve(&self, key: SystemKey) -> Result<String> {
        match self.providers.get(&key) {
            Some(provider) => provider.resolve(),
            None => anyhow::bail!("Unknown system tag: '{}'", key.as_str()),
        }
    }
}
