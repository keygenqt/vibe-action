//! Value resolver — converts raw strings into typed ContextModel values.

use anyhow::Result;

use crate::models::action::ExpectMode;
use crate::models::context::ContextModel;

/// Resolves raw strings into typed ContextModel values.
pub struct Resolve;

impl Resolve {
    /// Main entry point — route to specific type handler.
    pub fn resolve(raw: &str, expect: &ExpectMode) -> Result<ContextModel> {
        match expect {
            ExpectMode::Void => Ok(ContextModel::Void),
            ExpectMode::Bool => Self::resolve_bool(raw),
            ExpectMode::Number => Self::resolve_number(raw),
            ExpectMode::String => Ok(ContextModel::String(raw.trim().to_string())),
            ExpectMode::List(inner) => Self::resolve_list(raw, inner),
        }
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

    /// Recursive list resolution.
    fn resolve_list(raw: &str, inner: &ExpectMode) -> Result<ContextModel> {
        let items: Vec<String> = raw
            .trim()
            .lines()
            .map(|l| l.trim().to_string())
            .filter(|l| !l.is_empty())
            .collect();

        let mut parsed = Vec::with_capacity(items.len());
        for item in items {
            parsed.push(Self::resolve(&item, inner)?);
        }
        Ok(ContextModel::List(parsed))
    }
}
