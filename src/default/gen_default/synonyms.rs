//! Default synonyms action template.

use std::path::PathBuf;

use crate::models::action::{ActionMode, ActionModel, ExpectMode};
use crate::models::arg::ArgActionModel;
use crate::models::flow::{FlowMode, FlowModel};

/// Returns the default synonyms FlowModel.
pub fn default() -> FlowModel {
    FlowModel {
        name: "synonyms".into(),
        mode: FlowMode::Output,
        about: "Find programming/technical synonyms for a word".into(),
        path: PathBuf::from("gen/synonyms.yaml"),
        args: vec![ArgActionModel {
            name: "word".into(),
            short: Some('w'),
            expect: ExpectMode::String,
            help: Some("Word to find synonyms for".into()),
            required: true,
            values: Vec::new(),
        }],
        trigger: ActionModel {
            tag: "synonyms".into(),
            r#type: ActionMode::Cmd,
            expect: ExpectMode::String,
            r#match: None,
            action: "{synonyms_list}".into(),
        },
        actions: vec![ActionModel {
            tag: "synonyms_list".into(),
            r#type: ActionMode::Llm,
            expect: ExpectMode::String,
            r#match: None,
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
        }],
    }
}
