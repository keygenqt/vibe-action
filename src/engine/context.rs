//! Runtime context for tag values.
//! Stores resolved tag -> value mapping and handles {tag} expansion.

use std::collections::HashMap;

use regex::Regex;

use crate::models::context::ContextModel;

/// Runtime context holding resolved tag values.
pub struct Context {
    values: HashMap<String, ContextModel>,
}

impl Context {
    /// Create empty context.
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }

    /// Store a resolved tag value.
    pub fn set(&mut self, tag: &str, value: ContextModel) {
        self.values.insert(tag.to_string(), value);
    }

    /// Fill {tag} placeholders with list expansion.
    /// Simple tags are skipped during the loop and handled at the very end.
    /// List tags expand the results matrix into multiple variants (cartesian product).
    pub fn fill(&self, text: &str) -> Vec<String> {
        let re = Regex::new(r"\{(\w+)\}").unwrap();

        // Collect all list tags found in the text template.
        let list_tags: Vec<(&str, &[ContextModel])> = re
            .captures_iter(text)
            .filter_map(|cap| {
                let tag = cap.get(1).unwrap().as_str();
                if let Some(ContextModel::List(items)) = self.values.get(tag) {
                    if !items.is_empty() {
                        return Some((tag, items.as_slice()));
                    }
                }
                None
            })
            .collect();

        // Start matrix resolution with the original text string.
        let mut results = vec![text.to_string()];

        // Generate the cartesian product: expand results for each item in each list tag.
        for (tag, items) in &list_tags {
            let mut next = Vec::new();
            for item in *items {
                for current in &results {
                    let replaced = current.replace(&format!("{{{}}}", tag), &item.to_string());
                    next.push(replaced);
                }
            }
            results = next;
        }

        // Fill any remaining non-list (scalar) placeholders across all expanded strings.
        results.iter().map(|r| self.fill_simple(r, &re)).collect()
    }

    /// Fill {tag} placeholders by merging list values into a single plain string.
    /// Uses a newline '\n' separator for multi-line blocks (e.g., LLM prompts)
    /// and a space ' ' separator for single-line structures (e.g., shell commands).
    pub fn fill_join(&self, text: &str) -> String {
        let re = Regex::new(r"\{(\w+)\}").unwrap();

        // Contextual line formatting detection.
        let separator = if text.contains('\n') { "\n" } else { " " };

        re.replace_all(text, |caps: &regex::Captures| {
            let tag = caps.get(1).unwrap().as_str();
            self.values
                .get(tag)
                .map(|v| match v {
                    // Flatten lists directly into the placeholder zone.
                    ContextModel::List(items) => items
                        .iter()
                        .map(|i| i.to_string())
                        .collect::<Vec<_>>()
                        .join(separator),
                    other => other.to_string(),
                })
                // Fallback to the original matching placeholder if the key is missing in memory.
                .unwrap_or_else(|| caps.get(0).unwrap().as_str().to_string())
        })
        .to_string()
    }

    /// Simple fill without list expansion.
    fn fill_simple(&self, text: &str, re: &Regex) -> String {
        re.replace_all(text, |caps: &regex::Captures| {
            let tag = caps.get(1).unwrap().as_str();
            self.values
                .get(tag)
                .map(|v| v.to_string())
                .unwrap_or_else(|| caps.get(0).unwrap().as_str().to_string())
        })
        .to_string()
    }
}
