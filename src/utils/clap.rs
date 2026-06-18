/// Builds the full hierarchical CLI command tree including dynamic YAML actions.
///
/// This macro dynamically extends the `action` subcommand with commands
/// loaded from the user's YAML action files at runtime.
#[macro_export]
macro_rules! build_app {
    ($config:expr) => {{
        use clap::{Arg, Command, CommandFactory};
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
