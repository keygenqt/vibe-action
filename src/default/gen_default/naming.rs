//! Default naming action template.

use std::path::PathBuf;

use crate::models::action::{ActionMode, ActionModel, ExpectMode};
use crate::models::arg::ArgActionModel;
use crate::models::flow::{FlowFormat, FlowModel};

/// Returns the default naming FlowModel.
pub fn default() -> FlowModel {
    FlowModel {
        name: "naming".into(),
        output: "tag_naming".into(),
        format: FlowFormat::Compact,
        about: "Generate code naming suggestions based on a description".into(),
        path: PathBuf::from("gen/naming.yaml"),
        r#match: None,
        clipboard: false,
        args: vec![ArgActionModel {
            name: "description".into(),
            short: Some('d'),
            expect: ExpectMode::String,
            help: Some("Description of what you need to name (variable, function, etc.)".into()),
            required: true,
            values: Vec::new(),
        }],
        actions: vec![
            ActionModel {
                tag: "tag_name_suggestions".into(),
                r#type: ActionMode::Llm,
                expect: ExpectMode::String,
                r#match: None,
                confirm: false,
                action: r#"
[Task]
Generate 5-10 high-quality, professional programming naming suggestions based on the description.
Strictly follow the programming language or style constraints requested in the description if specified.
Write strictly in English. Separate suggestions with commas.
Do not add introductory text, quotes, or explanations.

[Description]
{description}

[Result]
<names>
                "#
                .trim()
                .into(),
            },
            ActionModel {
                tag: "tag_naming".into(),
                r#type: ActionMode::Cmd,
                expect: ExpectMode::String,
                r#match: None,
                confirm: false,
                action: "echo '{tag_name_suggestions}'".into(),
            },
        ],
    }
}
