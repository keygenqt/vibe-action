//! Default synonyms action template.

use crate::models::action::{ActionMode, ActionModel, ExpectMode};
use crate::models::arg::ArgActionModel;
use crate::models::flow::FlowModel;

/// Returns the default synonyms FlowModel.
pub fn default() -> FlowModel {
    FlowModel {
        name: "synonyms".into(),
        about: "Find programming/technical synonyms for a word".into(),
        check: None,
        clipboard: false,
        args: vec![ArgActionModel {
            name: "query".into(),
            short: Some('q'),
            expect: ExpectMode::String,
            help: Some("Word or concept to find technical synonyms for".into()),
            default: None,
            values: Vec::new(),
        }],
        actions: vec![
            ActionModel {
                tag: "tag_synonyms_list".into(),
                r#type: ActionMode::Llm,
                expect: ExpectMode::List(Box::new(ExpectMode::String)), // ИСПРАВЛЕНО
                check: None,
                confirm: false,
                action: r#"
[Task]
Give me 5 professional programming terms or alternatives for the concept below.
Write each option strictly on a new line without numbering.

[Concept]
{query}
"#
                .trim()
                .into(),
            },
            ActionModel {
                tag: "tag_synonyms".into(),
                r#type: ActionMode::Value,
                expect: ExpectMode::String,
                check: None,
                confirm: false,
                action: "{tag_synonyms_list|join}".into(),
            },
        ],
    }
}
