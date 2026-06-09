//! Default find action template.

use std::path::PathBuf;

use crate::models::action::{ActionMode, ActionModel, ExpectMode};
use crate::models::flow::{FlowFormat, FlowModel};

/// Returns the default find FlowModel.
pub fn default() -> FlowModel {
    FlowModel {
        name: "find".into(),
        output: "tag_find".into(),
        format: FlowFormat::Compact,
        about: "Fuzzy file finder with AI-powered search".into(),
        path: PathBuf::from("act/find.yaml"),
        r#match: None,
        clipboard: false,
        args: vec![],
        actions: vec![ActionModel {
            tag: "tag_find".into(),
            r#type: ActionMode::Cmd,
            expect: ExpectMode::String,
            r#match: None,
            confirm: false,
            action: "echo 'Coming soon...'".into(),
        }],
    }
}
