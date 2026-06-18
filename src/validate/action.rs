//! ActionModel validation.
//! Checks type, expect, match regex, and non-empty action.

use anyhow::Result;
use regex::Regex;

use crate::{
    models::action::{ActionModel, ActionRun, ExpectMode, SwitchCase},
    utils::constants,
    validate::ValidateTrait,
};

impl ValidateTrait for ActionModel {
    /// Validate action fields.
    fn validate(&self) -> Result<()> {
        // Validate switch: at most one else, and it must be last.
        if let Some(switch) = &self.switch {
            // Count else branches — only one allowed.
            let else_count = switch
                .iter()
                .filter(|s| matches!(s, SwitchCase::Else { .. }))
                .count();
            if else_count > 1 {
                anyhow::bail!("Switch in '{}' has multiple else branches.", self.tag);
            }
            // Else must be the last branch.
            if else_count == 1 && !matches!(switch.last(), Some(SwitchCase::Else { .. })) {
                anyhow::bail!("Else in '{}' must be the last branch.", self.tag);
            }
            // Case conditions must be valid {tag|modifier} placeholders.
            let case_re = Regex::new(&format!("^{}$", constants::TAG_PLACEHOLDER_PATTERN)).unwrap();
            for branch in switch {
                if let SwitchCase::Case { case, .. } = branch {
                    if !case_re.is_match(case.trim()) {
                        anyhow::bail!(
                            "Case condition in '{}' must be a single {{tag|modifier}}, got: '{}'",
                            self.tag,
                            case
                        );
                    }
                }
            }
        } else if self.action.as_ref().map_or(true, |a| a.trim().is_empty()) {
            // Action cannot be empty when switch is not set.
            anyhow::bail!("Action '{}' has empty command/prompt.", self.tag);
        }

        // LLM must have expect set (need to know what to parse).
        if self.run == ActionRun::Llm && self.expect == ExpectMode::Void {
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
