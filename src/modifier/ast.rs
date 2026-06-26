//! AST modifier — parses source code into structured JSON via vibe-ast.

use anyhow::Result;
use vibe_ast::Language;
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
        let lang = Self::lang_from_arg(arg)?;
        match value {
            ContextModel::String(code) => {
                let nodes = parse_text(code, lang)
                    .map_err(|e| anyhow::anyhow!("AST parse error: {}", e))?;
                let json = serde_json::to_string(&nodes)
                    .map_err(|e| anyhow::anyhow!("JSON serialize error: {}", e))?;
                Ok(ContextModel::String(json))
            }
            _ => anyhow::bail!("Modifier 'ast' expects a string (source code)"),
        }
    }
}
