//! Runtime context for tag values.
//! Stores resolved tag -> value mapping and handles {tag} expansion with pipe modifiers.

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

    /// Check if template has pure list tags (without |join) that trigger loop mode.
    pub fn has_loop_tags(&self, text: &str) -> bool {
        let re = Regex::new(r"\{(\w+)(?:\|(\w+))?\}").unwrap();
        re.captures_iter(text).any(|cap| {
            let tag = cap.get(1).unwrap().as_str();
            let modifier = cap.get(2).map(|m| m.as_str()).unwrap_or("");
            if modifier == "join" {
                return false;
            }
            matches!(self.values.get(tag), Some(ContextModel::List(items)) if !items.is_empty())
        })
    }

    /// Fill {tag} placeholders with list expansion (cartesian product).
    /// Skips tags with |join modifier. Replaces full placeholder including modifiers.
    pub fn fill(&self, text: &str, escape: bool) -> Vec<String> {
        let re = Regex::new(r"\{(\w+)(?:\|(\w+))?\}").unwrap();
        let text = &Self::strip_all_template_quotes(text, escape);

        // Collect (tag_name, full_placeholder, items) for replacement.
        let list_tags: Vec<(String, String, &[ContextModel])> = re
            .captures_iter(text)
            .filter_map(|cap| {
                let full_match = cap.get(0).unwrap().as_str().to_string();
                let tag = cap.get(1).unwrap().as_str();
                let modifier = cap.get(2).map(|m| m.as_str()).unwrap_or("");
                if modifier == "join" {
                    return None;
                }
                if let Some(ContextModel::List(items)) = self.values.get(tag) {
                    if !items.is_empty() {
                        return Some((tag.to_string(), full_match, items.as_slice()));
                    }
                }
                None
            })
            .collect();

        let mut results = vec![text.to_string()];
        for (_tag_name, full_placeholder, items) in &list_tags {
            let mut next = Vec::new();
            for item in *items {
                for current in &results {
                    let raw_val = item.to_string();
                    let safe_val = if escape {
                        shell_words::quote(&raw_val).to_string()
                    } else {
                        raw_val
                    };
                    // Replace the full placeholder (e.g., {tag|upper})
                    let replaced = current.replace(full_placeholder, &safe_val);
                    next.push(replaced);
                }
            }
            results = next;
        }

        // Fill remaining scalar tags in each expanded result.
        results.iter().map(|r| self.fill_join(r, escape)).collect()
    }

    /// Fill {tag} placeholders by merging list values into a single plain string.
    /// Supports pipe modifiers: join, upper, lower, trim, length.
    /// Escapes each list element individually before joining.
    pub fn fill_join(&self, text: &str, escape: bool) -> String {
        let re = Regex::new(r"\{(\w+)(?:\|(\w+))?\}").unwrap();
        let text = &Self::strip_all_template_quotes(text, escape);

        let separator = if text.contains('\n') { "\n" } else { " " };

        re.replace_all(text, |caps: &regex::Captures| {
            let tag = caps.get(1).unwrap().as_str();
            let modifier = caps.get(2).map(|m| m.as_str()).unwrap_or("");

            self.values
                .get(tag)
                .map(|v| match v {
                    ContextModel::List(items) => items
                        .iter()
                        .map(|i| {
                            // Apply modifier and escape each element individually.
                            let mut s = Self::apply_modifier(&i.to_string(), modifier);
                            if escape {
                                s = shell_words::quote(&s).to_string();
                            }
                            s
                        })
                        .collect::<Vec<_>>()
                        .join(separator),
                    other => {
                        let mut s = Self::apply_modifier(&other.to_string(), modifier);
                        if escape {
                            s = shell_words::quote(&s).to_string();
                        }
                        s
                    }
                })
                .unwrap_or_else(|| caps.get(0).unwrap().as_str().to_string())
        })
        .to_string()
    }

    /// Apply pipe modifier to a resolved value.
    fn apply_modifier(text: &str, modifier: &str) -> String {
        match modifier {
            "upper" => text.to_uppercase(),
            "lower" => text.to_lowercase(),
            "trim" => text.trim().to_string(),
            "length" => text.chars().count().to_string(),
            _ => text.to_string(),
        }
    }

    /// Removes surrounding single or double quotes from any {tag} placeholder.
    fn strip_all_template_quotes(text: &str, escape: bool) -> String {
        if !escape {
            return text.to_string();
        }
        let re_clean = regex::Regex::new(r#"(['"])\{(\w+)(?:\|(\w+))?\}(['"])"#).unwrap();
        re_clean
            .replace_all(text, |caps: &regex::Captures| {
                let left = caps.get(1).unwrap().as_str();
                let tag = caps.get(2).unwrap().as_str();
                let modifier = caps
                    .get(3)
                    .map(|m| format!("|{}", m.as_str()))
                    .unwrap_or_default();
                let right = caps.get(4).unwrap().as_str();

                if left == right {
                    format!("{{{}{}}}", tag, modifier)
                } else {
                    caps.get(0).unwrap().as_str().to_string()
                }
            })
            .to_string()
    }
}
