//! Default translate action template.

use std::path::PathBuf;

use crate::models::action::{ActionMode, ActionModel, ExpectMode};
use crate::models::flow::{FlowMode, FlowModel};

/// Returns the default translate FlowModel.
pub fn default() -> FlowModel {
    FlowModel {
        name: "translate".into(),
        mode: FlowMode::Output,
        about: "Translate text or files to another language".into(),
        path: PathBuf::from("mod/translate.yaml"),
        args: vec![],
        trigger: ActionModel {
            tag: "translate".into(),
            r#type: ActionMode::Cmd,
            expect: ExpectMode::String,
            r#match: None,
            action: r#"
Coming soon...
            "#
            .trim()
            .into(),
        },
        actions: vec![],
    }
}
