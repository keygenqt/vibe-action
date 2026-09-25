//! ActionModel validation.
//! See [`crate::validate`] module-level docs for validation rules.

use std::collections::HashSet;
use std::sync::OnceLock;

use anyhow::Result;
use regex::Regex;

use crate::models::action::ActionModel;
use crate::operator::inspect::inspect::InspectKey;
use crate::operator::operator::OperatorKey;
use crate::utils::escape;
use crate::validate::ValidateTrait;

/// Extract operator names from a mods/when string.
/// Splits on unescaped `|`, takes the part before the first unescaped `:`.
/// Braces escape separators, mirroring apply_ops parsing.
fn operator_names(s: &str) -> Vec<String> {
    escape::split_escaped(s, '|')
        .iter()
        .map(|p| {
            let p = p.trim();
            match escape::find_unescaped(p, ':') {
                Some(idx) => p[..idx].trim().to_string(),
                None => p.to_string(),
            }
        })
        .filter(|n| !n.is_empty())
        .collect()
}

impl ValidateTrait for ActionModel {
    fn validate(&self) -> Result<()> {
        if self.action.trim().is_empty() {
            anyhow::bail!("Action '{}' has empty command/prompt.", self.tag);
        }

        // Action-level when: inspect operators only.
        if let Some(when) = &self.when {
            for name in operator_names(when) {
                if InspectKey::from_str(&name).is_none() {
                    anyhow::bail!(
                        "Action '{}': 'when' allows inspect operators only, got '{}'.",
                        self.tag,
                        name
                    );
                }
            }
        }

        let candidates = self.val.as_ref().map(|v| v.as_slice()).unwrap_or(&[]);
        if !candidates.is_empty() {
            for (i, c) in candidates.iter().enumerate() {
                if c.name.trim().is_empty() {
                    anyhow::bail!("Action '{}': val[{}] has empty name.", self.tag, i);
                }
                // Validate mods: all operator names must be known.
                if let Some(mods) = &c.mods {
                    for name in operator_names(mods) {
                        if OperatorKey::from_str(&name).is_none() {
                            anyhow::bail!(
                                "Action '{}': val '{}' uses unknown operator '{}' in mods.",
                                self.tag,
                                c.name,
                                name
                            );
                        }
                    }
                }
                // Validate when: inspect operators only.
                if let Some(when) = &c.when {
                    for name in operator_names(when) {
                        if InspectKey::from_str(&name).is_none() {
                            anyhow::bail!(
                                "Action '{}': val '{}' uses non-inspect operator '{}' in when.",
                                self.tag,
                                c.name,
                                name
                            );
                        }
                    }
                }
                // Validate fail: inspect operators only (post-mods hard check).
                if let Some(fail) = &c.fail {
                    for name in operator_names(fail) {
                        if InspectKey::from_str(&name).is_none() {
                            anyhow::bail!(
                                "Action '{}': val '{}' uses non-inspect operator '{}' in fail.",
                                self.tag,
                                c.name,
                                name
                            );
                        }
                    }
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

        if let Some(pattern) = &self.reg {
            Regex::new(pattern).map_err(|e| {
                anyhow::anyhow!("Action '{}' has invalid check regex: {}", self.tag, e)
            })?;
        }

        Ok(())
    }
}
