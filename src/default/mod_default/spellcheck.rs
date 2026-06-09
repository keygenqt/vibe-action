//! Default spellcheck action template.

use std::path::PathBuf;

use crate::models::action::{ActionMode, ActionModel, ExpectMode};
use crate::models::flow::{FlowFormat, FlowModel};

/// Returns the default spellcheck FlowModel.
pub fn default() -> FlowModel {
    FlowModel {
        name: "spellcheck".into(),
        output: "tag_spellcheck".into(),
        format: FlowFormat::Compact,
        about: "Check and fix spelling in text or files".into(),
        path: PathBuf::from("mod/spellcheck.yaml"),
        r#match: None,
        clipboard: false,
        args: vec![],
        actions: vec![ActionModel {
            tag: "tag_spellcheck".into(),
            r#type: ActionMode::Cmd,
            expect: ExpectMode::String,
            r#match: None,
            confirm: false,
            action: "echo 'Coming soon...'".into(),
        }],
    }
}
