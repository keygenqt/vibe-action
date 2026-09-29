//! AST operator — parses source code into structured JSON via vibe-ast.

use anyhow::Result;
use vibe_ast::ArkTSNode;
use vibe_ast::BashNode;
use vibe_ast::BatchNode;
use vibe_ast::CSharpNode;
use vibe_ast::DartNode;
use vibe_ast::GoNode;
use vibe_ast::JavaNode;
use vibe_ast::JavaScriptNode;
use vibe_ast::KotlinNode;
use vibe_ast::Language;
use vibe_ast::MarkdownNode;
use vibe_ast::PythonNode;
use vibe_ast::RustNode;
use vibe_ast::SwiftNode;
use vibe_ast::TypeScriptNode;
use vibe_ast::nodes::nodes::Nodes;
use vibe_ast::parse_file;
use vibe_ast::parse_text;

use crate::operator::operator::ITEM_SEP;
use crate::operator::operator::Operator;
use crate::operator::operator::OperatorKey;
use crate::operator::read::read::ReadKey;
use crate::utils;

pub struct AstOperator;

impl AstOperator {
    fn lang_from_arg(arg: &str) -> Result<Language> {
        utils::langs::lang_from_ext(arg).ok_or_else(|| anyhow::anyhow!("Unknown language: {}", arg))
    }

    /// Remove non-essential data from AST based on language.
    fn clean(nodes: &mut Nodes) {
        match nodes {
            Nodes::ArkTS(n) => Self::clean_arkts(n),
            Nodes::Bash(n) => Self::clean_bash(n),
            Nodes::Batch(n) => Self::clean_batch(n),
            Nodes::CSharp(n) => Self::clean_csharp(n),
            Nodes::Dart(n) => Self::clean_dart(n),
            Nodes::Go(n) => Self::clean_go(n),
            Nodes::Java(n) => Self::clean_java(n),
            Nodes::JavaScript(n) => Self::clean_javascript(n),
            Nodes::Kotlin(n) => Self::clean_kotlin(n),
            Nodes::Markdown(n) => Self::clean_markdown(n),
            Nodes::Python(n) => Self::clean_python(n),
            Nodes::Rust(n) => Self::clean_rust(n),
            Nodes::Swift(n) => Self::clean_swift(n),
            Nodes::TypeScript(n) => Self::clean_typescript(n),
        }
    }

    /// Remove non-essential data from ArkTS AST.
    fn clean_arkts(n: &mut ArkTSNode) {
        n.imports.clear();
        n.tags.clear();
        n.markers.clear();
        n.warnings.clear();
        for f in &mut n.functions {
            f.body = None;
        }
        for c in &mut n.classes {
            c.body = None;
        }
        for s in &mut n.structs {
            s.body = None;
        }
        for e in &mut n.enums {
            e.body = None;
        }
        for i in &mut n.interfaces {
            i.body = None;
        }
    }

    /// Remove non-essential data from Bash AST.
    fn clean_bash(n: &mut BashNode) {
        n.imports.clear();
        n.tags.clear();
        n.markers.clear();
        n.warnings.clear();
        for f in &mut n.functions {
            f.body = None;
        }
    }

    /// Remove non-essential data from Batch AST.
    fn clean_batch(n: &mut BatchNode) {
        n.imports.clear();
        n.tags.clear();
        n.markers.clear();
        n.warnings.clear();
        for f in &mut n.functions {
            f.body = None;
        }
    }

    /// Remove non-essential data from C# AST.
    fn clean_csharp(n: &mut CSharpNode) {
        n.imports.clear();
        n.tags.clear();
        n.markers.clear();
        n.warnings.clear();
        for f in &mut n.functions {
            f.body = None;
        }
        for c in &mut n.classes {
            c.body = None;
        }
        for s in &mut n.structs {
            s.body = None;
        }
        for e in &mut n.enums {
            e.body = None;
        }
        for i in &mut n.interfaces {
            i.body = None;
        }
    }

    /// Remove non-essential data from Dart AST.
    fn clean_dart(n: &mut DartNode) {
        n.imports.clear();
        n.tags.clear();
        n.markers.clear();
        n.warnings.clear();
        for f in &mut n.functions {
            f.body = None;
        }
        for c in &mut n.classes {
            c.body = None;
        }
        for e in &mut n.enums {
            e.body = None;
        }
        for i in &mut n.interfaces {
            i.body = None;
        }
    }

    /// Remove non-essential data from Go AST.
    fn clean_go(n: &mut GoNode) {
        n.imports.clear();
        n.tags.clear();
        n.markers.clear();
        n.warnings.clear();
        for f in &mut n.functions {
            f.body = None;
        }
        for s in &mut n.structs {
            s.body = None;
        }
        for e in &mut n.enums {
            e.body = None;
        }
    }

    /// Remove non-essential data from Java AST.
    fn clean_java(n: &mut JavaNode) {
        n.imports.clear();
        n.tags.clear();
        n.markers.clear();
        n.warnings.clear();
        for f in &mut n.functions {
            f.body = None;
        }
        for c in &mut n.classes {
            c.body = None;
        }
        for s in &mut n.structs {
            s.body = None;
        }
        for e in &mut n.enums {
            e.body = None;
        }
        for i in &mut n.interfaces {
            i.body = None;
        }
    }

    /// Remove non-essential data from JavaScript AST.
    fn clean_javascript(n: &mut JavaScriptNode) {
        n.imports.clear();
        n.tags.clear();
        n.markers.clear();
        n.warnings.clear();
        for f in &mut n.functions {
            f.body = None;
        }
        for c in &mut n.classes {
            c.body = None;
        }
    }

    /// Remove non-essential data from Kotlin AST.
    fn clean_kotlin(n: &mut KotlinNode) {
        n.imports.clear();
        n.tags.clear();
        n.markers.clear();
        n.warnings.clear();
        for f in &mut n.functions {
            f.body = None;
        }
        for c in &mut n.classes {
            c.body = None;
        }
        for e in &mut n.enums {
            e.body = None;
        }
        for i in &mut n.interfaces {
            i.body = None;
        }
    }

    /// Remove non-essential data from Markdown AST.
    fn clean_markdown(n: &mut MarkdownNode) {
        n.tags.clear();
        n.markers.clear();
        n.warnings.clear();
        for h in &mut n.headings {
            h.body = None;
        }
    }

    /// Remove non-essential data from Python AST.
    fn clean_python(n: &mut PythonNode) {
        n.imports.clear();
        n.tags.clear();
        n.markers.clear();
        n.warnings.clear();
        for f in &mut n.functions {
            f.body = None;
        }
        for c in &mut n.classes {
            c.body = None;
        }
    }

    /// Remove non-essential data from Rust AST.
    fn clean_rust(n: &mut RustNode) {
        n.imports.clear();
        n.tags.clear();
        n.markers.clear();
        n.warnings.clear();
        for f in &mut n.functions {
            f.body = None;
        }
        for s in &mut n.structs {
            s.body = None;
        }
        for e in &mut n.enums {
            e.body = None;
        }
        for i in &mut n.interfaces {
            i.body = None;
        }
    }

    /// Remove non-essential data from Swift AST.
    fn clean_swift(n: &mut SwiftNode) {
        n.imports.clear();
        n.tags.clear();
        n.markers.clear();
        n.warnings.clear();
        for f in &mut n.functions {
            f.body = None;
        }
        for c in &mut n.classes {
            c.body = None;
        }
        for s in &mut n.structs {
            s.body = None;
        }
        for e in &mut n.enums {
            e.body = None;
        }
        for i in &mut n.interfaces {
            i.body = None;
        }
    }

    /// Remove non-essential data from TypeScript AST.
    fn clean_typescript(n: &mut TypeScriptNode) {
        n.imports.clear();
        n.tags.clear();
        n.markers.clear();
        n.warnings.clear();
        for f in &mut n.functions {
            f.body = None;
        }
        for c in &mut n.classes {
            c.body = None;
        }
        for s in &mut n.structs {
            s.body = None;
        }
        for e in &mut n.enums {
            e.body = None;
        }
        for i in &mut n.interfaces {
            i.body = None;
        }
    }
}

impl Operator for AstOperator {
    fn key(&self) -> OperatorKey {
        ReadKey::Ast.key()
    }

    fn apply(&self, value: &str, arg: &str) -> Result<String> {
        let files: Vec<&str> = value.split(ITEM_SEP).collect();
        let mut results = Vec::new();

        for path in &files {
            let json = if arg.is_empty() || arg == "brief" {
                match parse_file(path) {
                    Ok(mut nodes) => {
                        Self::clean(&mut nodes);
                        let mut json_value = serde_json::to_value(&nodes)?;

                        let is_empty = match &json_value {
                            serde_json::Value::Object(map) => map.values().all(|v| {
                                v.is_null()
                                    || (v.is_object()
                                        && v.as_object().map_or(false, |o| o.is_empty()))
                                    || (v.is_array()
                                        && v.as_array().map_or(false, |a| a.is_empty()))
                            }),
                            _ => false,
                        };

                        if is_empty {
                            String::new()
                        } else {
                            if let serde_json::Value::Object(map) = &mut json_value {
                                map.insert(
                                    "path".to_string(),
                                    serde_json::Value::String(path.to_string()),
                                );
                            }
                            serde_json::to_string(&json_value)?
                        }
                    }
                    Err(_) => String::new(),
                }
            } else if arg == "full" {
                match parse_file(path) {
                    Ok(nodes) => {
                        let mut json_value = serde_json::to_value(&nodes)?;
                        if let serde_json::Value::Object(map) = &mut json_value {
                            map.insert(
                                "path".to_string(),
                                serde_json::Value::String(path.to_string()),
                            );
                        }
                        serde_json::to_string(&json_value)?
                    }
                    Err(_) => String::new(),
                }
            } else {
                let lang = Self::lang_from_arg(arg)?;
                let nodes = parse_text(path, lang)?;
                serde_json::to_string(&nodes)?
            };

            if !json.is_empty() {
                results.push(json);
            }
        }

        Ok(results.join(ITEM_SEP))
    }
}
