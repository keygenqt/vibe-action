//! Runtime context for tag values.
//! Stores resolved tag -> value mapping and handles {tag} expansion with pipe modifiers.

use anyhow::Result;
use regex::Regex;
use std::collections::{HashMap, HashSet};

use crate::models::action::{ActionMode, ActionModel};
use crate::models::context::ContextModel;
use crate::modifier::modifier::ModifierRegistry;
use crate::utils::constants;

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
    pub fn fill(
        &self,
        action: &ActionModel,
        modifier: &ModifierRegistry,
    ) -> Result<ExpandedTemplate> {
        let is_encode = action.r#type == ActionMode::Cmd;
        let re = Regex::new(constants::TAG_PLACEHOLDER_PATTERN).unwrap();

        let clean_text = Self::strip_all_template_quotes(&action.action, is_encode);
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
                let modifier_value = cap.get(2).map(|m| m.as_str()).unwrap_or_default();
                if modifier_value.is_empty() && placeholder.contains('|') {
                    anyhow::bail!("Empty modifier not allowed in '{}'", placeholder);
                }
                let processed_value = modifier.apply_modifier(modifier_value, context_value)?;
                match processed_value {
                    ContextModel::String(s) => {
                        let mut final_string = s;
                        if is_encode {
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
                            if is_encode {
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

    /// Removes surrounding single or double quotes from any {tag} placeholder.
    fn strip_all_template_quotes(text: &str, escape: bool) -> String {
        if !escape {
            return text.to_string();
        }
        let re = regex::Regex::new(constants::TAG_PLACEHOLDER_PATTERN).unwrap();
        let mut result = text.to_string();
        for cap in re.captures_iter(text) {
            let placeholder = cap.get(0).unwrap().as_str();
            for quote in &["'", "\""] {
                let quoted = format!("{}{}{}", quote, placeholder, quote);
                if result.contains(&quoted) {
                    result = result.replace(&quoted, placeholder);
                }
            }
        }
        result
    }
}
