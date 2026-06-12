//! Engine orchestrator — resolves tags, executes steps, validates results.

use anyhow::Result;
use regex::Regex;

use crate::engine::cluster::Cluster;
use crate::engine::context::Context;
use crate::engine::resolve::Resolve;
use crate::engine::shell::Shell;
use crate::engine::sort::TopologicalSort;
use crate::models::action::{ActionMode, ActionModel, ExpectMode};
use crate::models::context::ContextModel;
use crate::models::flow::FlowModel;

/// Pipeline execution engine — resolves dependencies, executes actions, manages context.
pub struct Engine {
    /// Runtime context holding resolved tag values.
    ctx: Context,
    /// Actions in topological order (dependencies first).
    actions: Vec<ActionModel>,
    /// Tag name for the final result.
    output: String,
    /// Optional regex validation for the result.
    check_regex: Option<Regex>,
}

impl Engine {
    /// Create a new engine from a flow, sorting actions by dependencies.
    pub fn new(flow: &FlowModel) -> Result<Self> {
        let sorted_actions = TopologicalSort::sort(flow)?;
        let resolved_output = sorted_actions
            .last()
            .map(|a| a.tag.clone())
            .unwrap_or_default();

        Ok(Self {
            ctx: Context::new(),
            actions: sorted_actions,
            output: resolved_output,
            check_regex: match &flow.check {
                Some(pattern) => Some(Regex::new(pattern)?),
                None => None,
            },
        })
    }

    /// Get the sorted actions.
    pub fn actions(&self) -> &[ActionModel] {
        &self.actions
    }

    /// Get the enriched action text with all tags filled, formatted for display.
    pub fn action_display(&self, action: &ActionModel) -> String {
        let is_encode = action.r#type == ActionMode::Cmd;
        let filled = match self.ctx.fill(&action.action, is_encode) {
            Ok(expanded) => expanded.items.join(
                "\n\n---------------------------------------------------------------------------\n\n",
            ),
            Err(_) => action.action.clone(),
        };
        match action.r#type {
            ActionMode::Cmd => filled
                .replace(" && ", " \\\n  && ")
                .replace(" | ", " \\\n  | ")
                .replace(" ; ", " \\\n  ; "),
            _ => filled.trim().to_string(),
        }
    }

    /// Get the validated result.
    pub fn result(&self) -> Result<String> {
        let value = self
            .ctx
            .get(&self.output)
            .map(|v| v.to_string())
            .unwrap_or_default();

        if let Some(re) = &self.check_regex {
            if !re.is_match(&value) {
                anyhow::bail!("Result does not match pattern '{}': {}", re.as_str(), value);
            }
        }
        Ok(value)
    }

    /// Execute action: fill tags, run shell/LLM, validate with regex, resolve types, store in context.
    pub async fn exec_action(&mut self, action: &ActionModel) -> Result<()> {
        // Execute action and get raw outputs with list metadata.
        let (outputs, is_list) = Self::execute_action(action, &self.ctx).await?;
        // Build the final ContextModel: list of items or single value.
        let data = if is_list {
            let inner_expect = match &action.expect {
                ExpectMode::List(inner) => inner.as_ref(),
                _ => &action.expect,
            };
            let mut parsed_items = Vec::with_capacity(outputs.len());
            for item in outputs {
                let validated = Resolve::resolve(&item, inner_expect)?;
                parsed_items.push(validated);
            }
            ContextModel::List(parsed_items)
        } else {
            let raw_single = outputs.into_iter().next().unwrap_or_default();
            Resolve::resolve(&raw_single, &action.expect)?
        };
        // Store validated result in context.
        self.ctx.set(&action.tag, data);
        Ok(())
    }

    /// Fill template, execute shell/LLM, validate each output against regex.
    async fn execute_action(action: &ActionModel, ctx: &Context) -> Result<(Vec<String>, bool)> {
        let escape = action.r#type == ActionMode::Cmd;
        let compiled_check = action.check.as_ref().map(|p| Regex::new(p)).transpose()?;
        let expanded = ctx.fill(&action.action, escape)?;
        let mut results = Vec::with_capacity(expanded.items.len());

        match action.r#type {
            ActionMode::Cmd => {
                for single_action in &expanded.items {
                    let raw = Shell::exec(single_action).await?;
                    if let Some(re) = &compiled_check {
                        if !re.is_match(raw.trim()) {
                            anyhow::bail!(
                                "Result for '{}' does not match pattern '{}': '{}'",
                                action.tag,
                                re.as_str(),
                                raw.trim()
                            );
                        }
                    }
                    Self::log_action(
                        &action.tag,
                        &action.r#type,
                        &action.action,
                        single_action,
                        &raw,
                    );
                    results.push(raw);
                }
            }
            ActionMode::Llm => {
                let cluster_outputs = Cluster::exec(&expanded.items).await?;
                for cluster_res in cluster_outputs {
                    let result = cluster_res.result;
                    if let Some(re) = &compiled_check {
                        if !re.is_match(&result) {
                            anyhow::bail!(
                                "Result for '{}' does not match pattern '{}': '{}'",
                                action.tag,
                                re.as_str(),
                                &result
                            );
                        }
                    }
                    Self::log_action(
                        &action.tag,
                        &action.r#type,
                        &action.action,
                        &cluster_res.prompt,
                        &result,
                    );
                    results.push(result);
                }
            }
            ActionMode::Value => {
                for single_action in &expanded.items {
                    results.push(single_action.to_string());
                }
            }
        }
        Ok((results, expanded.is_list))
    }

    /// Log action execution details.
    pub fn log_action(
        tag: &str,
        r#type: &ActionMode,
        original: &str,
        resolved: &str,
        result: &str,
    ) {
        let size = 2000;
        let preview_original: String = original.chars().take(size).collect();
        let preview_resolved: String = resolved.chars().take(size).collect();
        let preview_result: String = result.chars().take(size).collect();
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
                ActionMode::Value => "val",
            },
            original.len(),
            preview_original,
            resolved.len(),
            preview_resolved,
            result.len(),
            preview_result,
        );
    }
}
