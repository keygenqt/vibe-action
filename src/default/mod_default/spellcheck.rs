//! Default spellcheck action template.

use std::path::PathBuf;

use crate::models::action::{ActionMode, ActionModel, ExpectMode};
use crate::models::flow::FlowModel;

/// Returns the default spellcheck FlowModel.
pub fn default() -> FlowModel {
    FlowModel {
        name: "spellcheck".into(),
        about: "Check and fix spelling in text or files".into(),
        path: PathBuf::from("mod/spellcheck.yaml"),
        args: vec![],
        trigger: ActionModel {
            tag: "spellcheck".into(),
            r#type: ActionMode::Cmd,
            expect: ExpectMode::String,
            r#match: None,
            action: r#"
echo "Spellcheck: coming soon..."
            "#
            .trim()
            .into(),
        },
        actions: vec![],
    }
}
