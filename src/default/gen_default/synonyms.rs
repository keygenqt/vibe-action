//! Default synonyms action template.

use std::path::PathBuf;

use crate::models::action::{ActionMode, ActionModel, ExpectMode};
use crate::models::flow::FlowModel;

/// Returns the default synonyms FlowModel.
pub fn default() -> FlowModel {
    FlowModel {
        name: "synonyms".into(),
        about: "Find synonyms for a given word".into(),
        path: PathBuf::from("gen/synonyms.yaml"),
        args: vec![],
        trigger: ActionModel {
            tag: "synonyms".into(),
            r#type: ActionMode::Cmd,
            expect: ExpectMode::String,
            r#match: None,
            action: r#"
echo "Synonyms: coming soon..."
            "#
            .trim()
            .into(),
        },
        actions: vec![],
    }
}
