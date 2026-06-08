//! Default tone action template.

use std::path::PathBuf;

use crate::models::action::{ActionMode, ActionModel, ExpectMode};
use crate::models::arg::ArgActionModel;
use crate::models::flow::{FlowMode, FlowModel};

/// Returns the default tone FlowModel.
pub fn default() -> FlowModel {
    FlowModel {
        name: "tone".into(),
        mode: FlowMode::Output,
        about: "Change the tone of a text based on your instructions".into(),
        path: PathBuf::from("gen/tone.yaml"),
        args: vec![ArgActionModel {
            name: "text".into(),
            short: Some('t'),
            expect: ExpectMode::String,
            help: Some("The text to rewrite along with tone instructions (e.g., 'make it formal: hi friend')".into()),
            required: true,
            values: Vec::new(),
        }],
        trigger: ActionModel {
            tag: "tone".into(),
            r#type: ActionMode::Cmd,
            expect: ExpectMode::String,
            r#match: None,
            action: "{rewritten_text}".into(),
        },
        actions: vec![ActionModel {
            tag: "rewritten_text".into(),
            r#type: ActionMode::Llm,
            expect: ExpectMode::String,
            r#match: None,
            action: r#"
[Task]
Rewrite the input text to make its tone professional, calm, and polite.
NEVER repeat rude, obscene, or aggressive words.
CRITICAL: Reply strictly in the EXACT SAME LANGUAGE as the input text.

[Examples]
Input: You are an idiot, do it faster. -> Result: Please focus and speed up the process.
Input: 你真笨，快点做！ -> Result: 请集中精力，加快工作进度。
Input: Это что за говнокод? Перепиши нормально! -> Result: Пожалуйста, проведите рефакторинг данного участка кода.

[Input]
{text}

[Result]
<rewritten_text>
        "#
            .trim()
            .into(),
        }],
    }
}
