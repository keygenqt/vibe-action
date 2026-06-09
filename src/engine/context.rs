//! Runtime context for tag values.
//! Stores resolved tag -> value mapping and handles {tag} expansion with pipe modifiers.

use anyhow::Result;
use regex::Regex;
use std::collections::HashMap;
use std::fmt::Write;

use crate::models::context::ContextModel;

/// @todo
pub struct ExpandedTemplate {
    /// @todo
    pub items: Vec<String>,
    /// @todo
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

    /// Fill {tag} placeholders and always return an ExpandedTemplate structure.
    pub fn fill(&self, text: &str, escape: bool) -> Result<ExpandedTemplate> {
        let re = Regex::new(r"\{(\w+)(?:\|(\w+))?\}").unwrap();

        let clean_text = Self::strip_all_template_quotes(text, escape);
        let mut results = vec![clean_text.clone()];
        let mut is_list = false;

        for cap in re.captures_iter(&clean_text) {
            let placeholder = cap.get(0).unwrap().as_str();
            let tag_name = cap.get(1).unwrap().as_str();

            if let Some(context_value) = self.values.get(tag_name) {
                let modifier = cap.get(2).map(|m| m.as_str()).unwrap_or_default();
                let processed_value = self.apply_modifier(&modifier, context_value, escape)?;

                match processed_value {
                    ContextModel::String(s) => {
                        let mut final_string = s;
                        if escape {
                            final_string = shell_words::quote(&final_string).to_string();
                        }
                        for current in results.iter_mut() {
                            *current = current.replace(placeholder, &final_string);
                        }
                    }
                    ContextModel::List(items) => {
                        is_list = true;
                        let mut next = Vec::new();
                        for item in items {
                            let mut final_string = item.to_string();
                            if escape {
                                final_string = shell_words::quote(&final_string).to_string();
                            }
                            for current in &results {
                                next.push(current.replace(placeholder, &final_string));
                            }
                        }
                        results = next;
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
        match modifier {
            "join" => {
                if let ContextModel::List(items) = value {
                    let mut buffer = String::new();
                    for (idx, item) in items.iter().enumerate() {
                        if idx > 0 {
                            buffer.push('\n');
                        }
                        let s = item.to_string();
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
            "upper" | "lower" | "trim" => {
                if let ContextModel::String(s) = value {
                    let mutated = match modifier {
                        "upper" => s.to_uppercase(),
                        "lower" => s.to_lowercase(),
                        "trim" => s.trim().to_string(),
                        _ => unreachable!(),
                    };
                    Ok(ContextModel::String(if escape {
                        shell_words::quote(&mutated).to_string()
                    } else {
                        mutated
                    }))
                } else {
                    anyhow::bail!("Modifier '{}' expects a string, but got a list", modifier)
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
