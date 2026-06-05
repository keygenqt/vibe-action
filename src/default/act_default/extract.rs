//! Default extract action template.

use std::path::PathBuf;

use crate::models::action::{ActionMode, ActionModel, ExpectMode};
use crate::models::flow::FlowModel;

/// Returns the default extract FlowModel.
pub fn default() -> FlowModel {
    FlowModel {
        name: "extract".into(),
        about: "Extract structured data from text or logs".into(),
        path: PathBuf::from("act/extract.yaml"),
        args: vec![],
        trigger: ActionModel {
            tag: "extract".into(),
            r#type: ActionMode::Cmd,
            expect: ExpectMode::String,
            r#match: None,
            action: r#"
echo "Extracted: coming soon..."
            "#
            .trim()
            .into(),
        },
        actions: vec![],
    }
}
