//! Runtime context model for tag values.
//! Stores resolved action results with type information.

/// A resolved tag value with its type.
#[derive(Debug, Clone)]
pub enum ContextModel {
    Void,
    Bool(bool),
    Number(f64),
    String(String),
    List(Vec<ContextModel>),
}

impl std::fmt::Display for ContextModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ContextModel::Void => write!(f, ""),
            ContextModel::Bool(v) => write!(f, "{}", v),
            ContextModel::Number(v) => write!(f, "{}", v),
            ContextModel::String(v) => write!(f, "{}", v),
            ContextModel::List(items) => {
                let strings: Vec<String> = items.iter().map(|i| i.to_string()).collect();
                write!(f, "{}", strings.join("\n"))
            }
        }
    }
}
