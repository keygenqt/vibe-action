//! Modifier trait, registry, and application logic.
//! Each modifier transforms a ContextModel value via the pipe syntax: {tag|modifier:arg}

use anyhow::Result;
use std::collections::HashMap;

/// Hidden delimiter separating array items within a string.
pub const ITEM_SEP: &str = "\x1F";

/// Enum of all available modifier keys.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ModifierKey {
    Ast,
    Clipboard,
    Contains,
    Empty,
    Equals,
    Filter,
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
    Strip,
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
            "filter" => Some(Self::Filter),
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
            "strip" => Some(Self::Strip),
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
    fn apply(&self, value: &str, arg: &str) -> Result<String>;
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
        registry.register(Box::new(super::filter::FilterModifier));
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
        registry.register(Box::new(super::strip::StripModifier::new()));
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
}

/// Invert a ContextModel::String ("true"/"false") or ContextModel::List of such strings.
pub fn invert(value: &str) -> String {
    match value {
        "true" => "false".to_string(),
        "false" => "true".to_string(),
        _ => value.to_string(),
    }
}
