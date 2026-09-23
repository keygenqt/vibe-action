//! Base64 image detection (prefix scan) and encoding/validation.
//! See [`crate::utils`] module-level docs for summary.

use base64::Engine;
use base64::engine::general_purpose::STANDARD as BASE64;

/// Base64 prefixes for image formats (PNG, JPEG, GIF, WEBP).
/// Used to scan free text for embedded base64 images (not for validation).
pub fn image_base64_prefixes() -> &'static [&'static str] {
    &["iVBOR", "/9j/", "R0lGOD", "UklGR"]
}

/// True if bytes decode into a valid image.
pub fn is_image_bytes(bytes: &[u8]) -> bool {
    image::load_from_memory(bytes).is_ok()
}

/// True if the string is base64 that decodes into a valid image.
pub fn is_image_base64(s: &str) -> bool {
    BASE64
        .decode(s.as_bytes())
        .ok()
        .is_some_and(|b| is_image_bytes(&b))
}

/// Encode raw bytes as a base64 string.
pub fn encode_to_base64(bytes: &[u8]) -> String {
    BASE64.encode(bytes)
}
