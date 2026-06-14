//! Default commit action template.

use crate::models::action::{ActionMode, ActionModel, ExpectMode};
use crate::models::arg::ArgActionModel;
use crate::models::flow::FlowModel;

/// Returns the default commit FlowModel.
pub fn default() -> FlowModel {
    FlowModel {
        name: "commit".into(),
        about: "AI-generated commit message".into(),
        check: None,
        clipboard: false,
        args: vec![ArgActionModel {
            name: "path".into(),
            short: Some('p'),
            expect: ExpectMode::String,
            help: Some("Path to git repository (default: current directory)".into()),
            default: Some(".".into()),
            values: Vec::new(),
        }],
        actions: vec![
            ActionModel {
                tag: "tag_changed_files".into(),
                r#type: ActionMode::Cmd,
                expect: ExpectMode::List(Box::new(ExpectMode::String)),
                check: Some(".+".into()),
                confirm: false,
                action: "cd {path} && git diff --name-only --relative".into(),
            },
            ActionModel {
                tag: "tag_file_diff".into(),
                r#type: ActionMode::Cmd,
                expect: ExpectMode::List(Box::new(ExpectMode::String)),
                check: None,
                confirm: false,
                action: r#"
cd {path}
if [ -f "{tag_changed_files}" ]; then
  git diff -- "{tag_changed_files}"
fi
                "#
                .trim()
                .into(),
            },
            ActionModel {
                tag: "tag_file_summary".into(),
                r#type: ActionMode::Llm,
                expect: ExpectMode::List(Box::new(ExpectMode::String)),
                check: None,
                confirm: false,
                action: r#"
[Task]
Summarize the git diff below in strictly ONE sentence (max 10 words).
Start with an action verb (Add, Fix, Refactor, Change).
Write strictly in English.

[Diff]
{tag_file_diff}
                "#
                .trim()
                .into(),
            },
            ActionModel {
                tag: "tag_commit_message".into(),
                r#type: ActionMode::Llm,
                expect: ExpectMode::String,
                check: None,
                confirm: false,
                action: r#"
[Task]
Summarize all summaries below into ONE short conventional commit message (max 10 words).
Allowed types: feat, fix, docs, refactor, test, chore.
Format strictly: <type>: <commit> (NO parentheses or scopes).

[Summaries]
{tag_file_summary|join:uniq}
                "#
                .trim()
                .into(),
            },
            ActionModel {
                tag: "tag_commit".into(),
                r#type: ActionMode::Cmd,
                expect: ExpectMode::String,
                check: None,
                confirm: true,
                action: "cd {path} && git add . && git commit -m '{tag_commit_message}'".into(),
            },
        ],
    }
}
