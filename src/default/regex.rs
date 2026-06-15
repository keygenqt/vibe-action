//! Default regex action template.

use crate::models::action::{ActionMode, ActionModel, ExpectMode};
use crate::models::arg::ArgActionModel;
use crate::models::flow::FlowModel;

/// Returns the default regex FlowModel.
pub fn default() -> FlowModel {
    FlowModel {
        name: "regex".into(),
        about: "Generate a regular expression pattern based on your description".into(),
        check: None,
        clipboard: true,
        args: vec![
            ArgActionModel {
                name: "query".into(),
                short: Some('q'),
                expect: ExpectMode::String,
                help: Some(
                    "What the regex should match (e.g., 'extract domain from email')".into(),
                ),
                default: None,
                values: Vec::new(),
            },
            ArgActionModel {
                name: "example".into(),
                short: Some('e'),
                expect: ExpectMode::String,
                help: Some("Optional example string to test the pattern against".into()),
                default: Some(String::new()),
                values: Vec::new(),
            },
        ],
        actions: vec![ActionModel {
            tag: "tag_regex".into(),
            r#type: ActionMode::Llm,
            expect: ExpectMode::String,
            check: None,
            confirm: false,
            action: r#"
[Task]
Generate a valid regular expression pattern based on the user's [Query].
If an [Example] string is provided, ensure the generated regex matches it correctly.
Output exactly 1 line.

[Query]
{query}

[Example]
{example}
                "#
            .trim()
            .into(),
        }],
    }
}
