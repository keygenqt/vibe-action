//! Write operators (value → world, pass-through): keys and registration.

use crate::operator::operator::OperatorKey;
use crate::operator::operator::OperatorRegistry;

/// Write operator keys (value → world, pass-through).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WriteKey {
    ClipboardText,
    ClipboardImage,
    File,
}

impl WriteKey {
    /// Parse from the pipe-syntax string; None if unknown.
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "clipboard_text" => Some(Self::ClipboardText),
            "clipboard_image" => Some(Self::ClipboardImage),
            "file" => Some(Self::File),
            _ => None,
        }
    }

    /// Wrap into the unified OperatorKey.
    pub fn key(self) -> OperatorKey {
        OperatorKey::Write(self)
    }
}

/// Register all Write operators.
pub fn register(registry: &mut OperatorRegistry) {
    registry.register(Box::new(
        super::impls::clipboard_text::ClipboardTextOperator,
    ));
    registry.register(Box::new(
        super::impls::clipboard_image::ClipboardImageOperator,
    ));
    registry.register(Box::new(super::impls::file::FileOperator));
}
