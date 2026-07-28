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

/// IDE plugin integration metadata.
/// Maps CLI argument names to a specific source (e.g., editor selection or clipboard).
/// This metadata is ignored by the CLI runtime and is intended solely for IDE plugins.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FlowApiModel {
    /// Map of argument names to their IDE source.
    #[serde(default)]
    pub args: HashMap<String, ApiSource>,
}
