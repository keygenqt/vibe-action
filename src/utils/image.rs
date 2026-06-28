//! Image utilities — base64 detection and prefix constants.

/// Base64 prefixes for image formats (PNG, JPEG, GIF, WEBP).
pub fn image_base64_prefixes() -> &'static [&'static str] {
    &["iVBOR", "/9j/", "R0lGOD", "UklGR"]
}

/// Check if a string starts with a base64-encoded image prefix.
pub fn is_image(value: &str) -> bool {
    image_base64_prefixes().iter().any(|p| value.starts_with(p))
}
