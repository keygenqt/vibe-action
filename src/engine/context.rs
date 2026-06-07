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

    /// Fill {tag} placeholders.
    /// Simple tags are replaced with their values.
    /// List tags expand into multiple strings (cartesian product).
    pub fn fill(&self, text: &str) -> Vec<String> {
        let re = Regex::new(r"\{(\w+)\}").unwrap();

        // Collect all list tags.
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

        // Start with the original text.
        let mut results = vec![text.to_string()];

        // For each list tag, expand results with each item.
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

        // Fill remaining simple tags in each result.
        results.iter().map(|r| self.fill_simple(r, &re)).collect()
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
