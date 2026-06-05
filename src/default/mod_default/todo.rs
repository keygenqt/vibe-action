//! Default todo action template.

use std::path::PathBuf;

use crate::models::action::{ActionMode, ActionModel, ExpectMode};
use crate::models::flow::FlowModel;

/// Returns the default todo FlowModel.
pub fn default() -> FlowModel {
    FlowModel {
        name: "todo".into(),
        about: "Find and document @todo markers in code".into(),
        path: PathBuf::from("mod/todo.yaml"),
        args: vec![],
        trigger: ActionModel {
            tag: "todo".into(),
            r#type: ActionMode::Cmd,
            expect: ExpectMode::String,
            r#match: None,
            action: r#"
echo "Todo: coming soon..."
            "#
            .trim()
            .into(),
        },
        actions: vec![],
    }
}
