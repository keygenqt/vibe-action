//! Transform operators (value → value): keys and registration.

use crate::operator::operator::OperatorKey;
use crate::operator::operator::OperatorRegistry;

/// Transform operator keys (value → value).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TransformKey {
    Base64,
    Default,
    Filter,
    Format,
    Grep,
    Join,
    Lower,
    Item,
    Replace,
    Reverse,
    Size,
    Sort,
    Split,
    Strip,
    Tail,
    Take,
    Trim,
    Uniq,
    Upper,
}

impl TransformKey {
    /// Parse from the pipe-syntax string; None if unknown.
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "base64" => Some(Self::Base64),
            "default" => Some(Self::Default),
            "filter" => Some(Self::Filter),
            "format" => Some(Self::Format),
            "grep" => Some(Self::Grep),
            "join" => Some(Self::Join),
            "lower" => Some(Self::Lower),
            "item" => Some(Self::Item),
            "replace" => Some(Self::Replace),
            "reverse" => Some(Self::Reverse),
            "size" => Some(Self::Size),
            "sort" => Some(Self::Sort),
            "split" => Some(Self::Split),
            "strip" => Some(Self::Strip),
            "tail" => Some(Self::Tail),
            "take" => Some(Self::Take),
            "trim" => Some(Self::Trim),
            "uniq" => Some(Self::Uniq),
            "upper" => Some(Self::Upper),
            _ => None,
        }
    }

    /// Wrap into the unified OperatorKey.
    pub fn key(self) -> OperatorKey {
        OperatorKey::Transform(self)
    }
}

/// Register all Transform operators.
pub fn register(registry: &mut OperatorRegistry) {
    registry.register(Box::new(super::impls::base64::Base64Operator));
    registry.register(Box::new(super::impls::default::DefaultOperator));
    registry.register(Box::new(super::impls::filter::FilterOperator));
    registry.register(Box::new(super::impls::format::FormatOperator));
    registry.register(Box::new(super::impls::grep::GrepOperator));
    registry.register(Box::new(super::impls::join::JoinOperator));
    registry.register(Box::new(super::impls::lower::LowerOperator));
    registry.register(Box::new(super::impls::item::ItemOperator));
    registry.register(Box::new(super::impls::replace::ReplaceOperator));
    registry.register(Box::new(super::impls::reverse::ReverseOperator));
    registry.register(Box::new(super::impls::size::SizeOperator));
    registry.register(Box::new(super::impls::sort::SortOperator));
    registry.register(Box::new(super::impls::split::SplitOperator));
    registry.register(Box::new(super::impls::strip::StripOperator::new()));
    registry.register(Box::new(super::impls::tail::TailOperator));
    registry.register(Box::new(super::impls::take::TakeOperator));
    registry.register(Box::new(super::impls::trim::TrimOperator));
    registry.register(Box::new(super::impls::uniq::UniqOperator));
    registry.register(Box::new(super::impls::upper::UpperOperator));
}
