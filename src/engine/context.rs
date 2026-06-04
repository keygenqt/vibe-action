//! Runtime context for tag values.
//! Stores resolved tag -> value mapping and handles {tag} substitution.

use std::collections::HashMap;

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

    /// Replace all {tag} placeholders in text with their resolved values.
    /// Unknown tags are left unchanged.
    pub fn substitute(&self, text: &str) -> String {
        let re = regex::Regex::new(r"\{(\w+)\}").unwrap();
        re.replace_all(text, |caps: &regex::Captures| {
            let tag = &caps[1];
            self.values
                .get(tag)
                .map(|v| v.to_string())
                .unwrap_or_else(|| caps.get(0).unwrap().as_str().to_string())
        })
        .to_string()
    }

    /// Check if any {tag} in text points to a List value.
    /// Returns the tag name and the list items if found.
    pub fn get_list_for_action(&self, text: &str) -> Option<(String, Vec<ContextModel>)> {
        let re = regex::Regex::new(r"\{(\w+)\}").unwrap();
        for cap in re.captures_iter(text) {
            let tag = cap.get(1).unwrap().as_str();
            if let Some(ContextModel::List(items)) = self.values.get(tag) {
                return Some((tag.to_string(), items.clone()));
            }
        }
        None
    }
}
