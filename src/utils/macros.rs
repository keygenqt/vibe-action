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

/// Print a success message inside a green box-drawing panel with white text.
#[macro_export]
macro_rules! print_success_block {
    ($($arg:tt)*) => {{
        let msg = format!($($arg)*);
        let max_width = 80usize;
        let mut lines: Vec<String> = Vec::new();

        // Safe Word Wrapping logic without breaking multiline layouts
        for raw_line in msg.lines() {
            let words: Vec<&str> = raw_line.split_whitespace().collect();
            if words.is_empty() {
                lines.push(String::new());
                continue;
            }

            let mut current_line = String::new();
            for word in words {
                let word_count = word.chars().count();
                let current_count = current_line.chars().count();

                if current_count + word_count + (if current_count > 0 { 1 } else { 0 }) <= max_width {
                    if !current_line.is_empty() {
                        current_line.push(' ');
                    }
                    current_line.push_str(word);
                } else {
                    if !current_line.is_empty() {
                        lines.push(current_line);
                    }
                    // Handle rare edge-case: single word longer than max_width
                    if word_count > max_width {
                        let word_chars: Vec<char> = word.chars().collect();
                        for chunk in word_chars.chunks(max_width) {
                            lines.push(chunk.iter().collect());
                        }
                        current_line = String::new();
                    } else {
                        current_line = word.to_string();
                    }
                }
            }
            if !current_line.is_empty() {
                lines.push(current_line);
            }
        }

        // Calculate dynamic block boundaries safely
        let max_line_width = lines.iter()
            .map(|l| l.chars().count())
            .max()
            .unwrap_or(0);

        // Force width to be at least 80 characters for consistent look
        let width = max_line_width.max(80);

        // PERFECT MATHEMATICAL ALIGNMENT (Total width = width + 4)
        // Header "┌── success ──" (14) + corner "┐" (1) = 15 fixed chars.
        // Remaining dashes needed to reach (width + 4): width + 4 - 15 = width - 11.
        let top_padding = width.saturating_sub(11);
        let top = format!("┌── success ──{}┐", "─".repeat(top_padding));

        // Total bottom width is (width + 2) dashes + 2 corners = width + 4.
        let bottom = format!("└{}┘", "─".repeat(width + 2));


        // Render the colored container output
        println!("\x1b[1m\x1b[32m{}\x1b[0m", top);
        for line in &lines {
            let padding = " ".repeat(width.saturating_sub(line.chars().count()));
            println!("\x1b[1m\x1b[32m│\x1b[0m \x1b[37m{}{}\x1b[0m \x1b[1m\x1b[32m│\x1b[0m", line, padding);
        }
        println!("\x1b[1m\x1b[32m{}\x1b[0m", bottom);
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
            let msg = format!($($arg)*);
            // Clear to end of line before printing.
            print!("\r\x1b[1m\x1b[36mprogress\x1b[0m: {}\x1b[K", msg);
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
