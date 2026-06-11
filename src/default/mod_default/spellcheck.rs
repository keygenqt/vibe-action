//! Default spellcheck action template.

use std::path::PathBuf;

use crate::models::action::{ActionMode, ActionModel, ExpectMode};
use crate::models::arg::ArgActionModel;
use crate::models::flow::{FlowFormat, FlowModel};

/// Returns the default spellcheck FlowModel.
pub fn default() -> FlowModel {
    FlowModel {
        name: "spellcheck".into(),
        output: "tag_spellcheck".into(),
        format: FlowFormat::Rich,
        about: "Check and fix spelling in text or files".into(),
        path: PathBuf::from("mod/spellcheck.yaml"),
        r#match: None,
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
                r#match: Some(".+".into()),
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
                tag: "tag_checked".into(),
                r#type: ActionMode::Llm,
                expect: ExpectMode::String,
                r#match: None,
                confirm: false,
                action: r#"
[Task]
Check the text inside <text></text> for spelling and grammar errors.
Fix all errors. Preserve formatting, code blocks, and proper names.
If there are no errors, reply with "No errors found."
Otherwise, reply with the corrected text.

<text>
{tag_content}
</text>
                "#
                .trim()
                .into(),
            },
            ActionModel {
                tag: "tag_spellcheck".into(),
                r#type: ActionMode::Value,
                expect: ExpectMode::String,
                r#match: None,
                confirm: false,
                action: "{tag_checked|normalize}".into(),
            },
        ],
    }
}
