//! Human-readable formatters: duration, bytes, base64 image extraction.
//! See [`crate::utils`] module-level docs for summary.

use std::time::Duration;

/// Input type for time formatting — either raw milliseconds or a Duration.
pub enum TimeInput {
    Millis(u64),
    Duration(Duration),
}

/// Allows using a u64 (milliseconds) where a TimeInput is expected.
impl From<u64> for TimeInput {
    fn from(ms: u64) -> Self {
        TimeInput::Millis(ms)
    }
}

/// Allows using a Duration where a TimeInput is expected.
impl From<Duration> for TimeInput {
    fn from(d: Duration) -> Self {
        TimeInput::Duration(d)
    }
}

/// Formats a time value into a human-readable string.
/// Uses seconds for values >= 1s, milliseconds for >= 1ms, microseconds otherwise.
pub fn format_duration<T: Into<TimeInput>>(input: T) -> String {
    let duration = match input.into() {
        TimeInput::Millis(ms) => Duration::from_millis(ms),
        TimeInput::Duration(d) => d,
    };

    let secs = duration.as_secs_f64();

    if secs >= 1.0 {
        format!("{:.2}s", secs)
    } else if duration.as_millis() > 0 {
        format!("{}ms", duration.as_millis())
    } else {
        format!("{}µs", duration.as_micros())
    }
}

/// Formats a byte size into a human-readable string.
/// Uses GB for >= 1GB, MB for >= 1MB, KB for >= 1KB, bytes otherwise.
pub fn format_bytes(bytes: usize) -> String {
    if bytes >= 1_000_000_000 {
        format!("{:.1}GB", bytes as f64 / 1_000_000_000.0)
    } else if bytes >= 1_000_000 {
        format!("{:.1}MB", bytes as f64 / 1_000_000.0)
    } else if bytes >= 1_000 {
        format!("{}KB", bytes / 1_000)
    } else {
        format!("{}B", bytes)
    }
}

/// Extract base64 images from resolved prompt text.
/// Returns cleaned prompt and list of extracted images.
pub fn format_image_prompt(prompt: &str) -> (String, Vec<String>) {
    let mut images = Vec::new();
    let mut cleaned = prompt.to_string();
    let prefixes = crate::utils::image::image_base64_prefixes();

    for prefix in prefixes {
        while let Some(pos) = cleaned.find(prefix) {
            let end = cleaned[pos..]
                .find(|c: char| c.is_whitespace())
                .map(|p| pos + p)
                .unwrap_or(cleaned.len());

            images.push(cleaned[pos..end].to_string());
            cleaned.replace_range(pos..end, "");
        }
    }

    (cleaned.trim().to_string(), images)
}
