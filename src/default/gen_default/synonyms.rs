//! Default synonyms action template.

use std::path::PathBuf;

use crate::models::action::{ActionMode, ActionModel, ExpectMode};
use crate::models::arg::ArgActionModel;
use crate::models::flow::{FlowFormat, FlowModel};

/// Returns the default synonyms FlowModel.
pub fn default() -> FlowModel {
    FlowModel {
        name: "synonyms".into(),
        output: "tag_synonyms".into(),
        format: FlowFormat::Compact,
        about: "Find programming/technical synonyms for a word".into(),
        path: PathBuf::from("gen/synonyms.yaml"),
        r#match: None,
        clipboard: false,
        args: vec![ArgActionModel {
            name: "word".into(),
            short: Some('w'),
            expect: ExpectMode::String,
            help: Some("Word to find synonyms for".into()),
            default: None,
            values: Vec::new(),
        }],
        actions: vec![
            ActionModel {
                tag: "tag_synonyms_list".into(),
                r#type: ActionMode::Llm,
                expect: ExpectMode::String,
                r#match: None,
                confirm: false,
                action: r#"
[Task]
Find 5-10 programming or technical synonyms for the word below.
Write strictly in English. Separate synonyms with commas.
Do not add introductory text, quotes, or explanations.

[Word]
{word}

[Result]
<synonyms>
                "#
                .trim()
                .into(),
            },
            ActionModel {
                tag: "tag_synonyms".into(),
                r#type: ActionMode::Value,
                expect: ExpectMode::String,
                r#match: None,
                confirm: false,
                action: "{tag_synonyms_list}".into(),
            },
        ],
    }
}
