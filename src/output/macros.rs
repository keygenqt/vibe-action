//! CLI output helpers — thin wrappers over OutputRegistry.

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
