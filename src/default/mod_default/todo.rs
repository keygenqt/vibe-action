//! Default todo action template.

use std::path::PathBuf;

use crate::models::action::{ActionMode, ActionModel, ExpectMode};
use crate::models::flow::{FlowFormat, FlowModel};

/// Returns the default todo FlowModel.
pub fn default() -> FlowModel {
    FlowModel {
        name: "todo".into(),
        output: "tag_todo".into(),
        format: FlowFormat::Compact,
        about: "Find and document @todo markers in code".into(),
        path: PathBuf::from("mod/todo.yaml"),
        r#match: None,
        clipboard: false,
        args: vec![],
        actions: vec![ActionModel {
            tag: "tag_todo".into(),
            r#type: ActionMode::Cmd,
            expect: ExpectMode::String,
            r#match: None,
            confirm: false,
            action: "echo 'Coming soon...'".into(),
        }],
    }
}
