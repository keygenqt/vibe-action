//! Default tone action template.

use crate::models::action::{ActionMode, ActionModel, ExpectMode};
use crate::models::arg::ArgActionModel;
use crate::models::flow::FlowModel;

/// Returns the default tone FlowModel.
pub fn default() -> FlowModel {
    FlowModel {
        name: "tone".into(),
        about: "Change the tone of a text based on your instructions".into(),
        check: None,
        clipboard: true,
        args: vec![ArgActionModel {
            name: "query".into(),
            short: Some('q'),
            expect: ExpectMode::String,
            help: Some("The text to rewrite along with tone instructions (e.g., 'make it formal: hi friend')".into()),
            default: None,
            values: Vec::new(),
        }],
        actions: vec![
            ActionModel {
                tag: "tag_rewritten_text".into(),
                r#type: ActionMode::Llm,
                expect: ExpectMode::String,
                check: None,
                confirm: false,
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
{query}
                "#
                .trim()
                .into(),
            },
            ActionModel {
                tag: "tag_tone".into(),
                r#type: ActionMode::Value,
                expect: ExpectMode::String,
                check: None,
                confirm: false,
                action: "{tag_rewritten_text}".into(),
            },
        ],
    }
}
