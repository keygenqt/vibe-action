//! Value resolver — converts raw strings into typed ContextModel values.

use anyhow::Result;
use serde_json::Value;

use crate::models::action::ExpectMode;
use crate::models::context::ContextModel;

/// Resolves raw strings into typed ContextModel values.
pub struct Resolve;

// Resolve — отдельные методы для каждого типа
impl Resolve {
    /// Main entry point — route to specific type handler.
    /// Main entry point — route to specific type handler.
    pub fn resolve(outputs: Vec<String>, expect: &ExpectMode) -> Result<ContextModel> {
        let raw = outputs.join("\n");

        match expect {
            ExpectMode::Void => Self::resolve_void(&raw),
            ExpectMode::Bool => Self::resolve_bool(&raw),
            ExpectMode::Number => Self::resolve_number(&raw),
            ExpectMode::String => Self::resolve_string(&raw),
            ExpectMode::Json => Self::resolve_json(&raw),
            ExpectMode::List(inner) => {
                if outputs.len() > 1 {
                    let mut parsed = Vec::with_capacity(outputs.len());
                    for item in outputs {
                        let trimmed = item.trim().to_string();
                        if !trimmed.is_empty() {
                            parsed.push(Self::resolve(vec![trimmed], inner)?);
                        }
                    }
                    Ok(ContextModel::List(parsed))
                } else {
                    Self::resolve_list(&raw, inner)
                }
            }
        }
    }

    /// Resolve void (no value expected).
    fn resolve_void(_raw: &str) -> Result<ContextModel> {
        Ok(ContextModel::Void)
    }

    /// Resolve boolean value (supports ru/en/zh).
    fn resolve_bool(raw: &str) -> Result<ContextModel> {
        let lower = raw.trim().to_lowercase();
        match lower.as_str() {
            "true" | "yes" | "да" | "是" => Ok(ContextModel::Bool(true)),
            "false" | "no" | "нет" | "否" => Ok(ContextModel::Bool(false)),
            _ => anyhow::bail!("Expected bool, got: '{}'", raw),
        }
    }

    /// Resolve numeric value (integer or float).
    fn resolve_number(raw: &str) -> Result<ContextModel> {
        let clean: String = raw.trim().replace(',', ".");
        let num: f64 = clean
            .parse()
            .map_err(|_| anyhow::anyhow!("Expected number, got: '{}'", raw))?;
        Ok(ContextModel::Number(num))
    }

    /// Resolve string value (trimmed).
    fn resolve_string(raw: &str) -> Result<ContextModel> {
        Ok(ContextModel::String(raw.trim().to_string()))
    }

    /// Resolve JSON value. Strips ```json code blocks if present.
    fn resolve_json(raw: &str) -> Result<ContextModel> {
        let trimmed = raw.trim();
        let json_str = if trimmed.starts_with("```json") {
            trimmed
                .strip_prefix("```json")
                .and_then(|s| s.strip_suffix("```"))
                .map(|s| s.trim())
                .unwrap_or(trimmed)
        } else if trimmed.starts_with("```") {
            trimmed
                .strip_prefix("```")
                .and_then(|s| s.strip_suffix("```"))
                .map(|s| s.trim())
                .unwrap_or(trimmed)
        } else {
            trimmed
        };
        let json: Value = serde_json::from_str(json_str)
            .map_err(|_| anyhow::anyhow!("Expected JSON, got: '{}'", raw))?;
        Ok(ContextModel::Json(json))
    }

    /// Recursive list resolution.
    fn resolve_list(raw: &str, inner: &ExpectMode) -> Result<ContextModel> {
        let items: Vec<&str> = raw
            .trim()
            .lines()
            .map(|l| l.trim())
            .filter(|l| !l.is_empty())
            .collect();

        if items.is_empty() {
            return Ok(ContextModel::List(vec![]));
        }
        let mut parsed = Vec::with_capacity(items.len());
        for item in &items {
            parsed.push(Self::resolve(vec![item.to_string()], inner)?);
        }
        Ok(ContextModel::List(parsed))
    }
}
