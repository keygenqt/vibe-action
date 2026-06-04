//! LLM cluster executor.
//! Sends prompts to vibe-cluster and returns model responses.

use anyhow::Result;
use vibe_cluster::Prompt;

use crate::{configs::app::AppConfig, models::context::ContextModel};

pub struct Cluster;

impl Cluster {
    /// Execute an LLM prompt and return the response.
    pub async fn exec(prompt: &str) -> Result<ContextModel> {
        let complexity = Self::complexity(prompt).await.unwrap_or(0.5);
        let config = AppConfig::instance()?;
        let cluster = config.create_cluster(complexity)?;
        let response = cluster
            .call(Prompt::new(prompt, None))
            .await
            .map_err(|e| anyhow::anyhow!("Cluster error: {}", e))?
            .text
            .ok_or_else(|| anyhow::anyhow!("Empty response from cluster"))?;
        Ok(ContextModel::String(response.trim().to_string()))
    }

    /// Estimate complexity for a prompt.
    async fn complexity(prompt: &str) -> Result<f32> {
        let config = AppConfig::instance()?;
        let estimator = config.create_estimator()?;

        let complexity_prompt = format!(
            "Rate prompt complexity from 0.00 to 1.00 based on:\n\
             - Depth of knowledge required (shallow -> deep expertise)\n\
             - Code involvement (no code -> architecture/creative)\n\
             - Analytical demand (simple recall -> comparison/synthesis)\n\
             - Output complexity (few words -> structured document/code)\n\n\
             Reply ONLY with the number (two decimal places, e.g. 0.50).\n\n\
             <prompt>\n{prompt}\n</prompt>"
        );

        let response = estimator
            .call(Prompt::new(&complexity_prompt, None))
            .await
            .map_err(|e| anyhow::anyhow!("Estimator error: {}", e))?
            .text
            .ok_or_else(|| anyhow::anyhow!("Empty response from estimator"))?;

        let result: f32 = response
            .trim()
            .replace(',', ".")
            .parse()
            .map_err(|_| anyhow::anyhow!("Invalid complexity response: '{}'", response))?;

        Ok(((result.clamp(0.0, 1.0)) * 100.0).round() / 100.0)
    }
}
