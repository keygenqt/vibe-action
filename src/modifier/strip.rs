//! Strip modifier — automatically detects and removes common wrapper/fence delimiters.
//! No arguments needed. Supports markdown code fences (```), tilde fences (~~~),
//! inline backticks (`), and triple quotes ("""). Extensible via the patterns list.

use anyhow::Result;
use regex::Regex;

use crate::modifier::modifier::ITEM_SEP;

use super::modifier::Modifier;
use super::modifier::ModifierKey;

pub struct StripModifier {
    patterns: Vec<Regex>,
}

impl StripModifier {
    pub fn new() -> Self {
        Self {
            patterns: vec![
                // Markdown code fence: ```lang\n...\n```
                Regex::new(r"^```[^\n]*\n([\s\S]*?)\n?```$").unwrap(),
                // Tilde fence: ~~~lang\n...\n~~~
                Regex::new(r"^~~~[^\n]*\n([\s\S]*?)\n?~~~$").unwrap(),
                // Inline backticks: `code`
                Regex::new(r"^`(.+?)`$").unwrap(),
                // Triple quotes: """..."""
                Regex::new(r#"^"""([\s\S]*?)"""$"#).unwrap(),
            ],
        }
    }

    /// Try to detect and strip a wrapper from a single string.
    fn strip_wrapper(&self, s: &str) -> String {
        let trimmed = s.trim();
        for pattern in &self.patterns {
            if let Some(captures) = pattern.captures(trimmed) {
                if let Some(content) = captures.get(1) {
                    return content.as_str().trim().to_string();
                }
            }
        }
        trimmed.to_string()
    }
}

impl Modifier for StripModifier {
    fn key(&self) -> ModifierKey {
        ModifierKey::Strip
    }

    fn apply(&self, value: &str, _arg: &str) -> Result<String> {
        Ok(value
            .split(ITEM_SEP)
            .map(|i| self.strip_wrapper(i))
            .collect::<Vec<_>>()
            .join(ITEM_SEP))
    }
}
