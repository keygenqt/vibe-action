//! System provider for `{system_clipboard}` — text content from clipboard.

use crate::configs::app::AppConfig;
use crate::models::context::ContextModel;
use crate::system::system::{SystemKey, SystemProvider};
use anyhow::Result;

pub struct SystemClipboardProvider;

impl SystemProvider for SystemClipboardProvider {
    fn key(&self) -> SystemKey {
        SystemKey::Clipboard
    }

    fn resolve(&self) -> Result<ContextModel> {
        let mut clipboard = arboard::Clipboard::new()
            .map_err(|e| anyhow::anyhow!("Failed to access clipboard: {}", e))?;
        let text = clipboard
            .get_text()
            .map_err(|_| anyhow::anyhow!("Clipboard is empty or contains non-text data."))?;

        let bpe = tiktoken_rs::cl100k_base().unwrap();
        let tokens = bpe.encode_with_special_tokens(&text).len();
        let config = AppConfig::instance()?;
        let max_ctx = config
            .cluster
            .iter()
            .map(|c| c.num_ctx)
            .max()
            .unwrap_or(4096);

        if tokens > max_ctx {
            anyhow::bail!(
                "Clipboard text is too large ({} tokens). Max context size is {} tokens.",
                tokens,
                max_ctx
            );
        }

        Ok(ContextModel::String(text))
    }
}
