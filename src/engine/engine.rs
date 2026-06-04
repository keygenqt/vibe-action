//! Engine orchestrator — resolves tags, executes steps, validates results.

use anyhow::Result;
use regex::Regex;

use crate::engine::cluster::Cluster;
use crate::engine::context::Context;
use crate::engine::resolver::Resolver;
use crate::engine::shell::Shell;
use crate::models::action::ActionMode;
use crate::models::context::ContextModel;
use crate::models::flow::FlowModel;

/// Progress info for a running step.
#[derive(Debug, Clone)]
pub struct StepProgress {
    /// Step tag name.
    pub tag: String,
    /// Step type: cmd or llm.
    pub step_type: String,
    /// Percent complete (0.0-100.0).
    pub percent: f32,
    /// Current step number.
    pub current: usize,
    /// Total steps (including trigger).
    pub total: usize,
}

pub struct Engine;

impl Engine {
    /// Run a flow from start to finish.
    pub async fn run(flow: &FlowModel, on_progress: Option<fn(&StepProgress)>) -> Result<String> {
        let mut ctx = Context::new();
        let order = Resolver::resolve(flow)?;
        let total = order.len() + 1;

        tracing::info!(
            "Starting flow: {} ({} steps + trigger)",
            flow.keys.join(", "),
            order.len()
        );

        for (i, action) in order.iter().enumerate() {
            let percent = (i as f32 / total as f32) * 100.0;

            if let Some(cb) = on_progress {
                cb(&StepProgress {
                    tag: action.tag.clone(),
                    step_type: format!("{:?}", action.r#type),
                    percent,
                    current: i + 1,
                    total,
                });
            }

            let compiled_match = match &action.r#match {
                Some(pattern) => Some(Regex::new(pattern)?),
                None => None,
            };

            let resolved_action = ctx.substitute(&action.action);
            tracing::debug!(
                "[{}/{}] {} ({:?}): {}",
                i + 1,
                order.len(),
                action.tag,
                action.r#type,
                resolved_action
            );

            let raw = match action.r#type {
                ActionMode::Cmd => {
                    let result = Shell::exec(&resolved_action).await?;
                    tracing::debug!("  <- {} ({} bytes)", action.tag, result.to_string().len());
                    result
                }
                ActionMode::Llm => {
                    tracing::debug!("  -> calling LLM...");
                    let result = Cluster::exec(&resolved_action).await?;
                    let preview: String = result.to_string().chars().take(100).collect();
                    tracing::debug!("  <- {}: {}", action.tag, preview);
                    result
                }
            };

            let validated = ContextModel::from_str(
                &action.tag,
                &raw.to_string(),
                &action.expect,
                compiled_match.as_ref(),
            )?;

            ctx.set(&action.tag, validated);
        }

        // Trigger.
        if let Some(cb) = on_progress {
            cb(&StepProgress {
                tag: flow.trigger.tag.clone(),
                step_type: format!("{:?}", flow.trigger.r#type),
                percent: 100.0,
                current: total,
                total,
            });
        }

        let compiled_trigger_match = match &flow.trigger.r#match {
            Some(pattern) => Some(Regex::new(pattern)?),
            None => None,
        };

        let resolved_trigger = ctx.substitute(&flow.trigger.action);
        tracing::debug!(
            "[trigger] {} ({:?}): {}",
            flow.trigger.tag,
            flow.trigger.r#type,
            resolved_trigger
        );

        let result = match flow.trigger.r#type {
            ActionMode::Cmd => Shell::exec(&resolved_trigger).await?,
            ActionMode::Llm => Cluster::exec(&resolved_trigger).await?,
        };

        let validated = ContextModel::from_str(
            &flow.trigger.tag,
            &result.to_string(),
            &flow.trigger.expect,
            compiled_trigger_match.as_ref(),
        )?;

        tracing::info!("Flow completed: {}", flow.trigger.tag);
        Ok(validated.to_string())
    }
}
