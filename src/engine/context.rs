//! Runtime context for tag values.
//! Stores resolved tag -> value mapping and handles {tag} expansion with pipe modifiers.

use anyhow::Result;
use std::collections::HashMap;
use std::collections::HashSet;

use crate::models::action::ActionRun;
use crate::models::context::ContextModel;
use crate::modifier::modifier::ModifierRegistry;
use crate::utils;

/// Result of expanding a template with context values.
pub struct ExpandedTemplate {
    /// Original template before expansion.
    pub raw: String,
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
            values: std::collections::HashMap::new(),
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
        raw: &str,
        run: &ActionRun,
        modifier: &ModifierRegistry,
    ) -> Result<ExpandedTemplate> {
        let is_encode = run == &ActionRun::Cmd;

        // Strip quotes utilizing the safe parser-driven snapshot pass
        let clean_text = Self::strip_all_template_quotes(raw, is_encode);

        let mut processed_placeholders = HashSet::new();
        let mut list_expanded_tags: HashSet<String> = HashSet::new();
        let mut results = vec![clean_text.clone()];
        let mut is_list = false;

        // Drive string replacements cleanly through our unified static singleton iterator
        for mat in crate::engine::parser::TagIterator::new(&clean_text) {
            let placeholder = mat.full_match.as_str();
            let tag_name = mat.base_tag.as_str();

            if processed_placeholders.contains(placeholder) {
                continue;
            }

            if let Some(context_value) = self.values.get(tag_name) {
                // Strictly preserve empty pipe constraints contract (e.g. "{tag|}")
                if mat.modifiers.is_empty() && placeholder.contains('|') {
                    anyhow::bail!("Empty modifier not allowed in '{}'", placeholder);
                }

                // Architectural fix: forward the pre-parsed modifiers array directly to the registry
                let processed_value = modifier.apply_modifier(&mat.modifiers, context_value)?;

                match processed_value {
                    ContextModel::List(items) => {
                        if list_expanded_tags.contains(tag_name) {
                            // Same tag already expanded — replace in each existing element
                            for (i, current) in results.iter_mut().enumerate() {
                                let idx = i % items.len();
                                let val = items[idx].to_string();
                                if is_encode {
                                    *current = current.replace(
                                        placeholder,
                                        &shell_words::quote(&val).to_string(),
                                    );
                                } else {
                                    *current = current.replace(placeholder, &val);
                                }
                            }
                        } else {
                            // First list expansion for this tag
                            is_list = true;
                            list_expanded_tags.insert(tag_name.to_string());
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
                        }
                    }
                    _ => {
                        let mut final_string = processed_value.to_string();
                        if is_encode {
                            final_string = shell_words::quote(&final_string).to_string();
                        }
                        if utils::image::is_image(&final_string) {
                            final_string = format!(" {} ", final_string);
                        }
                        for current in results.iter_mut() {
                            *current = current.replace(placeholder, &final_string);
                        }
                    }
                }
            }
            processed_placeholders.insert(placeholder.to_string());
        }

        Ok(ExpandedTemplate {
            raw: raw.to_string(),
            items: results,
            is_list,
        })
    }

    /// Removes surrounding single or double quotes from any {tag} placeholder.
    fn strip_all_template_quotes(text: &str, escape: bool) -> String {
        if !escape {
            return text.to_string();
        }

        let mut result = text.to_string();

        // Leverage TagIterator to safely process all valid unescaped placeholders
        for mat in crate::engine::parser::TagIterator::new(text) {
            let placeholder = mat.full_match.as_str();
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
