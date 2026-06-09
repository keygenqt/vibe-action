//! Default extract action template.

use std::path::PathBuf;

use crate::models::action::{ActionMode, ActionModel, ExpectMode};
use crate::models::flow::{FlowFormat, FlowModel};

/// Returns the default extract FlowModel.
pub fn default() -> FlowModel {
    FlowModel {
        name: "extract".into(),
        output: "tag_extract".into(),
        format: FlowFormat::Compact,
        about: "Extract structured data from text or logs".into(),
        path: PathBuf::from("act/extract.yaml"),
        r#match: None,
        clipboard: false,
        args: vec![],
        actions: vec![ActionModel {
            tag: "tag_extract".into(),
            r#type: ActionMode::Cmd,
            expect: ExpectMode::String,
            r#match: None,
            confirm: false,
            action: "echo 'Coming soon...'".into(),
        }],
    }
}
