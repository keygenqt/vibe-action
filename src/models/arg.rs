//! Action argument model.
//! Defines CLI arguments for YAML actions.

use serde::Deserialize;
use serde::Serialize;

/// Expected result type.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ArgExpect {
    /// Boolean (true/false).
    Bool,
    /// Integer or float.
    Number,
    /// Text.
    String,
    /// Image file path or base64 string.
    Image,
}

impl std::fmt::Display for ArgExpect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ArgExpect::Bool => write!(f, "bool"),
            ArgExpect::Number => write!(f, "number"),
            ArgExpect::String => write!(f, "string"),
            ArgExpect::Image => write!(f, "image"),
        }
    }
}

/// CLI argument definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArgActionModel {
    /// Argument name (used as --name and {name} tag).
    pub name: String,
    /// Short flag (e.g. -p).
    #[serde(default)]
    pub short: Option<char>,
    /// Expected type.
    pub expect: ArgExpect,
    /// Help text.
    #[serde(default)]
    pub help: Option<String>,
    /// Default value. If set, the argument is optional and uses this value when not provided.
    #[serde(default)]
    pub default: Option<String>,
}

/// Convert ArgActionModel into a clap::Arg for CLI building.
impl From<&ArgActionModel> for clap::Arg {
    fn from(model: &ArgActionModel) -> Self {
        let leaked_name: &'static str = &*Box::leak(model.name.clone().into_boxed_str());
        let mut arg = clap::Arg::new(leaked_name)
            .long(leaked_name)
            .required(model.default.is_none());
        if let Some(help) = &model.help {
            let leaked_help: &'static str = &*Box::leak(help.clone().into_boxed_str());
            arg = arg.help(leaked_help);
        }
        if let Some(c) = model.short {
            arg = arg.short(c);
        }
        match &model.expect {
            ArgExpect::Bool => arg.action(clap::ArgAction::SetTrue),
            _ => arg.value_parser(clap::value_parser!(String)),
        }
    }
}
