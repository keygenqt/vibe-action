//! Runtime context for tag values.
//! Stores resolved tag -> value mapping and handles {tag} expansion with pipe modifiers.

use anyhow::Result;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fmt::Write;

use crate::models::context::ContextModel;

/// Result of expanding a template with context values.
pub struct ExpandedTemplate {
    /// Expanded template strings (one per combination if list tags were present).
    pub items: Vec<String>,
    /// Whether the expansion involved list tags (loop mode).
    pub is_list: bool,
}

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

    /// Fill {tag} placeholders and return ExpandedTemplate.
    pub fn fill(&self, text: &str, escape: bool) -> Result<ExpandedTemplate> {
        // Supports modifiers with special chars: {tag|trim:-}, {tag|join}, {tag|upper}
        let re = Regex::new(r"\{(\w+)(?:\|([^}]+))?\}").unwrap();

        let clean_text = Self::strip_all_template_quotes(text, escape);
        let mut processed_tags = HashSet::new();
        let mut results = vec![clean_text.clone()];
        let mut is_list = false;

        for cap in re.captures_iter(&clean_text) {
            let placeholder = cap.get(0).unwrap().as_str();
            let tag_name = cap.get(1).unwrap().as_str();

            if processed_tags.contains(tag_name) {
                continue;
            }

            if let Some(context_value) = self.values.get(tag_name) {
                let modifier = cap.get(2).map(|m| m.as_str()).unwrap_or_default();
                let processed_value = self.apply_modifier(modifier, context_value, escape)?;

                match processed_value {
                    ContextModel::String(s) => {
                        let mut final_string = s;
                        if escape {
                            final_string = shell_words::quote(&final_string).to_string();
                        }
                        for current in results.iter_mut() {
                            *current = current.replace(placeholder, &final_string);
                        }
                        processed_tags.insert(tag_name.to_string());
                    }
                    ContextModel::List(items) => {
                        is_list = true;
                        let mut next = Vec::new();
                        for item in items {
                            let mut s = item.to_string();
                            if escape {
                                s = shell_words::quote(&s).to_string();
                            }
                            for current in &results {
                                next.push(current.replace(placeholder, &s));
                            }
                        }
                        results = next;
                        processed_tags.insert(tag_name.to_string());
                    }
                    _ => {}
                }
            }
        }

        Ok(ExpandedTemplate {
            items: results,
            is_list,
        })
    }

    /// Apply pipe modifier to a resolved ContextModel value.
    fn apply_modifier(
        &self,
        modifier: &str,
        value: &ContextModel,
        escape: bool,
    ) -> Result<ContextModel> {
        let (name, arg) = modifier.split_once(':').unwrap_or((modifier, ""));
        match name {
            "join" => {
                if let ContextModel::List(items) = value {
                    let mut buffer = String::new();
                    let mut seen = std::collections::HashSet::new();
                    let is_uniq = arg == "uniq";
                    for item in items {
                        let s = item.to_string();
                        if s.is_empty() {
                            continue;
                        }
                        if is_uniq && !seen.insert(s.clone()) {
                            continue;
                        }
                        if !buffer.is_empty() {
                            buffer.push('\n');
                        }
                        if escape {
                            write!(buffer, "{}", shell_words::quote(&s))?;
                        } else {
                            buffer.push_str(&s);
                        }
                    }
                    Ok(ContextModel::String(buffer))
                } else {
                    anyhow::bail!("Modifier 'join' expects a list, but got a scalar value")
                }
            }
            "upper" => {
                if let ContextModel::String(s) = value {
                    Ok(ContextModel::String(s.to_uppercase()))
                } else {
                    anyhow::bail!("Modifier 'upper' expects a string")
                }
            }
            "lower" => {
                if let ContextModel::String(s) = value {
                    Ok(ContextModel::String(s.to_lowercase()))
                } else {
                    anyhow::bail!("Modifier 'lower' expects a string")
                }
            }
            "trim" => {
                let chars: Vec<char> = arg.chars().collect();
                match value {
                    ContextModel::String(s) => {
                        let trimmed = s
                            .trim_matches(|c: char| c.is_whitespace() || chars.contains(&c))
                            .to_string();
                        if arg.is_empty() {
                            Ok(ContextModel::String(trimmed))
                        } else if trimmed == arg {
                            Ok(ContextModel::String(String::new()))
                        } else {
                            Ok(ContextModel::String(trimmed))
                        }
                    }
                    ContextModel::List(items) => {
                        let filtered: Vec<ContextModel> = items
                            .iter()
                            .filter(|i| {
                                let s = i
                                    .to_string()
                                    .trim_matches(|c: char| c.is_whitespace() || chars.contains(&c))
                                    .to_string();
                                if arg.is_empty() {
                                    !s.is_empty()
                                } else {
                                    s != arg
                                }
                            })
                            .map(|i| {
                                let s = i
                                    .to_string()
                                    .trim_matches(|c: char| c.is_whitespace() || chars.contains(&c))
                                    .to_string();
                                ContextModel::String(s)
                            })
                            .collect();
                        Ok(ContextModel::List(filtered))
                    }
                    _ => anyhow::bail!("Modifier 'trim' expects a string or list"),
                }
            }
            _ => Ok(value.clone()),
        }
    }

    /// Removes surrounding single or double quotes from any {tag} placeholder.
    fn strip_all_template_quotes(text: &str, escape: bool) -> String {
        if !escape {
            return text.to_string();
        }
        let re_clean = regex::Regex::new(r#"(['"])\{(\w+)(?:\|([^}]+))?\}(['"])"#).unwrap();
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
