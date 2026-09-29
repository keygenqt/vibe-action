//! Dynamic CLI command tree from YAML pipelines and custom colored help.
//! See [`crate::utils`] module-level docs for summary.

use std::collections::HashSet;

use clap::Command;

use crate::configs::app::AppConfig;
use crate::default::default::default_pipelines;
use crate::models::pipeline::PipelineModel;
use crate::output::output::OutputKind;
use crate::print_template;
use crate::utils::app;

/// System commands displayed in a separate section.
pub const SYSTEM_COMMANDS: &[&str] = &["clean", "status", "bench", "stop"];

/// Builds the full hierarchical CLI command tree including dynamic YAML actions.
/// Grouped pipelines nest under a group parent command (vibe <group> <action>).
#[macro_export]
macro_rules! build_app {
    ($config:expr) => {{
        use clap::Arg;
        use clap::Command;
        use clap::CommandFactory;
        let mut app = $crate::App::command();
        if let Some(actions_model) = &$config.pipelines {
            // Group parent commands, ordered by first appearance.
            let mut group_cmds: Vec<(String, Command)> = Vec::new();
            for pipeline in &actions_model.pipelines {
                let mut dynamic_cmd = Command::new(pipeline.name.as_str()).about(&pipeline.about);
                for arg_def in &pipeline.args {
                    let clap_arg: Arg = arg_def.into();
                    dynamic_cmd = dynamic_cmd.arg(clap_arg);
                }
                if pipeline.uses_query() {
                    dynamic_cmd = dynamic_cmd.arg(
                        Arg::new("query")
                            .help("Query input (positional argument)")
                            .required(false),
                    );
                }
                match &pipeline.group {
                    Some(group) => {
                        if let Some((_, cmd)) = group_cmds.iter_mut().find(|(n, _)| n == group) {
                            *cmd = std::mem::take(cmd).subcommand(dynamic_cmd);
                        } else {
                            let about = $config
                                .groups
                                .iter()
                                .find(|c| &c.name == group)
                                .map(|c| c.about.clone())
                                .unwrap_or_default();
                            group_cmds.push((
                                group.clone(),
                                Command::new(group.as_str())
                                    .about(about)
                                    .subcommand(dynamic_cmd),
                            ));
                        }
                    }
                    None => app = app.subcommand(dynamic_cmd),
                }
            }
            for (_, cmd) in group_cmds {
                app = app.subcommand(cmd);
            }
        }
        app
    }};
}

/// Print custom colored help with grouped sections.
pub fn print_custom_help(app_builder: &Command, config: &AppConfig) {
    let empty: Vec<PipelineModel> = Vec::new();
    let pipelines: &[PipelineModel] = config
        .pipelines
        .as_ref()
        .map(|f| f.pipelines.as_slice())
        .unwrap_or(&empty);

    // Flat actions (no group), sorted by name.
    let mut actions: Vec<&PipelineModel> = pipelines.iter().filter(|p| p.group.is_none()).collect();
    actions.sort_by(|a, b| a.name.cmp(&b.name));

    // Groups in first-appearance order.
    let mut group_names: Vec<&str> = Vec::new();
    for p in pipelines {
        if let Some(g) = p.group.as_deref() {
            if !group_names.contains(&g) {
                group_names.push(g);
            }
        }
    }

    let max_len = actions
        .iter()
        .map(|p| p.name.len())
        .chain(group_names.iter().map(|g| g.len()))
        .chain(SYSTEM_COMMANDS.iter().map(|c| c.len()))
        .max()
        .unwrap_or(15)
        + 1;

    // Collect built-in pipeline names to distinguish custom actions.
    let builtin_names: HashSet<String> = default_pipelines()
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

    if !group_names.is_empty() {
        print_template!(
            OutputKind::Plain,
            "{header|bright_green|bold}",
            "header" => "Groups:"
        );

        for g in &group_names {
            let about = config
                .groups
                .iter()
                .find(|c| c.name == *g)
                .map(|c| c.about.clone())
                .unwrap_or_default();
            let formatted_name = format!("  {:<width$}", g, width = max_len);

            // Nested actions for the plugin (ExportContext::Groups).
            let mut group_actions: Vec<&PipelineModel> = pipelines
                .iter()
                .filter(|p| p.group.as_deref() == Some(g))
                .collect();
            group_actions.sort_by(|a, b| a.name.cmp(&b.name));

            let actions_json: Vec<serde_json::Value> = group_actions
                .iter()
                .map(|p| {
                    let is_custom = !builtin_names.contains(&p.name);
                    serde_json::json!({
                        "name": p.name,
                        "about": p.about,
                        "args": p.args,
                        "api": p.api,
                        "is_custom": is_custom,
                        "yaml_path": p.file_path.as_ref().map(|path| path.display().to_string()),
                    })
                })
                .collect();

            print_template!(
                ExportContext::Groups,
                OutputKind::Plain,
                "{formatted_name|cyan|bold} {about}",
                "formatted_name" => formatted_name,
                "name" => g,
                "about" => about,
                "actions" => actions_json,
            );
        }
    }

    print_template!(
        OutputKind::Plain,
        "\n{header|bright_green|bold}",
        "header" => "Actions:"
    );

    for pipeline in &actions {
        let formatted_name = format!("  {:<width$}", pipeline.name, width = max_len);
        let is_custom = !builtin_names.contains(&pipeline.name);

        print_template!(
            ExportContext::Actions,
            OutputKind::Plain,
            "{formatted_name|cyan|bold} {about}",
            "formatted_name" => formatted_name,
            "name" => &pipeline.name,
            "about" => &pipeline.about,
            "args" => &pipeline.args,
            "api" => &pipeline.api,
            "is_custom" => is_custom,
            "yaml_path" => pipeline.file_path.as_ref().map(|p| p.display().to_string()),
        );
    }

    print_template!(
        OutputKind::Plain,
        "\n{header|bright_green|bold}",
        "header" => "Commands:"
    );

    for cmd_name in SYSTEM_COMMANDS {
        if cmd_name != &"help" {
            if let Some(cmd) = app_builder.find_subcommand(cmd_name) {
                let formatted_name = format!("  {:<width$}", cmd.get_name(), width = max_len);

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

/// Print custom colored help for a single group's actions.
pub fn print_group_help(config: &AppConfig, group: &str) {
    let Some(pipelines) = config.pipelines.as_ref() else {
        return;
    };
    let about = config
        .groups
        .iter()
        .find(|c| c.name == group)
        .map(|c| c.about.clone())
        .unwrap_or_default();

    let mut actions: Vec<&PipelineModel> = pipelines
        .pipelines
        .iter()
        .filter(|p| p.group.as_deref() == Some(group))
        .collect();
    actions.sort_by(|a, b| a.name.cmp(&b.name));

    let max_len = actions.iter().map(|p| p.name.len()).max().unwrap_or(15) + 1;

    print_template!(
        OutputKind::Plain,
        "\n{group|bright_green|bold} - {about}\n\n{usage|bright_green|bold} {app_name_cli|cyan|bold} {group_cli|cyan|bold} {command|cyan}\n",
        "group" => group,
        "about" => about,
        "usage" => "Usage:",
        "app_name_cli" => app::app_name(),
        "group_cli" => group,
        "command" => "[ACTION]"
    );

    print_template!(
        OutputKind::Plain,
        "{header|bright_green|bold}",
        "header" => "Actions:"
    );

    for pipeline in &actions {
        let formatted_name = format!("  {:<width$}", pipeline.name, width = max_len);

        print_template!(
            ExportContext::Actions,
            OutputKind::Plain,
            "{formatted_name|cyan|bold} {about}",
            "formatted_name" => formatted_name,
            "name" => &pipeline.name,
            "about" => &pipeline.about,
            "args" => &pipeline.args,
            "api" => &pipeline.api,
            "is_custom" => true,
            "yaml_path" => pipeline.file_path.as_ref().map(|p| p.display().to_string()),
        );
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
}
