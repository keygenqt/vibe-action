//! Engine orchestrator — resolves tags, executes steps, validates results.

use anyhow::Result;
use regex::Regex;

use crate::engine::cluster::Cluster;
use crate::engine::context::Context;
use crate::engine::resolve::Resolve;
use crate::engine::shell::Shell;
use crate::engine::sort::TopologicalSort;
use crate::models::action::{ActionMode, ActionModel, ExpectMode};
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
    match_regex: Option<Regex>,
}

impl Engine {
    /// Create a new engine from a flow, sorting actions by dependencies.
    pub fn new(flow: &FlowModel) -> Result<Self> {
        Ok(Self {
            ctx: Context::new(),
            actions: TopologicalSort::sort(flow)?,
            output: flow.output.clone(),
            match_regex: match &flow.r#match {
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
        let filled = self.ctx.fill_join(&action.action, is_encode);
        match action.r#type {
            ActionMode::Cmd => filled
                .replace(" && ", " \\\n  && ")
                .replace(" | ", " \\\n  | ")
                .replace(" ; ", " \\\n  ; "),
            ActionMode::Llm => filled.trim().to_string(),
        }
    }

    /// Get the validated result.
    pub fn result(&self) -> Result<String> {
        let value = self
            .ctx
            .get(&self.output)
            .map(|v| v.to_string())
            .unwrap_or_default();

        if let Some(re) = &self.match_regex {
            if !re.is_match(&value) {
                anyhow::bail!("Result does not match pattern '{}': {}", re.as_str(), value);
            }
        }
        Ok(value)
    }

    /// Compile match regex, execute action, resolve and validate result, store in context.
    pub async fn exec_action(&mut self, action: &ActionModel) -> Result<String> {
        let compiled_match = match &action.r#match {
            Some(pattern) => Some(Regex::new(pattern)?),
            None => None,
        };
        let raw_outputs = Self::execute_action(action, &self.ctx).await?;
        let validated =
            Resolve::resolve(raw_outputs.clone(), &action.expect, compiled_match.as_ref())
                .map_err(|e| anyhow::anyhow!("[{}] {}", action.tag, e))?;
        self.ctx.set(&action.tag, validated);
        Ok(raw_outputs.join("\n"))
    }

    /// Execute an action and return raw string outputs.
    async fn execute_action(action: &ActionModel, ctx: &Context) -> Result<Vec<String>> {
        let escape = action.r#type == ActionMode::Cmd;

        // If the action expects a List return type, use cartesian expansion.
        // Otherwise, compress list tags into a single scalar block inline.
        let expanded = match &action.expect {
            ExpectMode::List(_) => ctx.fill(&action.action, escape),
            _ => vec![ctx.fill_join(&action.action, escape)],
        };

        let mut results = Vec::with_capacity(expanded.len());
        for single_action in &expanded {
            let raw = Self::exec_raw(action, single_action).await?;
            Self::log_action(
                &action.tag,
                &action.r#type,
                &action.action,
                single_action,
                &raw,
            );
            results.push(raw);
        }
        Ok(results)
    }

    /// Execute raw command or LLM prompt.
    async fn exec_raw(action: &ActionModel, resolved: &str) -> Result<String> {
        match action.r#type {
            ActionMode::Cmd => Shell::exec(resolved).await,
            ActionMode::Llm => Cluster::exec(resolved).await,
        }
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
