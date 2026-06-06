//! Default commit action template.

use std::path::PathBuf;

use crate::models::action::{ActionMode, ActionModel, ExpectMode};
use crate::models::arg::ActionArg;
use crate::models::flow::FlowModel;

/// Returns the default commit FlowModel.
pub fn default() -> FlowModel {
    FlowModel {
        name: "commit".into(),
        about: "AI-generated commit message".into(),
        path: PathBuf::from("act/commit.yaml"),
        args: vec![ActionArg {
            name: "path".into(),
            short: Some("p".into()),
            expect: ExpectMode::String,
            help: Some("Path to git repository (default: current directory)".into()),
            required: Some(true),
        }],
        trigger: ActionModel {
            tag: "commit".into(),
            r#type: ActionMode::Cmd,
            expect: ExpectMode::String,
            r#match: None,
            action: r#"
echo "Your commit:"
echo "cd {path} && git commit -m '{commit_message}'"
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
                tag: "file_diffs".into(),
                r#type: ActionMode::Cmd,
                expect: ExpectMode::String,
                r#match: None,
                action: "cd {path} && git diff --stat {changed_files}".into(),
            },
            ActionModel {
                tag: "file_summaries".into(),
                r#type: ActionMode::Llm,
                expect: ExpectMode::String,
                r#match: None,
                action: r#"
For each file below, describe the changes in one line:
{file_diffs}
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
Write a single conventional commit message summarizing ALL changes below.
Choose the best type: feat (new feature), fix (bug fix), docs (documentation),
refactor (code restructuring), test (testing), chore (maintenance).
Examples:
  feat: add user authentication
  fix: resolve memory leak in parser
  docs: update API documentation
Reply with ONE message:
{file_summaries}
                "#
                .trim()
                .into(),
            },
        ],
    }
}
