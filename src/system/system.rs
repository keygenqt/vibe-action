//! System provider trait and registry.
//! Each provider resolves a `{system_*}` tag to a runtime value.

use crate::models::context::ContextModel;
use anyhow::Result;
use std::collections::HashMap;

/// Enum of all known system tag keys.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SystemKey {
    Clipboard,
    ClipboardImage,
    Date,
    Home,
    Language,
    Os,
    Pid,
    Pwd,
    Temp,
    Time,
    User,
}

impl SystemKey {
    /// Tag name without braces.
    pub fn as_str(&self) -> &'static str {
        match self {
            SystemKey::Clipboard => "system_clipboard",
            SystemKey::ClipboardImage => "system_clipboard_image",
            SystemKey::Date => "system_date",
            SystemKey::Home => "system_home",
            SystemKey::Language => "system_language",
            SystemKey::Os => "system_os",
            SystemKey::Pid => "system_pid",
            SystemKey::Pwd => "system_pwd",
            SystemKey::Temp => "system_temp",
            SystemKey::Time => "system_time",
            SystemKey::User => "system_user",
        }
    }

    /// Parse from a tag string like "system_clipboard".
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "system_clipboard" => Some(SystemKey::Clipboard),
            "system_clipboard_image" => Some(SystemKey::ClipboardImage),
            "system_date" => Some(SystemKey::Date),
            "system_home" => Some(SystemKey::Home),
            "system_language" => Some(SystemKey::Language),
            "system_os" => Some(SystemKey::Os),
            "system_pid" => Some(SystemKey::Pid),
            "system_pwd" => Some(SystemKey::Pwd),
            "system_temp" => Some(SystemKey::Temp),
            "system_time" => Some(SystemKey::Time),
            "system_user" => Some(SystemKey::User),
            _ => None,
        }
    }

    /// All known keys.
    pub fn all() -> &'static [SystemKey] {
        &[
            SystemKey::Clipboard,
            SystemKey::ClipboardImage,
            SystemKey::Date,
            SystemKey::Home,
            SystemKey::Language,
            SystemKey::Os,
            SystemKey::Pid,
            SystemKey::Pwd,
            SystemKey::Temp,
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
        registry.register(Box::new(super::clipboard::SystemClipboardProvider));
        registry.register(Box::new(
            super::clipboard_image::SystemClipboardImageProvider,
        ));
        registry.register(Box::new(super::date::SystemDateProvider));
        registry.register(Box::new(super::home::SystemHomeProvider));
        registry.register(Box::new(super::language::SystemLanguageProvider));
        registry.register(Box::new(super::os::SystemOsProvider));
        registry.register(Box::new(super::pid::SystemPidProvider));
        registry.register(Box::new(super::pwd::SystemPwdProvider));
        registry.register(Box::new(super::temp::SystemTempProvider));
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
