//! Action argument model.
//! Defines CLI arguments for YAML actions.

use serde::{Deserialize, Serialize};

use crate::models::{action::ExpectMode, arg_value::ArgActionValue};

/// CLI argument definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArgActionModel {
    /// Argument name (used as --name and {name} tag).
    pub name: String,
    /// Short flag (e.g. -p).
    #[serde(default)]
    pub short: Option<char>,
    /// Expected type.
    pub expect: ExpectMode,
    /// Help text.
    #[serde(default)]
    pub help: Option<String>,
    /// Whether the argument is required (default: false).
    #[serde(default)]
    pub required: bool,
    /// Resolved values from CLI input (not serialized).
    #[serde(skip, default)]
    pub values: Vec<ArgActionValue>,
}

impl ArgActionModel {
    /// Fill values from CLI matches. Handles multiple values (e.g., for list types).
    pub fn resolve_values(&mut self, matches: &clap::ArgMatches) {
        if let Some(values) = matches.get_many::<String>(&self.name) {
            self.values = values
                .map(|v| ArgActionValue::new_with_check(&self.name, v))
                .collect();
        }
    }
}

/// Convert ArgActionModel into a clap::Arg for CLI building.
impl From<&ArgActionModel> for clap::Arg {
    fn from(model: &ArgActionModel) -> Self {
        let leaked_name: &'static str = &*Box::leak(model.name.clone().into_boxed_str());
        let mut arg = clap::Arg::new(leaked_name)
            .long(leaked_name)
            .required(model.required);
        if let Some(help) = &model.help {
            let leaked_help: &'static str = &*Box::leak(help.clone().into_boxed_str());
            arg = arg.help(leaked_help);
        }
        if let Some(c) = model.short {
            arg = arg.short(c);
        }
        match &model.expect {
            ExpectMode::Bool => arg.action(clap::ArgAction::SetTrue),
            _ => arg.value_parser(clap::value_parser!(String)),
        }
    }
}

/// Display ExpectMode as a human-readable string.
impl std::fmt::Display for ExpectMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExpectMode::Void => write!(f, "void"),
            ExpectMode::Bool => write!(f, "bool"),
            ExpectMode::Number => write!(f, "number"),
            ExpectMode::String => write!(f, "string"),
            ExpectMode::Json => write!(f, "json"),
            ExpectMode::List(inner) => write!(f, "list<{}>", inner),
        }
    }
}
