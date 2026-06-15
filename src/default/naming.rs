//! Default naming action template.

use crate::models::action::{ActionMode, ActionModel, ExpectMode};
use crate::models::arg::ArgActionModel;
use crate::models::flow::FlowModel;

/// Returns the default naming FlowModel.
pub fn default() -> FlowModel {
    FlowModel {
        name: "naming".into(),
        about: "Generate code naming suggestions based on a description".into(),
        check: None,
        clipboard: false,
        args: vec![ArgActionModel {
            name: "query".into(),
            short: Some('q'),
            expect: ExpectMode::String,
            help: Some("Code component description (e.g., 'function to sort actions by dependency')".into()),
            default: None,
            values: Vec::new(),
        }],
        actions: vec![
            ActionModel {
                tag: "tag_name_suggestions".into(),
                r#type: ActionMode::Llm,
                expect: ExpectMode::List(Box::new(ExpectMode::String)),
                check: None,
                confirm: false,
                action: r#"
[Task]
Generate 5-10 high-quality, professional programming naming suggestions based on the description.
Strictly follow the programming language or style constraints requested in the description if specified.
Write strictly in English.
Output each suggestion on a new line.

[Query]
{query}
                "#
                .trim()
                .into(),
            },
            ActionModel {
                tag: "tag_naming".into(),
                r#type: ActionMode::Value,
                expect: ExpectMode::String,
                check: None,
                confirm: false,
                action: "{tag_name_suggestions|join:uniq}".into(),
            },
        ],
    }
}
