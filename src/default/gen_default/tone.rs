//! Default tone action template.

use std::path::PathBuf;

use crate::models::action::{ActionMode, ActionModel, ExpectMode};
use crate::models::flow::FlowModel;

/// Returns the default tone FlowModel.
pub fn default() -> FlowModel {
    FlowModel {
        name: "tone".into(),
        about: "Change the tone of a text (friendly, formal, casual, etc.)".into(),
        path: PathBuf::from("gen/tone.yaml"),
        args: vec![],
        trigger: ActionModel {
            tag: "tone".into(),
            r#type: ActionMode::Cmd,
            expect: ExpectMode::String,
            r#match: None,
            action: r#"
echo "Tone: coming soon..."
            "#
            .trim()
            .into(),
        },
        actions: vec![],
    }
}
