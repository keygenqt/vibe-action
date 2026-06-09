//! Default translate action template.

use std::path::PathBuf;

use crate::models::action::{ActionMode, ActionModel, ExpectMode};
use crate::models::flow::{FlowFormat, FlowModel};

/// Returns the default translate FlowModel.
pub fn default() -> FlowModel {
    FlowModel {
        name: "translate".into(),
        output: "tag_translate".into(),
        format: FlowFormat::Compact,
        about: "Translate text or files to another language".into(),
        path: PathBuf::from("mod/translate.yaml"),
        r#match: None,
        clipboard: false,
        args: vec![],
        actions: vec![ActionModel {
            tag: "tag_translate".into(),
            r#type: ActionMode::Cmd,
            expect: ExpectMode::String,
            r#match: None,
            confirm: false,
            action: "echo 'Coming soon...'".into(),
        }],
    }
}
