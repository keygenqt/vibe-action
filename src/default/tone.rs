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
You receive a text that may contain rude or aggressive language.
Transform it into a professional, calm, and polite equivalent.
Keep the original meaning. Keep the same language.
Output the transformed text.

[Examples]
Input: You are an idiot, do it faster. -> Please focus and speed up the process.
Input: 你真笨，快点做！ -> 请集中精力，加快工作进度。
Input: Меня заебал этот код. -> Меня расстраивает этот код, нужно его улучшить.

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
