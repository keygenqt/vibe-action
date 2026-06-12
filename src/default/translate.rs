//! Default translate action template.

use crate::models::action::{ActionMode, ActionModel, ExpectMode};
use crate::models::arg::ArgActionModel;
use crate::models::flow::FlowModel;

/// Returns the default translate FlowModel.
pub fn default() -> FlowModel {
    FlowModel {
        name: "translate".into(),
        about: "Translate text or files to another language".into(),
        check: None,
        clipboard: true,
        args: vec![
            ArgActionModel {
                name: "text".into(),
                short: Some('t'),
                expect: ExpectMode::String,
                help: Some("Text to translate".into()),
                default: Some(String::new()),
                values: Vec::new(),
            },
            ArgActionModel {
                name: "file".into(),
                short: Some('f'),
                expect: ExpectMode::String,
                help: Some("File to translate".into()),
                default: Some(String::new()),
                values: Vec::new(),
            },
            ArgActionModel {
                name: "to".into(),
                short: Some('l'),
                expect: ExpectMode::String,
                help: Some("Target language (e.g., Russian, English, Chinese)".into()),
                default: None,
                values: Vec::new(),
            },
        ],
        actions: vec![
            ActionModel {
                tag: "tag_content".into(),
                r#type: ActionMode::Cmd,
                expect: ExpectMode::String,
                check: Some(".+".into()),
                confirm: false,
                action: r#"
if [ -n "{file}" ]; then
  cat "{file}"
elif [ -n "{text}" ]; then
  echo "{text}"
fi
                "#
                .trim()
                .into(),
            },
            ActionModel {
                tag: "tag_translated".into(),
                r#type: ActionMode::Llm,
                expect: ExpectMode::String,
                check: None,
                confirm: false,
                action: r#"
[Task]
Translate the entire [Text] to the language specified in [User].
Do not translate proper names, code, or comments inside code blocks.
Reply with the full translated text.

[User]
{to}

[Text]
{tag_content}
                "#
                .trim()
                .into(),
            },
            ActionModel {
                tag: "tag_translate".into(),
                r#type: ActionMode::Value,
                expect: ExpectMode::String,
                check: None,
                confirm: false,
                action: "{tag_translated}".into(),
            },
        ],
    }
}
