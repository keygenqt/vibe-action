//! Status command handler.
//! Shows system status: version, config paths, context state.

use colored::Colorize;

use crate::configs::app::AppConfig;
use crate::output::output::OutputKind;
use crate::utils::app::{app_version, config_version};
use crate::{print_template, print_text, utils};

/// Execute the `status` command.
pub async fn execute() {
    let config = match AppConfig::instance() {
        Ok(c) => c,
        Err(e) => {
            print_text!(OutputKind::Error, "{}", e);
            std::process::exit(1);
        }
    };
    print_template!(
        ExportContext::Status,
        OutputKind::Info,
        "actions - {actions}, version - {version}, config - {config}",
        "actions" => config
            .flows
            .as_ref()
            .map(|f| f.flows.len())
            .unwrap_or(0)
            .to_string()
            .cyan(),
        "version" => format!("v{}", app_version()).cyan(),
        "config" => format!("v{}", config_version()).cyan(),
        "actions_path" => utils::path::actions_dir().display().to_string(),
    );
}
