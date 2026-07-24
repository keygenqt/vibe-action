use clap::Command;
use colored::Colorize;

use crate::{output::output::OutputKind, print_template, print_text, utils::app};

/// System commands displayed in a separate section.
pub const SYSTEM_COMMANDS: &[&str] = &["clean", "status", "bench"];

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
pub fn print_custom_help(app_builder: &Command) {
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

    print_text!(OutputKind::Plain, "{}", app::app_about());
    print_template!(
        OutputKind::Plain,
        "{usage} {app_name} {command}\n",
        "usage" => "Usage:".bright_green().bold(),
        "app_name" => app::app_name().cyan().bold(),
        "command" => "[COMMAND]".cyan()
    );

    print_text!(OutputKind::Plain, "{}", "Actions:".bright_green().bold());
    for sub in &actions {
        let name = sub.get_name();
        let about = sub.get_about().unwrap_or_default();

        let formatted_name = format!("  {:<width$}", name.cyan().bold(), width = max_len);

        print_template!(
            ExportContext::Actions,
            OutputKind::Plain,
            "{name} {about}",
            "name" => formatted_name,
            "about" => about
        );
    }

    print_text!(OutputKind::Plain, "\n{}", "Commands:".bright_green().bold());
    for cmd_name in SYSTEM_COMMANDS {
        if cmd_name != &"help" {
            if let Some(cmd) = app_builder.find_subcommand(cmd_name) {
                let formatted_name = format!("  {:<15}", cmd.get_name().cyan().bold());

                print_template!(
                    OutputKind::Plain,
                    "{name} {about}",
                    "name" => formatted_name,
                    "about" => cmd.get_about().unwrap_or_default()
                );
            }
        }
    }

    print_text!(OutputKind::Plain, "\n{}", "Options:".bright_green().bold());
    print_template!(
        OutputKind::Plain,
        "{flag} {desc}",
        "flag" => format!("  {:<15}", "-h, --help".cyan().bold()),
        "desc" => "Print help"
    );
    print_template!(
        OutputKind::Plain,
        "{flag} {desc}",
        "flag" => format!("  {:<15}", "-V, --version".cyan().bold()),
        "desc" => "Print version"
    );
}
