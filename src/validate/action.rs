//! ActionModel validation.
//! Checks non-empty action, val-candidate fields, and action interpolation.

use std::collections::HashSet;
use std::sync::OnceLock;

use anyhow::Result;
use regex::Regex;

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
                    let (split_str, merge_str) = each.resolve();
                    if split_str.is_empty() {
                        anyhow::bail!(
                            "Action '{}': val '{}' each.split is empty.",
                            self.tag,
                            c.name
                        );
                    }
                    if merge_str.is_empty() {
                        anyhow::bail!(
                            "Action '{}': val '{}' each.merge is empty.",
                            self.tag,
                            c.name
                        );
                    }
                }
            }

            // `action` is pure `{name}` interpolation: only declared names.
            let declared: HashSet<&str> = candidates.iter().map(|c| c.name.as_str()).collect();
            static RE: OnceLock<Regex> = OnceLock::new();
            let re = RE.get_or_init(|| Regex::new(r"\{(\w+)\}").unwrap());
            for cap in re.captures_iter(&self.action) {
                let name = cap.get(1).unwrap().as_str();
                if !declared.contains(name) {
                    anyhow::bail!(
                        "Action '{}': `action` references undeclared name '{{{}}}'.",
                        self.tag,
                        name
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
