//! AST modifier — parses source code into structured JSON via vibe-ast.

use anyhow::Result;
use vibe_ast::Language;
use vibe_ast::parse_file;
use vibe_ast::parse_text;

use super::modifier::Modifier;
use super::modifier::ModifierKey;
use crate::models::context::ContextModel;

pub struct AstModifier;

impl AstModifier {
    fn lang_from_arg(arg: &str) -> Result<Language> {
        match arg {
            "rs" => Ok(Language::Rust),
            "py" => Ok(Language::Python),
            "ts" => Ok(Language::TypeScript),
            "js" => Ok(Language::JavaScript),
            "java" => Ok(Language::Java),
            "go" => Ok(Language::Go),
            "cs" => Ok(Language::CSharp),
            "kt" => Ok(Language::Kotlin),
            "swift" => Ok(Language::Swift),
            "dart" => Ok(Language::Dart),
            "sh" => Ok(Language::Bash),
            "bat" => Ok(Language::Batch),
            "ets" => Ok(Language::ArkTS),
            "md" => Ok(Language::Markdown),
            _ => anyhow::bail!("Unknown language: {}", arg),
        }
    }
}

impl Modifier for AstModifier {
    fn key(&self) -> ModifierKey {
        ModifierKey::Ast
    }

    fn apply(&self, value: &ContextModel, arg: &str) -> Result<ContextModel> {
        match value {
            ContextModel::String(s) => {
                if arg.is_empty() {
                    match parse_file(s) {
                        Ok(nodes) => Ok(ContextModel::String(serde_json::to_string(&nodes)?)),
                        Err(_) => Ok(ContextModel::String(String::new())),
                    }
                } else {
                    let lang = Self::lang_from_arg(arg)?;
                    let nodes = parse_text(s, lang)?;
                    let json = serde_json::to_string(&nodes)?;
                    Ok(ContextModel::String(json))
                }
            }
            ContextModel::List(files) => {
                let results: Vec<String> = files
                    .iter()
                    .map(
                        |path| match self.apply(&ContextModel::String(path.clone()), "")? {
                            ContextModel::String(json) => Ok(json),
                            _ => unreachable!(),
                        },
                    )
                    .collect::<Result<Vec<_>>>()?;
                Ok(ContextModel::List(results))
            }
        }
    }
}
