//! Inspect operators (→ bool): keys and registration.

use crate::operator::operator::{OperatorKey, OperatorRegistry};

/// Inspect operator keys (→ bool).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InspectKey {
    Contains,
    Empty,
    Equals,
}

impl InspectKey {
    /// Parse from the pipe-syntax string; None if unknown.
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "contains" => Some(Self::Contains),
            "empty" => Some(Self::Empty),
            "equals" => Some(Self::Equals),
            _ => None,
        }
    }

    /// Wrap into the unified OperatorKey.
    pub fn key(self) -> OperatorKey {
        OperatorKey::Inspect(self)
    }
}

/// Register all Inspect operators.
pub fn register(registry: &mut OperatorRegistry) {
    registry.register(Box::new(super::impls::contains::ContainsOperator));
    registry.register(Box::new(super::impls::empty::EmptyOperator));
    registry.register(Box::new(super::impls::equals::EqualsOperator));
}
