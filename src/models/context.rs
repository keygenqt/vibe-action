// //! Runtime context model for tag values.
// //! Stores resolved action results with type information.

// use serde::Deserialize;
// use serde::Serialize;

// /// A resolved tag value with its type.
// #[derive(Debug, Clone, PartialEq, Serialize)]
// pub enum ContextModel {
//     String(String),
//     List(Vec<String>),
// }

// impl std::fmt::Display for ContextModel {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             ContextModel::String(v) => write!(f, "{}", v),
//             ContextModel::List(items) => write!(f, "{}", items.join("\n")),
//         }
//     }
// }

// impl<'de> Deserialize<'de> for ContextModel {
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//     where
//         D: serde::Deserializer<'de>,
//     {
//         let s = String::deserialize(deserializer)?;
//         match s.as_str() {
//             "string" => Ok(ContextModel::String(String::new())),
//             "list" => Ok(ContextModel::List(Vec::new())),
//             _ => Err(serde::de::Error::custom(format!(
//                 "Unknown expect type: '{}'. Expected 'string' or 'list'.",
//                 s
//             ))),
//         }
//     }
// }
