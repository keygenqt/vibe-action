//! Default find action template.

use std::path::PathBuf;

use crate::models::action::{ActionMode, ActionModel, ExpectMode};
use crate::models::flow::FlowModel;

/// Returns the default find FlowModel.
pub fn default() -> FlowModel {
    FlowModel {
        name: "find".into(),
        about: "Fuzzy file finder with AI-powered search".into(),
        path: PathBuf::from("act/find.yaml"),
        args: vec![],
        trigger: ActionModel {
            tag: "find".into(),
            r#type: ActionMode::Cmd,
            expect: ExpectMode::String,
            r#match: None,
            action: r#"
echo "Find: coming soon..."
            "#
            .trim()
            .into(),
        },
        actions: vec![],
    }
}
