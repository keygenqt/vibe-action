//! Engine orchestrator — resolves tags, executes steps, validates results.

use anyhow::Result;
use regex::Regex;

use crate::engine::cluster::Cluster;
use crate::engine::context::{Context, ExpandedTemplate};
use crate::engine::resolve::Resolve;
use crate::engine::shell::Shell;
use crate::engine::sort::TopologicalSort;
use crate::models::action::{ActionModel, ActionRun, ExpectMode, SwitchCase};
use crate::models::context::ContextModel;
use crate::models::flow::FlowModel;
use crate::modifier::modifier::ModifierRegistry;

/// Pipeline execution engine — resolves dependencies, executes actions, manages context.
pub struct Engine {
    /// Runtime context holding resolved tag values.
    ctx: Context,
    /// Registry of pipe modifiers for transforming tag values.
    modifier: ModifierRegistry,
    /// Actions in topological order (dependencies first).
    actions: Vec<ActionModel>,
    /// Tag name for the final result.
    output: String,
    /// Optional regex validation for the result.
    check_regex: Option<Regex>,
    /// Default system prompt for all LLM requests.
    pub system: String,
    /// Number of retries for failed LLM steps (default: 0).
    pub retries: u32,
}

impl Engine {
    /// Create a new engine from a flow, sorting actions by dependencies.
    pub fn new(system: &str, retries: u32, flow: &FlowModel) -> Result<Self> {
        let sorted_actions = TopologicalSort::sort(flow)?;
        let resolved_output = sorted_actions
            .last()
            .map(|a| a.tag.clone())
            .unwrap_or_default();

        // Load input arguments as context tags.
        let mut ctx = Context::new();
        for (name, value) in &flow.input_args {
            ctx.set(name, ContextModel::String(value.clone()));
        }

        Ok(Self {
            ctx,
            modifier: ModifierRegistry::new(),
            actions: sorted_actions,
            output: resolved_output,
            check_regex: match &flow.check {
                Some(pattern) => Some(Regex::new(pattern)?),
                None => None,
            },
            system: system.to_string(),
            retries,
        })
    }

    /// Get the sorted actions.
    pub fn actions(&self) -> &[ActionModel] {
        &self.actions
    }

    /// Get the enriched action text with all tags filled, formatted for display.
    pub fn action_display(&self, action: &ActionModel) -> Result<String> {
        let expanded = self.resolve_action(action)?;
        let values = expanded.items.join(
            "\n\n---------------------------------------------------------------------------\n\n",
        );
        Ok(match action.run {
            ActionRun::Cmd => values
                .replace(" && ", " \\\n  && ")
                .replace(" | ", " \\\n  | ")
                .replace(" ; ", " \\\n  ; "),
            _ => values.trim().to_string(),
        })
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
        let (outputs, is_list) = self.execute_action(action).await?;
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

    /// Resolve the effective action from switch or action field.
    fn resolve_action(&self, resolve_action: &ActionModel) -> Result<ExpandedTemplate> {
        let Some(switch) = &resolve_action.switch else {
            let raw = resolve_action.action.clone().ok_or_else(|| {
                anyhow::anyhow!(
                    "Action '{}' has no action and no switch",
                    resolve_action.tag
                )
            })?;
            return self.ctx.fill(&raw, &resolve_action.run, &self.modifier);
        };

        struct ResolvedBranch {
            cases: ExpandedTemplate,
            actions: ExpandedTemplate,
        }

        let mut switch_cases: Vec<ResolvedBranch> = Vec::with_capacity(switch.len());
        let mut switch_else: Option<ExpandedTemplate> = None;

        for branch in switch {
            match branch {
                SwitchCase::Case { case, action } => {
                    let cases = self.ctx.fill(case, &ActionRun::Value, &self.modifier)?;
                    let actions = self.ctx.fill(action, &resolve_action.run, &self.modifier)?;
                    // Check
                    if cases.items.len() != actions.items.len() {
                        anyhow::bail!("Error");
                    }
                    // Save for resolve
                    switch_cases.push(ResolvedBranch {
                        cases: cases,
                        actions,
                    });
                }
                SwitchCase::Else { action } => {
                    switch_else =
                        Some(self.ctx.fill(action, &resolve_action.run, &self.modifier)?);
                }
            }
        }

        let mut template = ExpandedTemplate::default();
        for switch in switch_cases {
            for (i, action_item) in switch.actions.items.iter().enumerate() {
                if !template.items.contains(action_item) && switch.cases.items[i] == "true" {
                    template.raw = switch.actions.raw.clone();
                    template.is_list = switch.actions.is_list;
                    template.items.push(action_item.clone());
                }
            }
        }

        if template.items.is_empty() {
            if let Some(switch_else) = switch_else {
                return Ok(switch_else);
            } else {
                anyhow::bail!("Error");
            }
        }

        Ok(template)
    }

    /// Fill template, execute shell/LLM, validate each output against regex.
    async fn execute_action(&self, action: &ActionModel) -> Result<(Vec<String>, bool)> {
        let compiled_check = action.check.as_ref().map(|p| Regex::new(p)).transpose()?;

        let expanded = self.resolve_action(action)?;
        let mut results = Vec::with_capacity(expanded.items.len());

        match action.run {
            ActionRun::Cmd => {
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
                    Self::log_action(&action.tag, &action.run, &expanded.raw, single_action, &raw);
                    results.push(raw);
                }
            }
            ActionRun::Llm | ActionRun::LlmSmall | ActionRun::LlmMedium | ActionRun::LlmLarge => {
                let cluster_outputs =
                    Cluster::exec(&self.system, self.retries, &action.run, &expanded.items).await?;
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
                        &action.run,
                        &expanded.raw,
                        &cluster_res.prompt,
                        &result,
                    );
                    results.push(result);
                }
            }
            ActionRun::Value => {
                for single_action in &expanded.items {
                    if let Some(re) = &compiled_check {
                        if !re.is_match(single_action.trim()) {
                            anyhow::bail!(
                                "Result for '{}' does not match pattern '{}': '{}'",
                                action.tag,
                                re.as_str(),
                                single_action.trim()
                            );
                        }
                    }
                    results.push(single_action.to_string());
                }
            }
        }
        Ok((results, expanded.is_list))
    }

    /// Log action execution details.
    pub fn log_action(tag: &str, run: &ActionRun, original: &str, resolved: &str, result: &str) {
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
            match run {
                ActionRun::Cmd => "cmd",
                ActionRun::Llm
                | ActionRun::LlmSmall
                | ActionRun::LlmMedium
                | ActionRun::LlmLarge => "llm",
                ActionRun::Value => "val",
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
