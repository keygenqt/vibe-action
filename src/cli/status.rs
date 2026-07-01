//! Status command handler.
//! Shows system status: version, config paths, context state.

use colored::Colorize;

use crate::configs::app::AppConfig;
use crate::utils::app::{app_version, config_version};
use crate::{exit_error, print_info};

/// Execute the `status` command.
pub async fn execute() {
    let config = match AppConfig::instance() {
        Ok(c) => c,
        Err(e) => exit_error!("{}", e),
    };
    print_info!(
        "actions - {}, version - {}, config - {}",
        config
            .flows
            .as_ref()
            .map(|f| f.flows.len())
            .unwrap_or(0)
            .to_string()
            .cyan(),
        format!("v{}", app_version()).cyan(),
        format!("v{}", config_version()).cyan(),
    );
}
