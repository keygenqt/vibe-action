//! System provider for `{system_clipboard_image}` — image from clipboard as base64 PNG.

use crate::models::context::ContextModel;
use crate::system::system::{SystemKey, SystemProvider};
use anyhow::Result;
use image::ImageEncoder;
use image::codecs::png::PngEncoder;

pub struct SystemClipboardImageProvider;

impl SystemProvider for SystemClipboardImageProvider {
    fn key(&self) -> SystemKey {
        SystemKey::ClipboardImage
    }

    fn resolve(&self) -> Result<ContextModel> {
        let mut clipboard = match arboard::Clipboard::new() {
            Ok(cb) => cb,
            Err(_) => return Ok(ContextModel::String(String::new())),
        };

        let img = match clipboard.get_image() {
            Ok(img) => img,
            Err(_) => return Ok(ContextModel::String(String::new())),
        };

        let mut png_bytes = Vec::new();
        let encoder = PngEncoder::new(&mut png_bytes);
        encoder
            .write_image(
                &img.bytes,
                img.width as u32,
                img.height as u32,
                image::ExtendedColorType::Rgba8,
            )
            .map_err(|e| anyhow::anyhow!("Failed to encode PNG: {}", e))?;

        use base64::Engine;
        let base64 = base64::engine::general_purpose::STANDARD.encode(&png_bytes);
        Ok(ContextModel::String(base64))
    }
}
