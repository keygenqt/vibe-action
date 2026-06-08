//! Default commit action template.

use std::path::PathBuf;

use crate::models::action::{ActionMode, ActionModel, ExpectMode};
use crate::models::arg::ArgActionModel;
use crate::models::flow::{FlowMode, FlowModel};

/// Returns the default commit FlowModel.
pub fn default() -> FlowModel {
    FlowModel {
        name: "commit".into(),
        mode: FlowMode::Ask,
        about: "AI-generated commit message".into(),
        path: PathBuf::from("act/commit.yaml"),
        args: vec![ArgActionModel {
            name: "path".into(),
            short: Some('p'),
            expect: ExpectMode::String,
            help: Some("Path to git repository (default: current directory)".into()),
            required: true,
            values: Vec::new(),
        }],
        trigger: ActionModel {
            tag: "commit".into(),
            r#type: ActionMode::Cmd,
            expect: ExpectMode::String,
            r#match: None,
            action: r#"
cd {path} && git add . && git commit -m '{commit_message}'
            "#
            .trim()
            .into(),
        },
        actions: vec![
            ActionModel {
                tag: "changed_files".into(),
                r#type: ActionMode::Cmd,
                expect: ExpectMode::List(Box::new(ExpectMode::String)),
                r#match: Some(".+".into()),
                action: "cd {path} && git diff --name-only".into(),
            },
            ActionModel {
                tag: "file_diff".into(),
                r#type: ActionMode::Cmd,
                expect: ExpectMode::List(Box::new(ExpectMode::String)),
                r#match: None,
                action: "cd {path} && git diff {changed_files}".into(),
            },
            ActionModel {
                tag: "file_summary".into(),
                r#type: ActionMode::Llm,
                expect: ExpectMode::List(Box::new(ExpectMode::String)),
                r#match: None,
                action: r#"
[Task]
Summarize the git diff below in strictly ONE sentence (max 10 words).
Start with an action verb (Add, Fix, Refactor, Change).
Write strictly in English.

[Diff]
{file_diff}

[Result]
<summary>
                "#
                .trim()
                .into(),
            },
            ActionModel {
                tag: "commit_message".into(),
                r#type: ActionMode::Llm,
                expect: ExpectMode::String,
                r#match: None,
                action: r#"
[Task]
Combine all summaries below into ONE conventional commit message.
Write strictly in English and in ONE short sentence (max 15 words).
Allowed types: feat, fix, docs, refactor, test, chore.

[Summaries]
{file_summary}

[Result]
<type>: <commit>

                "#
                .trim()
                .into(),
            },
        ],
    }
}
