//! Status command handler.
//! Shows system status: version, config paths, context state.

use crate::configs::app::AppConfig;
use crate::default::default::default_flows;
use crate::output::output::OutputKind;
use crate::print_template;
use crate::print_text;
use crate::utils;
use crate::utils::app::app_version;
use crate::utils::app::config_version;

/// Execute the `status` command.
pub async fn execute() {
    let config = match AppConfig::instance() {
        Ok(c) => c,
        Err(e) => {
            print_text!(OutputKind::Error, "{}", e);
            std::process::exit(1);
        }
    };

    // Collect built-in flow names to count custom actions.
    let builtin_names: std::collections::HashSet<String> = default_flows()
        .iter()
        .filter_map(|f| f.name().ok())
        .collect();

    let (total_actions, total_actions_api, custom_actions_api) = config
        .flows
        .as_ref()
        .map(|f| {
            let total = f.flows.len();
            let total_api = f.flows.iter().filter(|fl| fl.api.is_some()).count();
            // Assuming the flow struct has a `name` field.
            let custom_api = f
                .flows
                .iter()
                .filter(|fl| fl.api.is_some() && !builtin_names.contains(&fl.name))
                .count();
            (total, total_api, custom_api)
        })
        .unwrap_or((0, 0, 0));

    print_template!(
        ExportContext::Status,
        OutputKind::Info,
        "actions - {total_actions|cyan}, version - {version|cyan}, config - {config|cyan}",
        "total_actions" => total_actions,
        "total_actions_api" => total_actions_api,
        "custom_actions_api" => custom_actions_api,
        "version" => format!("v{}", app_version()),
        "config" => format!("v{}", config_version()),
        "actions_path" => utils::path::actions_dir().display().to_string(),
        "config_path" => utils::path::config_path().display().to_string(),
        "cache_path" => utils::path::cache_dir().display().to_string(),
    );
}
