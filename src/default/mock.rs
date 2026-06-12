//! Default mock action template.

use crate::models::action::{ActionMode, ActionModel, ExpectMode};
use crate::models::arg::ArgActionModel;
use crate::models::flow::FlowModel;

/// Returns the default mock FlowModel.
pub fn default() -> FlowModel {
    FlowModel {
        name: "mock".into(),
        about: "Generate realistic mock data arrays (JSON, YAML, CSV) for testing".into(),
        check: None,
        clipboard: true,
        args: vec![
            ArgActionModel {
                name: "query".into(),
                short: Some('q'),
                expect: ExpectMode::String,
                help: Some("Describe the data you need (e.g., '5 users with id, name, and unique uuid')".into()),
                default: None,
                values: Vec::new(),
            },
            ArgActionModel {
                name: "format".into(),
                short: Some('f'),
                expect: ExpectMode::String,
                help: Some("Output format: json, yaml, csv (default: json)".into()),
                default: Some("json".into()),
                values: Vec::new(),
            },
        ],
        actions: vec![
            ActionModel {
                tag: "tag_mock".into(),
                r#type: ActionMode::Llm,
                expect: ExpectMode::String,
                check: None,
                confirm: false,
                action: r#"
[Task]
Generate a realistic mock data array based on the user's [Query].
Format the output strictly as valid raw {format} data.
Strict rule: Absolutely NO explanations, NO markdown formatting, NO backticks (```). Output ONLY the raw data structure.

[Query]
{query}
                "#
                .trim()
                .into(),
            },
        ],
    }
}
