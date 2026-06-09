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

    /// Get a resolved tag value.
    pub fn get(&self, tag: &str) -> Option<&ContextModel> {
        self.values.get(tag)
    }

    /// Fill {tag} placeholders with list expansion.
    /// Simple tags are skipped during the loop and handled at the very end.
    /// List tags expand the results matrix into multiple variants (cartesian product).
    pub fn fill(&self, text: &str, escape: bool) -> Vec<String> {
        let re = Regex::new(r"\{(\w+)\}").unwrap();
        let text = &Self::strip_all_template_quotes(text, escape);

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
                    let raw_val = item.to_string();
                    let safe_val = if escape {
                        shell_words::quote(&raw_val).to_string()
                    } else {
                        raw_val
                    };
                    let replaced = current.replace(&format!("{{{}}}", tag), &safe_val);
                    next.push(replaced);
                }
            }
            results = next;
        }

        // Fill any remaining non-list (scalar) placeholders across all expanded strings.
        results
            .iter()
            .map(|r| self.fill_simple(r, &re, escape))
            .collect()
    }

    /// Fill {tag} placeholders by merging list values into a single plain string.
    /// Uses a newline '\n' separator for multi-line blocks (e.g., LLM prompts)
    /// and a space ' ' separator for single-line structures (e.g., shell commands).
    pub fn fill_join(&self, text: &str, escape: bool) -> String {
        let re = Regex::new(r"\{(\w+)\}").unwrap();
        let text = &Self::strip_all_template_quotes(text, escape);

        // Contextual line formatting detection.
        let separator = if text.contains('\n') { "\n" } else { " " };

        re.replace_all(text, |caps: &regex::Captures| {
            let tag = caps.get(1).unwrap().as_str();
            self.values
                .get(tag)
                .map(|v| {
                    let raw_string = match v {
                        ContextModel::List(items) => items
                            .iter()
                            .map(|i| i.to_string())
                            .collect::<Vec<_>>()
                            .join(separator),
                        other => other.to_string(),
                    };
                    if escape {
                        shell_words::quote(&raw_string).to_string()
                    } else {
                        raw_string
                    }
                })
                // Fallback to the original matching placeholder if the key is missing in memory.
                .unwrap_or_else(|| caps.get(0).unwrap().as_str().to_string())
        })
        .to_string()
    }

    /// Simple fill without list expansion.
    fn fill_simple(&self, text: &str, re: &Regex, escape: bool) -> String {
        let text = &Self::strip_all_template_quotes(text, escape);
        re.replace_all(&text, |caps: &regex::Captures| {
            let tag = caps.get(1).unwrap().as_str();
            self.values
                .get(tag)
                .map(|v| {
                    let raw_val = v.to_string();
                    if escape {
                        shell_words::quote(&raw_val).to_string()
                    } else {
                        raw_val
                    }
                })
                .unwrap_or_else(|| caps.get(0).unwrap().as_str().to_string())
        })
        .to_string()
    }

    /// Removes surrounding single or double quotes from any `{tag}` placeholder.
    fn strip_all_template_quotes(text: &str, escape: bool) -> String {
        if !escape {
            return text.to_string();
        }
        let re_clean = regex::Regex::new(r#"(['"])\{(\w+)\}(['"])"#).unwrap();
        re_clean
            .replace_all(text, |caps: &regex::Captures| {
                let left = caps.get(1).unwrap().as_str();
                let tag = caps.get(2).unwrap().as_str();
                let right = caps.get(3).unwrap().as_str();

                if left == right {
                    format!("{{{}}}", tag)
                } else {
                    caps.get(0).unwrap().as_str().to_string()
                }
            })
            .to_string()
    }
}
