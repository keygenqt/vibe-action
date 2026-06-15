//! Modifier trait, registry, and application logic.
//! Each modifier transforms a ContextModel value via the pipe syntax: {tag|modifier:arg}

use crate::models::context::ContextModel;
use anyhow::Result;
use std::collections::HashMap;

/// Enum of all available modifier keys.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ModifierKey {
    Join,
    Trim,
    Upper,
    Lower,
}

impl ModifierKey {
    /// Convert from string, returns None if unknown.
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "join" => Some(Self::Join),
            "trim" => Some(Self::Trim),
            "upper" => Some(Self::Upper),
            "lower" => Some(Self::Lower),
            _ => None,
        }
    }
}

/// Trait for pipe modifiers.
pub trait Modifier: Send + Sync {
    fn key(&self) -> ModifierKey;
    fn apply(&self, value: &ContextModel, arg: &str) -> Result<ContextModel>;
}

/// Registry of all modifiers.
pub struct ModifierRegistry {
    modifiers: HashMap<ModifierKey, Box<dyn Modifier>>,
}

impl ModifierRegistry {
    /// Create a new registry with all built-in modifiers.
    pub fn new() -> Self {
        let mut registry = Self {
            modifiers: HashMap::new(),
        };
        registry.register(Box::new(super::join::JoinModifier));
        registry.register(Box::new(super::trim::TrimModifier));
        registry.register(Box::new(super::upper::UpperModifier));
        registry.register(Box::new(super::lower::LowerModifier));
        registry
    }

    /// Register a new modifier.
    pub fn register(&mut self, modifier: Box<dyn Modifier>) {
        self.modifiers.insert(modifier.key(), modifier);
    }

    /// Get a modifier by its key.
    pub fn get(&self, key: ModifierKey) -> Option<&dyn Modifier> {
        self.modifiers.get(&key).map(|m| m.as_ref())
    }

    /// Apply a chain of pipe modifiers to a value.
    /// Format: "mod1:arg1|mod2:arg2|mod3"
    pub fn apply_modifier(&self, modifier: &str, value: &ContextModel) -> Result<ContextModel> {
        if modifier.is_empty() {
            return Ok(value.clone());
        }
        let mut current = value.clone();
        for part in modifier.split('|') {
            let (name, arg) = part.split_once(':').unwrap_or((part, ""));
            match ModifierKey::from_str(name) {
                Some(key) => {
                    if let Some(modifier) = self.get(key) {
                        current = modifier.apply(&current, arg)?;
                    }
                }
                None => {}
            }
        }
        Ok(current)
    }
}
