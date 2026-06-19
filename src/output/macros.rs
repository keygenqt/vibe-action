//! CLI output helpers — thin wrappers over OutputRegistry.

/// Format a message for display.
pub fn format_msg(s: &str) -> String {
    let s = s
        .strip_prefix("tag_")
        .or_else(|| s.strip_prefix("tag-"))
        .unwrap_or(s);
    let s = if s.ends_with('.') && !s.ends_with("..") {
        s.strip_suffix('.').unwrap_or(s)
    } else {
        s
    };
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(c) => c.to_lowercase().to_string() + chars.as_str(),
    }
}

#[macro_export]
macro_rules! print_newline {
    () => {{
        if $crate::configs::app::AppConfig::output().level()
            == $crate::output::output::OutputLevel::Cli
        {
            println!();
        }
    }};
}

#[macro_export]
macro_rules! print_error {
    ($($arg:tt)*) => {{
        let msg = format!($($arg)*);
        $crate::configs::app::AppConfig::output().error(&msg);
     }};
}

#[macro_export]
macro_rules! exit_error {
    ($($arg:tt)*) => {{
        let msg = format!($($arg)*);
        $crate::configs::app::AppConfig::output().error(&msg);
        std::process::exit(1);
    }};
}

#[macro_export]
macro_rules! print_warning {
    ($($arg:tt)*) => {{
        let msg = format!($($arg)*);
        $crate::configs::app::AppConfig::output().warning(&msg);
    }};
}

#[macro_export]
macro_rules! print_info {
    ($($arg:tt)*) => {{
        let msg = format!($($arg)*);
        $crate::configs::app::AppConfig::output().info(&msg);
    }};
}

#[macro_export]
macro_rules! print_success {
    ($($arg:tt)*) => {{
        let msg = format!($($arg)*);
        $crate::configs::app::AppConfig::output().success(&msg);
    }};
}

#[macro_export]
macro_rules! print_debug {
    ($($arg:tt)*) => {{
        let msg = format!($($arg)*);
        $crate::configs::app::AppConfig::output().debug(&msg);
    }};
}

#[macro_export]
macro_rules! print_trace {
    ($($arg:tt)*) => {{
        let msg = format!($($arg)*);
        $crate::configs::app::AppConfig::output().trace(&msg);
    }};
}

#[macro_export]
macro_rules! print_progress {
    ($($arg:tt)*) => {{
        let msg = format!($($arg)*);
         $crate::configs::app::AppConfig::output().progress(&msg);
    }};
}
