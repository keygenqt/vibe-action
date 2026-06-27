//! Action argument model.
//! Defines CLI arguments for YAML actions.

use serde::Deserialize;
use serde::Serialize;

/// Expected result type.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ArgInput {
    String,
    Bool,
    Number,
    Path,
    List(Box<ArgInput>),
}

impl std::fmt::Display for ArgInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fn fmt_inner(input: &ArgInput, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match input {
                ArgInput::String => write!(f, "string"),
                ArgInput::Bool => write!(f, "bool"),
                ArgInput::Number => write!(f, "number"),
                ArgInput::Path => write!(f, "path"),
                ArgInput::List(inner) => write!(f, "list<{}>", inner),
            }
        }
        fmt_inner(self, f)
    }
}

impl<'de> Deserialize<'de> for ArgInput {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        if let Some(inner_str) = s.strip_prefix("list<").and_then(|s| s.strip_suffix('>')) {
            let inner = ArgInput::deserialize(serde::de::value::StrDeserializer::<D::Error>::new(
                inner_str,
            ))?;
            return Ok(ArgInput::List(Box::new(inner)));
        }
        match s.as_str() {
            "string" => Ok(ArgInput::String),
            "bool" => Ok(ArgInput::Bool),
            "number" => Ok(ArgInput::Number),
            "path" => Ok(ArgInput::Path),
            _ => Err(serde::de::Error::custom(format!(
                "Unknown input type: {}. Expected: string, bool, number, path, list<string>.",
                s
            ))),
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
    /// Input type.
    pub input: ArgInput,
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
        match &model.input {
            ArgInput::Bool => arg.action(clap::ArgAction::SetTrue),
            ArgInput::Number => arg.value_parser(clap::value_parser!(f64)),
            ArgInput::Path => arg.value_parser(clap::value_parser!(String)),
            ArgInput::String => arg.value_parser(clap::value_parser!(String)),
            ArgInput::List(_) => arg
                .value_parser(clap::value_parser!(String))
                .value_delimiter(','),
        }
    }
}
