//! System provider trait and registry.
//! Each provider resolves a `{system_*}` tag to a runtime value.

use crate::models::context::ContextModel;
use anyhow::Result;
use std::collections::HashMap;

/// Enum of all known system tag keys.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SystemKey {
    Arch,
    Clipboard,
    ClipboardImage,
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
            SystemKey::Clipboard => "system_clipboard",
            SystemKey::ClipboardImage => "system_clipboard_image",
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
            "system_clipboard" => Some(SystemKey::Clipboard),
            "system_clipboard_image" => Some(SystemKey::ClipboardImage),
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
            SystemKey::Clipboard,
            SystemKey::ClipboardImage,
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
    fn resolve(&self) -> Result<ContextModel>;
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
        registry.register(Box::new(super::arch::SystemArchProvider));
        registry.register(Box::new(super::clipboard::SystemClipboardProvider));
        registry.register(Box::new(
            super::clipboard_image::SystemClipboardImageProvider,
        ));
        registry.register(Box::new(super::date::SystemDateProvider));
        registry.register(Box::new(super::dir_download::SystemDirDownloadProvider));
        registry.register(Box::new(super::dir_home::SystemDirHomeProvider));
        registry.register(Box::new(super::dir_pwd::SystemDirPwdProvider));
        registry.register(Box::new(super::dir_temp::SystemDirTempProvider));
        registry.register(Box::new(super::hostname::SystemHostnameProvider));
        registry.register(Box::new(super::language::SystemLanguageProvider));
        registry.register(Box::new(super::os::SystemOsProvider));
        registry.register(Box::new(super::pid::SystemPidProvider));
        registry.register(Box::new(super::shell::SystemShellProvider));
        registry.register(Box::new(super::time::SystemTimeProvider));
        registry.register(Box::new(super::user::SystemUserProvider));
        registry
    }

    /// Register a new provider.
    pub fn register(&mut self, provider: Box<dyn SystemProvider>) {
        self.providers.insert(provider.key(), provider);
    }

    /// Resolve a system tag by its key.
    pub fn resolve(&self, key: SystemKey) -> Result<ContextModel> {
        match self.providers.get(&key) {
            Some(provider) => provider.resolve(),
            None => anyhow::bail!("Unknown system tag: '{}'", key.as_str()),
        }
    }
}
