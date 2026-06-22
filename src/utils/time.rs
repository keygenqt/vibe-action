//! Time formatting utilities.
//! Converts milliseconds or Duration into a human-readable string.

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
