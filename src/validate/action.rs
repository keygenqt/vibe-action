//! ActionModel validation.
//! Checks type, expect, match regex, and non-empty action.

use anyhow::Result;
use regex::Regex;

use crate::models::action::ActionModel;
use crate::models::action::ActionRun;
use crate::models::action::ActionValue;
use crate::models::action::ExpectMode;
use crate::validate::ValidateTrait;

impl ValidateTrait for ActionModel {
    /// Validate action fields.
    fn validate(&self) -> Result<()> {
        // Validate action: switch cases or simple string must be valid.
        match &self.action {
            ActionValue::Simple(s) => {
                if s.trim().is_empty() {
                    anyhow::bail!("Action '{}' has empty command/prompt.", self.tag);
                }
            }
            ActionValue::Switch(cases) => {
                for case in cases {
                    let trimmed_when = case.when.trim();
                    if trimmed_when != "true" {
                        // Initialize our standalone iterator to inspect the condition layout
                        let mut iter = crate::engine::parser::TagIterator::new(trimmed_when);

                        match (iter.next(), iter.next()) {
                            (Some(mat), None) => {
                                // The placeholder must span across the exact entirety of the when string
                                if mat.full_match.len() != trimmed_when.len() {
                                    anyhow::bail!(
                                        "When condition in '{}' must be a single {{tag|modifier}} or 'true', got: '{}'",
                                        self.tag,
                                        case.when
                                    );
                                }
                            }
                            _ => {
                                // Fails if 0 placeholders or multiple placeholders are detected
                                anyhow::bail!(
                                    "When condition in '{}' must be a single {{tag|modifier}} or 'true', got: '{}'",
                                    self.tag,
                                    case.when
                                );
                            }
                        }
                    }
                }
            }
        }

        // LLM actions must have expect set (need to know what to parse).
        let is_llm = matches!(
            self.run,
            ActionRun::Tiny
                | ActionRun::Small
                | ActionRun::Medium
                | ActionRun::Large
                | ActionRun::Vision
        );
        if is_llm && self.expect == ExpectMode::Void {
            anyhow::bail!(
                "LLM action '{}' cannot have expect: void. Specify what to expect.",
                self.tag
            );
        }

        // Validate check regex if present.
        if let Some(pattern) = &self.check {
            Regex::new(pattern).map_err(|e| {
                anyhow::anyhow!("Action '{}' has invalid check regex: {}", self.tag, e)
            })?;
        }

        Ok(())
    }
}
