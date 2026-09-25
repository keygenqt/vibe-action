//! Strip operator — removes common wrapper/fence delimiters (``` ~~~ ` """).

use crate::operator::operator::Operator;
use crate::operator::operator::OperatorKey;
use crate::operator::operator::map_items;
use crate::operator::transform::transform::TransformKey;
use anyhow::Result;
use regex::Regex;

pub struct StripOperator {
    patterns: Vec<Regex>,
}

impl StripOperator {
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

impl Operator for StripOperator {
    fn key(&self) -> OperatorKey {
        TransformKey::Strip.key()
    }

    fn apply(&self, value: &str, _arg: &str) -> Result<String> {
        map_items(value, |s| Ok(self.strip_wrapper(s)))
    }
}
