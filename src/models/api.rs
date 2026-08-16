//! IDE plugin integration metadata.
//! Defines the input source for the query and the output target for the result.
//! This metadata is ignored by the CLI runtime and is intended solely for IDE plugins.

use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;

/// Specifies the target where the IDE plugin should apply the action result.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ApiTarget {
    /// Replace the currently selected text in the editor.
    Replace,
    /// Copy the result to the system clipboard.
    Clipboard,
    /// Display the result in a dialog.
    Dialog,
}

impl Default for ApiTarget {
    fn default() -> Self {
        ApiTarget::Replace
    }
}

/// Specifies the input source for the IDE plugin.
/// Represents the {query} tag and its modifier (e.g., query|file_path).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ApiInput(String);

impl Default for ApiInput {
    fn default() -> Self {
        ApiInput("query".to_string())
    }
}

impl ApiInput {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// IDE plugin integration metadata.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FlowApiModel {
    /// Defines the input source for the {query} tag.
    /// Defaults to "query" (editor selection).
    #[serde(default)]
    pub input: Option<ApiInput>,
    /// Defines where the IDE plugin should apply the final result.
    #[serde(default)]
    pub output: ApiTarget,
    /// Optional additional arguments mapped to query types.
    /// Each key is an argument name, each value is a query type (e.g., "query|file_path").
    /// Used for actions that need multiple inputs beyond the main {query} positional arg.
    #[serde(default)]
    pub args: HashMap<String, String>,
}
