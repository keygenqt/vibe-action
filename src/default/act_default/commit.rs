//! Default commit action template.

use std::path::PathBuf;

use crate::models::action::{ActionMode, ActionModel, ExpectMode};
use crate::models::arg::ArgActionModel;
use crate::models::flow::{FlowFormat, FlowModel};

/// Returns the default commit FlowModel.
pub fn default() -> FlowModel {
    FlowModel {
        name: "commit".into(),
        output: "tag_commit".into(),
        format: FlowFormat::Compact,
        about: "AI-generated commit message".into(),
        path: PathBuf::from("act/commit.yaml"),
        r#match: None,
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
                r#match: Some(".+".into()),
                confirm: false,
                action: "cd {path} && git diff --name-only".into(),
            },
            ActionModel {
                tag: "tag_file_diff".into(),
                r#type: ActionMode::Cmd,
                expect: ExpectMode::List(Box::new(ExpectMode::String)),
                r#match: None,
                confirm: false,
                action: "cd {path} && git diff {tag_changed_files}".into(),
            },
            ActionModel {
                tag: "tag_file_summary".into(),
                r#type: ActionMode::Llm,
                expect: ExpectMode::List(Box::new(ExpectMode::String)),
                r#match: None,
                confirm: false,
                action: r#"
[Task]
Summarize the git diff below in strictly ONE sentence (max 10 words).
Start with an action verb (Add, Fix, Refactor, Change).
Write strictly in English.

[Diff]
{tag_file_diff}

[Result]
<summary>
                "#
                .trim()
                .into(),
            },
            ActionModel {
                tag: "tag_commit_message".into(),
                r#type: ActionMode::Llm,
                expect: ExpectMode::String,
                r#match: None,
                confirm: false,
                action: r#"
[Task]
Summarize all summaries below into ONE short conventional commit message (max 10 words).
Allowed types: feat, fix, docs, refactor, test, chore.
Format strictly: <type>: <commit> (NO parentheses or scopes).

[Summaries]
{tag_file_summary|join}

[Result]
<type>: <commit>
                "#
                .trim()
                .into(),
            },
            ActionModel {
                tag: "tag_commit".into(),
                r#type: ActionMode::Cmd,
                expect: ExpectMode::String,
                r#match: None,
                confirm: true,
                action: "cd {path} && git add . && git commit -m '{tag_commit_message}'".into(),
            },
        ],
    }
}
