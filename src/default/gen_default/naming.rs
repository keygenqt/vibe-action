//! Default naming action template.

use std::path::PathBuf;

use crate::models::action::{ActionMode, ActionModel, ExpectMode};
use crate::models::flow::FlowModel;

/// Returns the default naming FlowModel.
pub fn default() -> FlowModel {
    FlowModel {
        name: "naming".into(),
        about: "Generate name suggestions for variables, functions, or projects".into(),
        path: PathBuf::from("gen/naming.yaml"),
        args: vec![],
        trigger: ActionModel {
            tag: "naming".into(),
            r#type: ActionMode::Cmd,
            expect: ExpectMode::String,
            r#match: None,
            action: r#"
echo "Naming: coming soon..."
            "#
            .trim()
            .into(),
        },
        actions: vec![],
    }
}
