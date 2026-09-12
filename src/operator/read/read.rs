//! Read operators (world → value): keys and registration.

use crate::operator::operator::{OperatorKey, OperatorRegistry};

/// Read operator keys (world → value).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ReadKey {
    Ast,
    Fetch,
    Resolve,
    Scan,
    Text,
}

impl ReadKey {
    /// Parse from the pipe-syntax string; None if unknown.
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "ast" => Some(Self::Ast),
            "fetch" => Some(Self::Fetch),
            "resolve" => Some(Self::Resolve),
            "scan" => Some(Self::Scan),
            "text" => Some(Self::Text),
            _ => None,
        }
    }

    /// Wrap into the unified OperatorKey.
    pub fn key(self) -> OperatorKey {
        OperatorKey::Read(self)
    }
}

/// Register all Read operators.
pub fn register(registry: &mut OperatorRegistry) {
    registry.register(Box::new(super::impls::ast::AstOperator));
    registry.register(Box::new(super::impls::fetch::FetchOperator));
    registry.register(Box::new(super::impls::resolve::ResolveOperator));
    registry.register(Box::new(super::impls::scan::ScanOperator));
    registry.register(Box::new(super::impls::text::TextOperator));
}
