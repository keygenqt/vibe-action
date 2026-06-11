//! Default extract action template.

use std::path::PathBuf;

use crate::models::action::{ActionMode, ActionModel, ExpectMode};
use crate::models::arg::ArgActionModel;
use crate::models::flow::{FlowFormat, FlowModel};

/// Returns the default extract FlowModel.
pub fn default() -> FlowModel {
    FlowModel {
        name: "extract".into(),
        output: "tag_extract".into(),
        format: FlowFormat::Rich,
        about: "Extract structured data or matching lines from text and logs".into(),
        path: PathBuf::from("act/extract.yaml"),
        r#match: None,
        clipboard: false,
        args: vec![
            ArgActionModel {
                name: "file".into(),
                short: Some('f'),
                expect: ExpectMode::String,
                help: Some("Path to the log or text file".into()),
                default: None,
                values: Vec::new(),
            },
            ArgActionModel {
                name: "query".into(),
                short: Some('q'),
                expect: ExpectMode::String,
                help: Some("Extraction criteria (e.g., 'find all errors')".into()),
                default: None,
                values: Vec::new(),
            },
        ],
        actions: vec![
            ActionModel {
                tag: "tag_lines".into(),
                r#type: ActionMode::Cmd,
                expect: ExpectMode::List(Box::new(ExpectMode::String)),
                r#match: None,
                confirm: false,
                action: "cat {file}".into(),
            },
            ActionModel {
                tag: "tag_content".into(),
                r#type: ActionMode::Llm,
                expect: ExpectMode::String,
                r#match: None,
                confirm: false,
                action: r#"
You are a log filter. Your task:
1. Read the log line and the search query.
2. If the line matches the query — output the EXACT line unchanged.
3. If the line does not match — output only a single dash: "-"
4. Do NOT add any comments, explanations, or extra text.
5. Do NOT skip lines. Process every line.

Query: {query}
Line: {tag_lines}
                "#
                .trim()
                .into(),
            },
            ActionModel {
                tag: "tag_clean".into(),
                r#type: ActionMode::Value,
                expect: ExpectMode::String,
                r#match: None,
                confirm: true,
                action: "{tag_content|trim:-}".into(),
            },
            ActionModel {
                tag: "tag_extract".into(),
                r#type: ActionMode::Value,
                expect: ExpectMode::String,
                r#match: None,
                confirm: false,
                action: "{tag_clean|join}".into(),
            },
        ],
    }
}
