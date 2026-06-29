//! Modifier trait, registry, and application logic.
//! Each modifier transforms a ContextModel value via the pipe syntax: {tag|modifier:arg}

use crate::models::context::ContextModel;
use anyhow::Result;
use std::collections::HashMap;

/// Enum of all available modifier keys.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ModifierKey {
    Ast,
    Clipboard,
    Contains,
    Empty,
    Equals,
    Format,
    IsDir,
    IsFile,
    Join,
    Load,
    Lower,
    Resolve,
    Reverse,
    Scan,
    Size,
    Sort,
    Split,
    Take,
    Text,
    Trim,
    Uniq,
    Upper,
}

impl ModifierKey {
    /// Convert from string, returns None if unknown.
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "ast" => Some(Self::Ast),
            "clipboard" => Some(Self::Clipboard),
            "contains" => Some(Self::Contains),
            "empty" => Some(Self::Empty),
            "equals" => Some(Self::Equals),
            "format" => Some(Self::Format),
            "is_dir" => Some(Self::IsDir),
            "is_file" => Some(Self::IsFile),
            "join" => Some(Self::Join),
            "load" => Some(Self::Load),
            "lower" => Some(Self::Lower),
            "resolve" => Some(Self::Resolve),
            "reverse" => Some(Self::Reverse),
            "scan" => Some(Self::Scan),
            "size" => Some(Self::Size),
            "sort" => Some(Self::Sort),
            "split" => Some(Self::Split),
            "take" => Some(Self::Take),
            "text" => Some(Self::Text),
            "trim" => Some(Self::Trim),
            "uniq" => Some(Self::Uniq),
            "upper" => Some(Self::Upper),
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
        registry.register(Box::new(super::ast::AstModifier));
        registry.register(Box::new(super::clipboard::ClipboardModifier));
        registry.register(Box::new(super::contains::ContainsModifier));
        registry.register(Box::new(super::empty::EmptyModifier));
        registry.register(Box::new(super::equals::EqualsModifier));
        registry.register(Box::new(super::format::FormatModifier));
        registry.register(Box::new(super::is_dir::IsDirModifier));
        registry.register(Box::new(super::is_file::IsFileModifier));
        registry.register(Box::new(super::join::JoinModifier));
        registry.register(Box::new(super::load::LoadModifier));
        registry.register(Box::new(super::lower::LowerModifier));
        registry.register(Box::new(super::resolve::ResolveModifier));
        registry.register(Box::new(super::reverse::ReverseModifier));
        registry.register(Box::new(super::scan::ScanModifier));
        registry.register(Box::new(super::size::SizeModifier));
        registry.register(Box::new(super::sort::SortModifier));
        registry.register(Box::new(super::split::SplitModifier));
        registry.register(Box::new(super::take::TakeModifier));
        registry.register(Box::new(super::text::TextModifier));
        registry.register(Box::new(super::trim::TrimModifier));
        registry.register(Box::new(super::uniq::UniqModifier));
        registry.register(Box::new(super::upper::UpperModifier));
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

    /// Apply a pre-parsed chain of pipe modifiers to a context value in chronological order.
    pub fn apply_modifier(
        &self,
        modifiers: &[crate::engine::parser::ModifierMatch],
        value: &ContextModel,
    ) -> Result<ContextModel> {
        let mut current = value.clone();

        for mat in modifiers {
            if let Some(key) = ModifierKey::from_str(&mat.name) {
                if let Some(modifier) = self.get(key) {
                    // Extract the safe isolated argument slice or fallback to an empty string contract
                    let arg = mat.argument.as_deref().unwrap_or("");
                    current = modifier.apply(&current, arg)?;
                }
            } else {
                anyhow::bail!(
                    "Unknown modifier key '{}' invoked in execution pipeline",
                    mat.name
                );
            }
        }

        Ok(current)
    }
}

/// Invert a ContextModel::String ("true"/"false") or ContextModel::List of such strings.
pub fn invert(value: ContextModel) -> ContextModel {
    match value {
        ContextModel::String(s) => {
            let inverted = if s == "true" {
                "false"
            } else if s == "false" {
                "true"
            } else {
                return ContextModel::String(s); // не bool-строка — не трогаем
            };
            ContextModel::String(inverted.to_string())
        }
        ContextModel::List(items) => {
            let inverted: Vec<String> = items
                .iter()
                .map(|i| {
                    if i == "true" {
                        "false".to_string()
                    } else if i == "false" {
                        "true".to_string()
                    } else {
                        i.clone()
                    }
                })
                .collect();
            ContextModel::List(inverted)
        }
    }
}
