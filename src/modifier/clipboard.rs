//! Clipboard modifier — copies value to system clipboard.

use anyhow::Result;

use super::modifier::Modifier;
use super::modifier::ModifierKey;
use crate::models::context::ContextModel;

pub struct ClipboardModifier;

impl Modifier for ClipboardModifier {
    fn key(&self) -> ModifierKey {
        ModifierKey::Clipboard
    }

    fn apply(&self, value: &ContextModel, _arg: &str) -> Result<ContextModel> {
        let text = value.to_string();
        let mut clipboard = arboard::Clipboard::new()
            .map_err(|e| anyhow::anyhow!("Failed to access clipboard: {}", e))?;
        clipboard
            .set_text(&text)
            .map_err(|e| anyhow::anyhow!("Failed to set clipboard: {}", e))?;
        Ok(value.clone()) // pass through unchanged
    }
}
