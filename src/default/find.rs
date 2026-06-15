//! Default find action template.

use crate::models::action::{ActionMode, ActionModel, ExpectMode};
use crate::models::arg::ArgActionModel;
use crate::models::flow::FlowModel;

/// Returns the default find FlowModel.
pub fn default() -> FlowModel {
    FlowModel {
        name: "find".into(),
        about: "Semantic file finder — finds files by meaning, not just name".into(),
        check: None,
        clipboard: false,
        args: vec![
            ArgActionModel {
                name: "path".into(),
                short: Some('p'),
                expect: ExpectMode::String,
                help: Some("Directory to search in (default: current)".into()),
                default: Some(".".into()),
                values: Vec::new(),
            },
            ArgActionModel {
                name: "query".into(),
                short: Some('q'),
                expect: ExpectMode::String,
                help: Some("What to find — describe in natural language".into()),
                default: None,
                values: Vec::new(),
            },
        ],
        actions: vec![
            ActionModel {
                tag: "tag_files".into(),
                r#type: ActionMode::Cmd,
                expect: ExpectMode::List(Box::new(ExpectMode::String)),
                check: None,
                confirm: false,
                action: "find {path} -type f -exec grep -Iq . {} \\; -print".into(),
            },
            ActionModel {
                tag: "tag_content".into(),
                r#type: ActionMode::Cmd,
                expect: ExpectMode::String,
                check: None,
                confirm: false,
                action: r#"
cat << 'EOF'
[Task]
Check if the [Content] code/comments contain the technical [Query] word, function name, or meaning.
If YES — print ONLY the [File] path inside brackets: <{tag_files}>
If NO — print ONLY: <->

[Query]
{query}

[Content]
EOF
cat "{tag_files}"
cat << 'EOF'

[File]
{tag_files}
EOF
                "#
                .trim()
                .into(),
            },
            ActionModel {
                tag: "tag_matches".into(),
                r#type: ActionMode::Llm,
                expect: ExpectMode::List(Box::new(ExpectMode::String)),
                check: None,
                confirm: false,
                action: "{tag_content}".into(),
            },
            ActionModel {
                tag: "tag_filtered".into(),
                r#type: ActionMode::Value,
                expect: ExpectMode::String,
                check: None,
                confirm: false,
                action: "{tag_matches|trim:<->}".into(),
            },
            ActionModel {
                tag: "tag_find".into(),
                r#type: ActionMode::Value,
                expect: ExpectMode::String,
                check: None,
                confirm: false,
                action: "{tag_filtered|join:uniq}".into(),
            },
        ],
    }
}
