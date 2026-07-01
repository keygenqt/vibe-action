//! Application metadata and CLI styling utilities.
//! Provides version info, help text formatting, and CLI styling configuration.

use clap::builder::Styles;
use colored::Colorize;

use crate::utils::constants::CONFIG_VERSION;

/// Returns the application version from Cargo.toml
pub fn app_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

/// Config version from constants.
pub fn config_version() -> &'static str {
    CONFIG_VERSION
}

/// Returns the application name for CLI usage.
pub fn app_name() -> &'static str {
    "vibe-action"
}

/// Returns the pretty application name for notifications and UI.
pub fn app_name_pretty() -> &'static str {
    "Vibe Action"
}

/// Returns the application about text with styling.
pub fn app_about() -> String {
    format!(
        r#"
{} - command router for shell and LLM tasks via YAML pipelines

{}
"#,
        app_name_pretty().bright_green().bold(),
        "Part of Vibe tools ecosystem".italic()
    )
}

/// Returns CLI styling configuration
pub fn app_styles() -> Styles {
    Styles::styled()
        .header(clap_cargo::style::HEADER)
        .usage(clap_cargo::style::USAGE)
        .literal(clap_cargo::style::LITERAL)
        .placeholder(clap_cargo::style::PLACEHOLDER)
        .error(clap_cargo::style::ERROR)
        .valid(clap_cargo::style::VALID)
        .invalid(clap_cargo::style::INVALID)
}
