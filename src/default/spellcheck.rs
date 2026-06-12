//! Default spellcheck action template.

use crate::models::action::{ActionMode, ActionModel, ExpectMode};
use crate::models::arg::ArgActionModel;
use crate::models::flow::FlowModel;

/// Returns the default spellcheck FlowModel.
pub fn default() -> FlowModel {
    FlowModel {
        name: "spellcheck".into(),
        about: "Check and fix spelling in text or files".into(),
        check: None,
        clipboard: true,
        args: vec![
            ArgActionModel {
                name: "text".into(),
                short: Some('t'),
                expect: ExpectMode::String,
                help: Some("Text to check".into()),
                default: Some(String::new()),
                values: Vec::new(),
            },
            ArgActionModel {
                name: "file".into(),
                short: Some('f'),
                expect: ExpectMode::String,
                help: Some("File to check".into()),
                default: Some(String::new()),
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
                tag: "tag_check_errors".into(),
                r#type: ActionMode::Llm,
                expect: ExpectMode::String,
                check: None,
                confirm: false,
                action: r#"
[Task]
Analyze the text below for typos, spelling mistakes, and grammar errors.
If errors exist, reply strictly with "DIRTY". If no errors exist, reply strictly with "CLEAR"

[Text]
{tag_content}
                "#
                .trim()
                .into(),
            },
            ActionModel {
                tag: "tag_check_errors_filter".into(),
                r#type: ActionMode::Cmd,
                expect: ExpectMode::String,
                check: None,
                confirm: false,
                action: r#"
if [ "{tag_check_errors}" = "DIRTY" ]; then
  echo "{tag_content}"
else
  echo "No errors found."
fi
                "#
                .trim()
                .into(),
            },
            ActionModel {
                tag: "tag_checked".into(),
                r#type: ActionMode::Llm,
                expect: ExpectMode::String,
                check: None,
                confirm: false,
                action: r#"
[Task]
Fix all spelling mistakes, typos, and grammar errors in the text below.
Strict rules: Output ONLY the corrected text. Preserve all code syntax, markdown tags, and formatting exactly as-is.

[Text]
{tag_check_errors_filter}
                "#
                .trim()
                .into(),
            },
            ActionModel {
                tag: "tag_spellcheck".into(),
                r#type: ActionMode::Value,
                expect: ExpectMode::String,
                check: None,
                confirm: false,
                action: "{tag_checked}".into(),
            },
        ],
    }
}
