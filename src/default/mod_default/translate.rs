//! Default translate action template.

use std::path::PathBuf;

use crate::models::action::{ActionMode, ActionModel, ExpectMode};
use crate::models::flow::FlowModel;

/// Returns the default translate FlowModel.
pub fn default() -> FlowModel {
    FlowModel {
        name: "translate".into(),
        about: "Translate text or files to another language".into(),
        path: PathBuf::from("mod/translate.yaml"),
        args: vec![],
        trigger: ActionModel {
            tag: "translate".into(),
            r#type: ActionMode::Cmd,
            expect: ExpectMode::String,
            r#match: None,
            action: r#"
echo "Translate: coming soon..."
            "#
            .trim()
            .into(),
        },
        actions: vec![],
    }
}
