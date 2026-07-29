//! Action argument model.
//! Defines CLI arguments for YAML actions.

use std::collections::HashMap;

use serde::Deserialize;
use serde::Serialize;

/// Specifies the source from which the IDE plugin should retrieve the argument value.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ApiSource {
    /// Use the currently selected text in the editor.
    Selection,
    /// Use the contents of the system clipboard.
    Clipboard,
}

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

/// IDE plugin integration metadata.
/// Maps CLI argument names to a specific source (e.g., editor selection or clipboard)
/// and defines where to route the final output.
/// This metadata is ignored by the CLI runtime and is intended solely for IDE plugins.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FlowApiModel {
    /// Map of argument names to their IDE source.
    #[serde(default)]
    pub args: HashMap<String, ApiSource>,

    /// Defines where the IDE plugin should apply the final result.
    #[serde(default)]
    pub output: ApiTarget,
}
