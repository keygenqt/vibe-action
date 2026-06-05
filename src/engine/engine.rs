//! Engine orchestrator — resolves tags, executes steps, validates results.

use anyhow::Result;
use regex::Regex;

use crate::engine::cluster::Cluster;
use crate::engine::context::Context;
use crate::engine::resolver::Resolver;
use crate::engine::shell::Shell;
use crate::models::action::{ActionMode, ActionModel, ExpectMode};
use crate::models::context::ContextModel;
use crate::models::flow::FlowModel;

/// Progress info for a running step.
#[derive(Debug, Clone)]
pub struct StepProgress {
    pub tag: String,
    pub step_type: String,
    pub percent: f32,
    pub current: usize,
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
            flow.name,
            order.len()
        );

        for (i, action) in order.iter().enumerate() {
            if let Some(cb) = on_progress {
                cb(&StepProgress {
                    tag: action.tag.clone(),
                    step_type: format!("{:?}", action.r#type),
                    percent: ((i + 1) as f32 / total as f32) * 100.0,
                    current: i + 1,
                    total,
                });
            }

            let compiled_match = match &action.r#match {
                Some(pattern) => Some(Regex::new(pattern)?),
                None => None,
            };

            let raw = Self::execute_action(action, &ctx).await?;

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

        let result = Self::execute_action(&flow.trigger, &ctx).await?;

        let validated = ContextModel::from_str(
            &flow.trigger.tag,
            &result.to_string(),
            &flow.trigger.expect,
            compiled_trigger_match.as_ref(),
        )?;

        tracing::info!("Flow completed: {}", flow.trigger.tag);

        if matches!(flow.trigger.expect, ExpectMode::Void) {
            Ok(String::new())
        } else {
            Ok(validated.to_string())
        }
    }

    /// Debug log for action execution.
    fn debug_log(tag: &str, r#type: &ActionMode, original: &str, resolved: &str, result: &str) {
        let preview_original: String = original.chars().take(300).collect();
        let preview_resolved: String = resolved.chars().take(300).collect();
        let preview_result: String = result.chars().take(300).collect();
        tracing::debug!(
            r#"[{}] ({})
------------- original (len:{})
{}
------------- resolved (len:{})
{}
------------- result (len:{})
{}
-------------"#,
            tag,
            match r#type {
                ActionMode::Cmd => "cmd",
                ActionMode::Llm => "llm",
            },
            original.len(),
            preview_original,
            resolved.len(),
            preview_resolved,
            result.len(),
            preview_result,
        );
    }

    /// Execute a single action, handling list<T> inputs.
    async fn execute_action(action: &ActionModel, ctx: &Context) -> Result<ContextModel> {
        if let Some((list_tag, list_values)) = ctx.get_list_for_action(&action.action) {
            let is_list_expect = matches!(&action.expect, ExpectMode::List(_));
            let mut results = Vec::new();

            for item in &list_values {
                let single_action = action
                    .action
                    .replace(&format!("{{{}}}", list_tag), &item.to_string());
                let raw = Self::exec_raw(action, &single_action).await?;
                Self::debug_log(
                    &action.tag,
                    &action.r#type,
                    &action.action,
                    &single_action,
                    &raw.to_string(),
                );
                results.push(raw);
            }

            if is_list_expect {
                Ok(ContextModel::List(results))
            } else {
                let joined: Vec<String> = results.iter().map(|r| r.to_string()).collect();
                Ok(ContextModel::String(joined.join("\n")))
            }
        } else {
            let resolved = ctx.substitute(&action.action);
            let result = Self::exec_raw(action, &resolved).await?;
            Self::debug_log(
                &action.tag,
                &action.r#type,
                &action.action,
                &resolved,
                &result.to_string(),
            );
            Ok(result)
        }
    }

    /// Execute raw command or LLM prompt.
    async fn exec_raw(action: &ActionModel, resolved: &str) -> Result<ContextModel> {
        match action.r#type {
            ActionMode::Cmd => Shell::exec(resolved).await,
            ActionMode::Llm => Cluster::exec(resolved).await,
        }
    }
}
