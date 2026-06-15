//! Default extract action template.

use crate::models::action::{ActionMode, ActionModel, ExpectMode};
use crate::models::arg::ArgActionModel;
use crate::models::flow::FlowModel;

/// Returns the default extract FlowModel.
pub fn default() -> FlowModel {
    FlowModel {
        name: "extract".into(),
        about: "Extract structured data or matching lines from text and logs".into(),
        check: None,
        clipboard: false,
        args: vec![
            ArgActionModel {
                name: "file".into(),
                short: Some('f'),
                expect: ExpectMode::String,
                help: Some("Path to the log or text file".into()),
                default: None,
                values: Vec::new(),
            },
            ArgActionModel {
                name: "query".into(),
                short: Some('q'),
                expect: ExpectMode::String,
                help: Some("Extraction criteria (e.g., 'find all errors')".into()),
                default: None,
                values: Vec::new(),
            },
        ],
        actions: vec![
            ActionModel {
                tag: "tag_lines".into(),
                r#type: ActionMode::Cmd,
                expect: ExpectMode::List(Box::new(ExpectMode::String)),
                check: None,
                confirm: false,
                action: "cat {file}".into(),
            },
            ActionModel {
                tag: "tag_content".into(),
                r#type: ActionMode::Llm,
                expect: ExpectMode::String,
                check: None,
                confirm: false,
                action: r#"
[Task]
Read the log line and the search query.
If the line matches the query — output the EXACT line unchanged.
If the line does not match — output only a single dash: "-"
Do NOT skip lines. Process every line.

[Query]
{query}

[Line]
{tag_lines}
                "#
                .trim()
                .into(),
            },
            ActionModel {
                tag: "tag_clean".into(),
                r#type: ActionMode::Value,
                expect: ExpectMode::String,
                check: None,
                confirm: false,
                action: "{tag_content|trim:-}".into(),
            },
            ActionModel {
                tag: "tag_extract".into(),
                r#type: ActionMode::Value,
                expect: ExpectMode::String,
                check: None,
                confirm: false,
                action: "{tag_clean|join:uniq}".into(),
            },
        ],
    }
}
