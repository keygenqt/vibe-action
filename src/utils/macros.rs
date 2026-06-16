//! CLI output helpers.
//! Macros for colored terminal output. In debug mode, uses tracing.

/// Strip ANSI color codes from a string.
pub fn strip_ansi(s: &str) -> String {
    let re = regex::Regex::new(r"\x1b\[[0-9;]*m").unwrap();
    re.replace_all(s, "").to_string()
}

/// Format a message for display.
pub fn format_msg(s: &str) -> String {
    // Strip tag_ or tag- prefix if present.
    let s = s
        .strip_prefix("tag_")
        .or_else(|| s.strip_prefix("tag-"))
        .unwrap_or(s);
    // Strip trailing dot only if it's a single dot (not part of "..." or "..").
    let s = if s.ends_with('.') && !s.ends_with("..") {
        s.strip_suffix('.').unwrap_or(s)
    } else {
        s
    };
    // Lowercase the first character.
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(c) => c.to_lowercase().to_string() + chars.as_str(),
    }
}

/// Print a blank line.
#[macro_export]
macro_rules! print_newline {
    () => {{
        if !$crate::configs::app::AppConfig::is_debug() {
            println!();
        }
    }};
}

/// Print a progress message.
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

/// Print an info message.
#[macro_export]
macro_rules! print_info {
    ($($arg:tt)*) => {{
        if !$crate::configs::app::AppConfig::is_test() {
            let msg = format!($($arg)*);
            if $crate::configs::app::AppConfig::is_debug() {
                tracing::info!("{}", $crate::utils::macros::strip_ansi(&msg));
            } else {
                let formatted = $crate::utils::macros::format_msg(&msg);
                println!("{}", format!("\x1b[1m\x1b[94minfo\x1b[0m: {}", formatted));
            }
        }
    }};
}

/// Print a state message.
#[macro_export]
macro_rules! print_state {
    ($($arg:tt)*) => {{
        if !$crate::configs::app::AppConfig::is_test() {
            let msg = format!($($arg)*);
            if $crate::configs::app::AppConfig::is_debug() {
                tracing::debug!("{}", $crate::utils::macros::strip_ansi(&msg));
            } else {
                let formatted = $crate::utils::macros::format_msg(&msg);
                println!("{}", format!("\x1b[1m\x1b[36mstate\x1b[0m: {}", formatted));
            }
        }
    }};
}

/// Print a warning message.
#[macro_export]
macro_rules! print_warning {
    ($($arg:tt)*) => {{
        if !$crate::configs::app::AppConfig::is_test() {
            let msg = format!($($arg)*);
            if $crate::configs::app::AppConfig::is_debug() {
                tracing::warn!("{}", $crate::utils::macros::strip_ansi(&msg));
            } else {
                let formatted = $crate::utils::macros::format_msg(&msg);
                println!("{}", format!("\x1b[1m\x1b[93mwarning\x1b[0m: {}", formatted));
            }
        }
    }};
}

/// Print an error message.
#[macro_export]
macro_rules! print_error {
    ($($arg:tt)*) => {{
        let msg = format!($($arg)*);
        if $crate::configs::app::AppConfig::is_test() {
            println!("{}", msg);
        } else {
            if $crate::configs::app::AppConfig::is_debug() {
                tracing::error!("{}", $crate::utils::macros::strip_ansi(&msg));
            } else {
                let formatted = $crate::utils::macros::format_msg(&msg);
                println!("{}", format!("\x1b[1m\x1b[91merror\x1b[0m: {}", formatted));
            }
        }
    }};
}

/// Print an error message and exit with code 1.
#[macro_export]
macro_rules! exit_error {
    ($($arg:tt)*) => {{
        let msg = format!($($arg)*);
        if $crate::configs::app::AppConfig::is_test() {
            panic!("{}", msg);
        } else {
            if $crate::configs::app::AppConfig::is_debug() {
                tracing::error!("{}", $crate::utils::macros::strip_ansi(&msg));
            } else {
                let formatted = $crate::utils::macros::format_msg(&msg);
                println!("{}", format!("\x1b[1m\x1b[91merror\x1b[0m: {}", formatted));
            }
            std::process::exit(1);
        }
    }};
}

/// Print a success message.
#[macro_export]
macro_rules! print_success {
    ($($arg:tt)*) => {{
        let msg = format!($($arg)*);
        if $crate::configs::app::AppConfig::is_test() {
            println!("{}", msg);
        } else {
            if $crate::configs::app::AppConfig::is_debug() {
                tracing::info!("{}", $crate::utils::macros::strip_ansi(&msg));
            } else {
                let formatted = $crate::utils::macros::format_msg(&msg);
                println!("{}", format!("\x1b[1m\x1b[32msuccess\x1b[0m: {}", formatted));
            }
        }
    }};
}

/// Print a success message with automatic line wrapping and framed borders.
#[macro_export]
macro_rules! print_rich_block {
    ($($arg:tt)*) => {{
        let msg = format!($($arg)*);
        if $crate::configs::app::AppConfig::is_test() {
            println!("{}", msg);
        } else {
            if $crate::configs::app::AppConfig::is_debug() {
                tracing::info!("{}", $crate::utils::macros::strip_ansi(&msg));
            } else {
                let max_width = 120usize;
                let wrapped: Vec<String> = msg
                    .lines()
                    .flat_map(|line| {
                        let chars: Vec<char> = line.chars().collect();
                        if chars.len() <= max_width {
                            vec![line.to_string()]
                        } else {
                            let mut result = Vec::new();
                            let mut start = 0;
                            while start < chars.len() {
                                let mut end = (start + max_width).min(chars.len());

                                if end < chars.len() && !chars[end].is_whitespace() && !chars[end - 1].is_whitespace() {
                                    let mut space_idx = end;
                                    while space_idx > start && !chars[space_idx].is_whitespace() {
                                        space_idx -= 1;
                                    }
                                    if space_idx > start {
                                        end = space_idx;
                                    }
                                }
                                let sub_str: String = chars[start..end].iter().collect();
                                if start == 0 || !sub_str.trim().is_empty() || sub_str.len() == max_width {
                                    result.push(sub_str.to_string());
                                }
                                start = end;
                                if start < chars.len() && chars[start].is_whitespace() {
                                    start += 1;
                                }
                            }
                            result
                        }
                    })
                    .collect();

                let width = wrapped.iter()
                    .map(|l| l.chars().count())
                    .max()
                    .unwrap_or(0)
                    .max(13);

                let top = format!("── success ──{}", "─".repeat(width.saturating_sub(13)));
                let bottom = "─".repeat(width);

                println!("\x1b[1m\x1b[32m{}\x1b[0m", top);
                for line in &wrapped {
                    println!("\x1b[37m{}\x1b[0m", line);
                }
                println!("\x1b[1m\x1b[32m{}\x1b[0m", bottom);
            }
        }
    }};
}
