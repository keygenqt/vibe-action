//! AppConfig validation.
//! Checks estimator and cluster configuration.

use anyhow::Result;

use crate::{configs::app::AppConfig, validate::ValidateTrait};

impl ValidateTrait for AppConfig {
    /// Validate configuration.
    fn validate(&self) -> Result<()> {
        // Validate estimator.
        self.estimator.validate()?;
        // Validate each cluster node.
        for node in &self.cluster {
            node.validate()?;
        }
        // Check complexity ranges cover 0.0-1.0 with 0.1 step.
        let mut covered = [false; 11]; // 0.0, 0.1, ..., 1.0
        for node in &self.cluster {
            let from = (node.complexity_from * 10.0).round() as usize;
            let to = (node.complexity_to * 10.0).round() as usize;
            for step in from..=to {
                covered[step] = true;
            }
        }
        if let Some(gap) = covered.iter().position(|&c| !c) {
            anyhow::bail!(
                "Complexity gap at {:.1}. No model covers this range.",
                gap as f32 / 10.0
            );
        }
        Ok(())
    }
}
