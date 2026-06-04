//! CLI output helpers.
//! Macros for colored terminal output. In debug mode, uses tracing.

/// Strip ANSI color codes from a string.
pub fn strip_ansi(s: &str) -> String {
    let re = regex::Regex::new(r"\x1b\[[0-9;]*m").unwrap();
    re.replace_all(s, "").to_string()
}

/// Print a blank line (CLI only, skipped in debug mode).
#[macro_export]
macro_rules! print_newline {
    () => {{
        if !$crate::configs::app::AppConfig::is_debug() {
            println!();
        }
    }};
}

/// Print an error message. CLI: red. Debug: tracing::error.
#[macro_export]
macro_rules! print_error {
    ($($arg:tt)*) => {{
        if $crate::configs::app::AppConfig::is_debug() {
            let msg = format!($($arg)*);
            tracing::error!("{}", $crate::utils::macros::strip_ansi(&msg));
        } else {
            println!("{}", format!("\x1b[1m\x1b[91merror\x1b[0m: {}", format!($($arg)*)));
        }
    }};
}

/// Print an info message. CLI: blue. Debug: tracing::info.
#[macro_export]
macro_rules! print_info {
    ($($arg:tt)*) => {{
        if $crate::configs::app::AppConfig::is_debug() {
            let msg = format!($($arg)*);
            tracing::info!("{}", $crate::utils::macros::strip_ansi(&msg));
        } else {
            println!("{}", format!("\x1b[1m\x1b[94minfo\x1b[0m: {}", format!($($arg)*)));
        }
    }};
}

/// Print a warning message. CLI: yellow. Debug: tracing::warn.
#[macro_export]
macro_rules! print_warning {
    ($($arg:tt)*) => {{
        if $crate::configs::app::AppConfig::is_debug() {
            let msg = format!($($arg)*);
            tracing::warn!("{}", $crate::utils::macros::strip_ansi(&msg));
        } else {
            println!("{}", format!("\x1b[1m\x1b[93mwarning\x1b[0m: {}", format!($($arg)*)));
        }
    }};
}

/// Print a success message. CLI: green. Debug: tracing::info.
#[macro_export]
macro_rules! print_success {
    ($($arg:tt)*) => {{
        if $crate::configs::app::AppConfig::is_debug() {
            let msg = format!($($arg)*);
            tracing::info!("{}", $crate::utils::macros::strip_ansi(&msg));
        } else {
            println!("{}", format!("\x1b[1m\x1b[32msuccess\x1b[0m: {}", format!($($arg)*)));
        }
    }};
}

/// Print a state message. CLI: cyan. Debug: tracing::debug.
#[macro_export]
macro_rules! print_state {
    ($($arg:tt)*) => {{
        if $crate::configs::app::AppConfig::is_debug() {
            let msg = format!($($arg)*);
            tracing::debug!("{}", $crate::utils::macros::strip_ansi(&msg));
        } else {
            println!("{}", format!("\x1b[1m\x1b[36mstate\x1b[0m: {}", format!($($arg)*)));
        }
    }};
}

/// Print a progress message (CLI only, skipped in debug mode).
#[macro_export]
macro_rules! print_progress {
    ($($arg:tt)*) => {{
        if !$crate::configs::app::AppConfig::is_debug() {
            print!("\r\x1b[1m\x1b[36mprogress\x1b[0m: {}", format!($($arg)*));
            std::io::Write::flush(&mut std::io::stdout()).unwrap();
        }
    }};
}

/// Print an error message and exit with code 1.
#[macro_export]
macro_rules! exit_error {
    ($($arg:tt)*) => {{
        if $crate::configs::app::AppConfig::is_debug() {
            let msg = format!($($arg)*);
            tracing::error!("{}", $crate::utils::macros::strip_ansi(&msg));
        } else {
            println!("{}", format!("\x1b[1m\x1b[91merror\x1b[0m: {}", format!($($arg)*)));
        }
        std::process::exit(1);
    }};
}
