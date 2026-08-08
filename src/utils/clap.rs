use std::collections::HashSet;

use clap::Command;

use crate::configs::app::AppConfig;
use crate::default::default::default_flows;
use crate::output::output::OutputKind;
use crate::print_template;
use crate::utils::app;

/// System commands displayed in a separate section.
pub const SYSTEM_COMMANDS: &[&str] = &["clean", "status", "bench", "stop"];

/// Builds the full hierarchical CLI command tree including dynamic YAML actions.
#[macro_export]
macro_rules! build_app {
    ($config:expr) => {{
        use clap::Arg;
        use clap::Command;
        use clap::CommandFactory;
        let mut app = $crate::App::command();
        if let Some(actions_model) = &$config.flows {
            for flow in &actions_model.flows {
                let mut dynamic_cmd = Command::new(flow.name.as_str()).about(&flow.about);
                for arg_def in &flow.args {
                    let clap_arg: Arg = arg_def.into();
                    dynamic_cmd = dynamic_cmd.arg(clap_arg);
                }
                app = app.subcommand(dynamic_cmd);
            }
        }
        app
    }};
}

/// Print custom colored help with grouped sections.
pub fn print_custom_help(app_builder: &Command, config: &AppConfig) {
    let mut actions: Vec<_> = app_builder
        .get_subcommands()
        .filter(|s| !SYSTEM_COMMANDS.contains(&s.get_name()))
        .collect();
    actions.sort_by_key(|s| s.get_name());

    let max_len = actions
        .iter()
        .map(|s| s.get_name().len())
        .max()
        .unwrap_or(15)
        + 1;

    // Collect built-in flow names to distinguish custom actions.
    let builtin_names: HashSet<String> = default_flows()
        .iter()
        .filter_map(|f| f.name().ok())
        .collect();

    print_template!(
        OutputKind::Plain,
        "\n{app_name|bright_green|bold} - command router for shell and LLM tasks via YAML pipelines\n\n{ecosystem|italic}\n\n{usage|bright_green|bold} {app_name_cli|cyan|bold} {command|cyan}\n",
        "app_name" => app::app_name_pretty(),
        "ecosystem" => "Part of Vibe tools ecosystem",
        "usage" => "Usage:",
        "app_name_cli" => app::app_name(),
        "command" => "[COMMAND]"
    );

    print_template!(
        OutputKind::Plain,
        "{header|bright_green|bold}",
        "header" => "Actions:"
    );

    for sub in &actions {
        let name = sub.get_name();
        let about = sub.get_about().unwrap_or_default().to_string();
        let formatted_name = format!("  {:<width$}", name, width = max_len);
        let is_custom = !builtin_names.contains(name);

        if let Some(flow) = config.flows.as_ref().and_then(|f| f.find(name)) {
            print_template!(
                ExportContext::Actions,
                OutputKind::Plain,
                "{formatted_name|cyan|bold} {about}",
                "formatted_name" => formatted_name,
                "name" => name,
                "about" => about,
                "args" => &flow.args,
                "api" => &flow.api,
                "is_custom" => is_custom
            );
        } else {
            print_template!(
                ExportContext::Actions,
                OutputKind::Plain,
                "{name|cyan|bold} {about}",
                "name" => formatted_name,
                "about" => about,
            );
        }
    }

    print_template!(
        OutputKind::Plain,
        "\n{header|bright_green|bold}",
        "header" => "Commands:"
    );

    for cmd_name in SYSTEM_COMMANDS {
        if cmd_name != &"help" {
            if let Some(cmd) = app_builder.find_subcommand(cmd_name) {
                let formatted_name = format!("  {:<15}", cmd.get_name());

                print_template!(
                    OutputKind::Plain,
                    "{name|cyan|bold} {about}",
                    "name" => formatted_name,
                    "about" => cmd.get_about().unwrap_or_default().to_string()
                );
            }
        }
    }

    print_template!(
        OutputKind::Plain,
        "\n{header|bright_green|bold}",
        "header" => "Options:"
    );

    print_template!(
        OutputKind::Plain,
        "{flag|cyan|bold} {desc}",
        "flag" => format!("  {:<15}", "-h, --help"),
        "desc" => "Print help"
    );
    print_template!(
        OutputKind::Plain,
        "{flag|cyan|bold} {desc}",
        "flag" => format!("  {:<15}", "-V, --version"),
        "desc" => "Print version"
    );
}
