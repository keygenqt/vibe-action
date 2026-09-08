//! ActionModel validation.
//! Checks non-empty action, val-candidate fields, and action interpolation.

use std::collections::HashSet;

use anyhow::Result;
use regex::Regex;

use crate::engine::parser::TagIterator;
use crate::models::action::ActionModel;
use crate::validate::ValidateTrait;

impl ValidateTrait for ActionModel {
    fn validate(&self) -> Result<()> {
        if self.action.trim().is_empty() {
            anyhow::bail!("Action '{}' has empty command/prompt.", self.tag);
        }

        let candidates = self.val.as_ref().map(|v| v.as_slice()).unwrap_or(&[]);
        if !candidates.is_empty() {
            for (i, c) in candidates.iter().enumerate() {
                if c.name.trim().is_empty() {
                    anyhow::bail!("Action '{}': val[{}] has empty name.", self.tag, i);
                }
                if c.data.trim().is_empty() {
                    anyhow::bail!("Action '{}': val '{}' has empty data.", self.tag, c.name);
                }
                if let Some(each) = &c.each {
                    if each.split.is_empty() {
                        anyhow::bail!(
                            "Action '{}': val '{}' each.split is empty.",
                            self.tag,
                            c.name
                        );
                    }
                    if each.merge.is_empty() {
                        anyhow::bail!(
                            "Action '{}': val '{}' each.merge is empty.",
                            self.tag,
                            c.name
                        );
                    }
                }
            }

            // `action` is pure `{name}` interpolation: only declared names, no modifiers.
            let declared: HashSet<&str> = candidates.iter().map(|c| c.name.as_str()).collect();
            for mat in TagIterator::new(&self.action) {
                if !mat.modifiers.is_empty() {
                    anyhow::bail!(
                        "Action '{}': modifiers are not allowed in `action` (found '{}').",
                        self.tag,
                        mat.full_match
                    );
                }
                if !declared.contains(mat.base_tag.as_str()) {
                    anyhow::bail!(
                        "Action '{}': `action` references undeclared name '{{{}}}'.",
                        self.tag,
                        mat.base_tag
                    );
                }
            }
        }

        if let Some(pattern) = &self.check {
            Regex::new(pattern).map_err(|e| {
                anyhow::anyhow!("Action '{}' has invalid check regex: {}", self.tag, e)
            })?;
        }

        Ok(())
    }
}
