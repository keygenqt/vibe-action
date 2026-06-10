//! Default find action template.

use std::path::PathBuf;

use crate::models::action::{ActionMode, ActionModel, ExpectMode};
use crate::models::arg::ArgActionModel;
use crate::models::flow::{FlowFormat, FlowModel};

/// Returns the default find FlowModel.
pub fn default() -> FlowModel {
    FlowModel {
        name: "find".into(),
        output: "tag_find".into(),
        format: FlowFormat::Rich,
        about: "Semantic file finder — finds files by meaning, not just name".into(),
        path: PathBuf::from("act/find.yaml"),
        r#match: None,
        clipboard: false,
        args: vec![
            ArgActionModel {
                name: "path".into(),
                short: Some('p'),
                expect: ExpectMode::String,
                help: Some("Directory to search in (default: current)".into()),
                required: true,
                values: Vec::new(),
            },
            ArgActionModel {
                name: "query".into(),
                short: Some('q'),
                expect: ExpectMode::String,
                help: Some("What to find — describe in natural language".into()),
                required: true,
                values: Vec::new(),
            },
        ],
        actions: vec![
            ActionModel {
                tag: "tag_files".into(),
                r#type: ActionMode::Cmd,
                expect: ExpectMode::List(Box::new(ExpectMode::String)),
                r#match: None,
                confirm: false,
                action: "find {path} -type f -exec grep -Iq . {} \\; -print".into(),
            },
            ActionModel {
                tag: "tag_content".into(),
                r#type: ActionMode::Cmd,
                expect: ExpectMode::String,
                r#match: None,
                confirm: false,
                action: r#"
cat << 'EOF'
[Instruction]
Check if the [Content] code/comments contain the technical [Query] word, function name, or meaning.
If YES — print ONLY the [File] path inside brackets: <{tag_files}>
If NO — print ONLY: <->
Strict rule: Output ONLY the bracketed value. No descriptions, no comments.

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
                r#match: None,
                confirm: false,
                action: "{tag_content}".into(),
            },
            ActionModel {
                tag: "tag_filtered".into(),
                r#type: ActionMode::Cmd,
                expect: ExpectMode::List(Box::new(ExpectMode::String)),
                r#match: None,
                confirm: false,
                action: r#"
echo '{tag_matches|join}' | sed -n 's/.*<\(.*\)>.*/\1/p' | sed '/^-$/d'
                "#
                .trim()
                .into(),
            },
            ActionModel {
                tag: "tag_find".into(),
                r#type: ActionMode::Cmd,
                expect: ExpectMode::String,
                r#match: None,
                confirm: false,
                action: "echo '{tag_filtered}'".into(),
            },
        ],
    }
}
