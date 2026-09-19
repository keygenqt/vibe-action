//! System provider trait and registry.
//! Each provider resolves a `system_*` tag.
//!
//! # Provider contract
//!
//! All `system_*` providers read runtime/environment state — no input,
//! no validation. Always return `Ok(String)`; missing value → `""`,
//! never `Err`.
//!
//! - `system_arch` — CPU architecture.
//! - `system_date` — current date, ISO 8601 (`YYYY-MM-DD`).
//! - `system_dir_download` — user downloads directory.
//! - `system_dir_home` — user home directory.
//! - `system_dir_pwd` — current working directory.
//! - `system_dir_temp` — temporary directory.
//! - `system_hostname` — machine hostname.
//! - `system_language` — system language code from `LANG` (e.g. `en`).
//! - `system_os` — operating system name.
//! - `system_pid` — current process ID.
//! - `system_shell` — current shell name from `SHELL` (basename).
//! - `system_time` — current time (`HH:MM:SS`).
//! - `system_user` — current user name.

use anyhow::Result;
use std::collections::HashMap;

use crate::system::impls::arch::SystemArchProvider;
use crate::system::impls::date::SystemDateProvider;
use crate::system::impls::dir_download::SystemDirDownloadProvider;
use crate::system::impls::dir_home::SystemDirHomeProvider;
use crate::system::impls::dir_pwd::SystemDirPwdProvider;
use crate::system::impls::dir_temp::SystemDirTempProvider;
use crate::system::impls::hostname::SystemHostnameProvider;
use crate::system::impls::language::SystemLanguageProvider;
use crate::system::impls::os::SystemOsProvider;
use crate::system::impls::pid::SystemPidProvider;
use crate::system::impls::shell::SystemShellProvider;
use crate::system::impls::time::SystemTimeProvider;
use crate::system::impls::user::SystemUserProvider;

/// Enum of all known system tag keys.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SystemKey {
    Arch,
    Date,
    DirDownload,
    DirHome,
    DirPwd,
    DirTemp,
    Hostname,
    Language,
    Os,
    Pid,
    Shell,
    Time,
    User,
}

impl SystemKey {
    pub fn as_str(&self) -> &'static str {
        match self {
            SystemKey::Arch => "system_arch",
            SystemKey::Date => "system_date",
            SystemKey::DirDownload => "system_dir_download",
            SystemKey::DirHome => "system_dir_home",
            SystemKey::DirPwd => "system_dir_pwd",
            SystemKey::DirTemp => "system_dir_temp",
            SystemKey::Hostname => "system_hostname",
            SystemKey::Language => "system_language",
            SystemKey::Os => "system_os",
            SystemKey::Pid => "system_pid",
            SystemKey::Shell => "system_shell",
            SystemKey::Time => "system_time",
            SystemKey::User => "system_user",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "system_arch" => Some(SystemKey::Arch),
            "system_date" => Some(SystemKey::Date),
            "system_dir_download" => Some(SystemKey::DirDownload),
            "system_dir_home" => Some(SystemKey::DirHome),
            "system_dir_pwd" => Some(SystemKey::DirPwd),
            "system_dir_temp" => Some(SystemKey::DirTemp),
            "system_hostname" => Some(SystemKey::Hostname),
            "system_language" => Some(SystemKey::Language),
            "system_os" => Some(SystemKey::Os),
            "system_pid" => Some(SystemKey::Pid),
            "system_shell" => Some(SystemKey::Shell),
            "system_time" => Some(SystemKey::Time),
            "system_user" => Some(SystemKey::User),
            _ => None,
        }
    }

    pub fn all() -> &'static [SystemKey] {
        &[
            SystemKey::Arch,
            SystemKey::Date,
            SystemKey::DirDownload,
            SystemKey::DirHome,
            SystemKey::DirPwd,
            SystemKey::DirTemp,
            SystemKey::Hostname,
            SystemKey::Language,
            SystemKey::Os,
            SystemKey::Pid,
            SystemKey::Shell,
            SystemKey::Time,
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
        registry.register(Box::new(SystemDateProvider));
        registry.register(Box::new(SystemDirDownloadProvider));
        registry.register(Box::new(SystemDirHomeProvider));
        registry.register(Box::new(SystemDirPwdProvider));
        registry.register(Box::new(SystemDirTempProvider));
        registry.register(Box::new(SystemHostnameProvider));
        registry.register(Box::new(SystemLanguageProvider));
        registry.register(Box::new(SystemOsProvider));
        registry.register(Box::new(SystemPidProvider));
        registry.register(Box::new(SystemShellProvider));
        registry.register(Box::new(SystemTimeProvider));
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
