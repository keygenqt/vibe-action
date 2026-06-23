//! ActionModel validation.
//! Checks type, expect, match regex, and non-empty action.

use anyhow::Result;
use regex::Regex;

use crate::{
    models::action::{ActionModel, ActionRun, ActionValue, ExpectMode},
    utils::constants,
    validate::ValidateTrait,
};

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
                let case_re =
                    Regex::new(&format!("^{}$", constants::TAG_PLACEHOLDER_PATTERN)).unwrap();
                for case in cases {
                    if case.when != "true" && !case_re.is_match(case.when.trim()) {
                        anyhow::bail!(
                            "When condition in '{}' must be a single {{tag|modifier}} or 'true', got: '{}'",
                            self.tag,
                            case.when
                        );
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
