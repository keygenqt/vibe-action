//! CLI output helpers.
//! Macros for colored terminal output. In debug mode, uses tracing.

/// Strip ANSI color codes from a string.
pub fn strip_ansi(s: &str) -> String {
    let re = regex::Regex::new(r"\x1b\[[0-9;]*m").unwrap();
    re.replace_all(s, "").to_string()
}

/// Format a message for display.
pub fn format_msg(s: &str) -> String {
    let s = s.strip_suffix('.').unwrap_or(s);
    let s = s
        .strip_prefix("tag_")
        .or_else(|| s.strip_prefix("tag-"))
        .unwrap_or(s);
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(c) => c.to_lowercase().to_string() + chars.as_str(),
    }
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
            let msg = format!($($arg)*);
            let formatted = $crate::utils::macros::lower_first_char(&msg);
            println!("{}", format!("\x1b[1m\x1b[91merror\x1b[0m: {}", formatted));
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
            let msg = format!($($arg)*);
            let formatted = $crate::utils::macros::format_msg(&msg);
            println!("{}", format!("\x1b[1m\x1b[94minfo\x1b[0m: {}", formatted));
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
            let msg = format!($($arg)*);
            let formatted = $crate::utils::macros::format_msg(&msg);
            println!("{}", format!("\x1b[1m\x1b[93mwarning\x1b[0m: {}", formatted));
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
            let msg = format!($($arg)*);
            let formatted = $crate::utils::macros::format_msg(&msg);
            println!("{}", format!("\x1b[1m\x1b[32msuccess\x1b[0m: {}", formatted));
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
            let msg = format!($($arg)*);
            let formatted = $crate::utils::macros::format_msg(&msg);
            println!("{}", format!("\x1b[1m\x1b[36mstate\x1b[0m: {}", formatted));
        }
    }};
}

/// Print a progress message (CLI only, skipped in debug mode).
#[macro_export]
macro_rules! print_progress {
    ($($arg:tt)*) => {{
        if !$crate::configs::app::AppConfig::is_debug() {
            let msg = format!($($arg)*);
            let formatted = $crate::utils::macros::format_msg(&msg);
            print!("\r\x1b[1m\x1b[36mprogress\x1b[0m: {}\x1b[K", formatted);
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
            let msg = format!($($arg)*);
            let formatted = $crate::utils::macros::format_msg(&msg);
            println!("{}", format!("\x1b[1m\x1b[91merror\x1b[0m: {}", formatted));
        }
        std::process::exit(1);
    }};
}

/// Print a success message framed by top and bottom lines matching the text length.
#[macro_export]
macro_rules! print_rich_block {
    ($($arg:tt)*) => {{
        let msg = format!($($arg)*);
        let max_line_width = msg.lines()
            .map(|l| l.chars().count())
            .max()
            .unwrap_or(0);
        let width = max_line_width;
        let top_padding = width.saturating_sub(13);
        let top = format!("── success ──{}", "─".repeat(top_padding));
        let bottom = "─".repeat(width);
        println!("\x1b[1m\x1b[32m{}\x1b[0m", top);
        for line in msg.lines() {
            println!("\x1b[37m{}\x1b[0m", line);
        }
        println!("\x1b[1m\x1b[32m{}\x1b[0m", bottom);
    }};
}
