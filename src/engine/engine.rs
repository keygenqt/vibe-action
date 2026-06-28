//! Engine orchestrator — resolves tags, executes steps, validates results.

use anyhow::Result;
use regex::Regex;

use crate::engine::cluster::Cluster;
use crate::engine::context::Context;
use crate::engine::context::ExpandedTemplate;
use crate::engine::resolve::Resolve;
use crate::engine::shell::Shell;
use crate::engine::sort::TopologicalSort;
use crate::models::action::ActionModel;
use crate::models::action::ActionRun;
use crate::models::action::ActionValue;
use crate::models::context::ContextModel;
use crate::models::flow::FlowModel;
use crate::modifier::modifier::ModifierRegistry;
use crate::print_trace;
use crate::utils;

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
        for (name, value) in &flow.input_tags {
            ctx.set(name, value.clone());
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
            .unwrap_or_default()
            .trim()
            .to_string();

        if let Some(re) = &self.check_regex {
            if !re.is_match(&value) {
                anyhow::bail!("Result does not match pattern '{}': {}", re.as_str(), value);
            }
        }
        Ok(value)
    }

    /// Execute action: fill tags, run shell/LLM, validate with regex, resolve types, store in context.
    pub async fn exec_action(&mut self, action: &ActionModel) -> Result<()> {
        let (outputs, is_list) = self.execute_action(action).await?;

        let data = if is_list {
            let inner_expect = match &action.expect {
                Some(ContextModel::List(_)) => Some(ContextModel::String(String::new())),
                other => other.clone(),
            };

            let mut parsed_items = Vec::with_capacity(outputs.len());
            for item in outputs {
                if let Some(validated) = Resolve::resolve(&item, &inner_expect)? {
                    match validated {
                        ContextModel::String(s) => parsed_items.push(s),
                        ContextModel::List(mut l) => parsed_items.append(&mut l),
                    }
                }
            }
            ContextModel::List(parsed_items)
        } else {
            let raw_single = outputs.into_iter().next().unwrap_or_default();
            match Resolve::resolve(&raw_single, &action.expect)? {
                Some(value) => value,
                None => return Ok(()),
            }
        };

        self.ctx.set(&action.tag, data);
        Ok(())
    }

    /// Resolve the effective action from switch or action field.
    fn resolve_action(&self, resolve_action: &ActionModel) -> Result<ExpandedTemplate> {
        match &resolve_action.action {
            ActionValue::Simple(raw) => {
                return self.ctx.fill(raw, &resolve_action.run, &self.modifier);
            }
            ActionValue::Switch(cases) => {
                for case in cases {
                    let full = format!("__case={};{}", case.when, case.then);
                    let template = self.ctx.fill(&full, &resolve_action.run, &self.modifier)?;
                    let mut result: Vec<String> = Vec::with_capacity(template.items.len());
                    for item in &template.items {
                        if let Some(rest) = item.strip_prefix("__case=true;") {
                            result.push(rest.to_string());
                        }
                    }
                    if !result.is_empty() {
                        return Ok(ExpandedTemplate {
                            raw: case.then.clone(),
                            items: result,
                            is_list: template.is_list,
                        });
                    }
                }
            }
        }

        anyhow::bail!("No case matched in switch '{}'", resolve_action.tag);
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
                    Self::validate_output(&action.tag, &raw, &compiled_check)?;
                    Self::log_action(&action.tag, &action.run, &expanded.raw, single_action, &raw);
                    results.push(raw);
                }
            }
            ActionRun::Vision => {
                let mut all_images = Vec::new();
                let mut cleaned_items = Vec::new();
                for item in &expanded.items {
                    let (cleaned, images) = utils::format::format_image_prompt(item);
                    cleaned_items.push(cleaned);
                    all_images.extend(images);
                }
                let cluster_outputs = Cluster::exec(
                    &self.system,
                    self.retries,
                    &action.run,
                    &cleaned_items,
                    if all_images.is_empty() {
                        None
                    } else {
                        Some(all_images)
                    },
                )
                .await?;
                for cluster_res in cluster_outputs {
                    Self::validate_output(&action.tag, &cluster_res.result, &compiled_check)?;
                    Self::log_action(
                        &action.tag,
                        &action.run,
                        &expanded.raw,
                        &cluster_res.prompt,
                        &cluster_res.result,
                    );
                    results.push(cluster_res.result);
                }
            }
            ActionRun::Tiny | ActionRun::Small | ActionRun::Medium | ActionRun::Large => {
                let cluster_outputs = Cluster::exec(
                    &self.system,
                    self.retries,
                    &action.run,
                    &expanded.items,
                    None,
                )
                .await?;
                for cluster_res in cluster_outputs {
                    Self::validate_output(&action.tag, &cluster_res.result, &compiled_check)?;
                    Self::log_action(
                        &action.tag,
                        &action.run,
                        &expanded.raw,
                        &cluster_res.prompt,
                        &cluster_res.result,
                    );
                    results.push(cluster_res.result);
                }
            }
            ActionRun::Value => {
                for single_action in &expanded.items {
                    Self::validate_output(&action.tag, single_action, &compiled_check)?;
                    results.push(single_action.to_string());
                }
            }
        }
        Ok((results, expanded.is_list))
    }

    /// Validate action output against optional regex pattern.
    fn validate_output(tag: &str, output: &str, check: &Option<Regex>) -> Result<()> {
        if let Some(re) = check {
            if !re.is_match(output.trim()) {
                anyhow::bail!(
                    "Result for '{}' does not match pattern '{}': '{}'",
                    tag,
                    re.as_str(),
                    output.trim()
                );
            }
        }
        Ok(())
    }

    /// Log action execution details.
    pub fn log_action(tag: &str, run: &ActionRun, original: &str, resolved: &str, result: &str) {
        let size = 2000;
        let preview_original: String = original.chars().take(size).collect();
        let preview_resolved: String = resolved.chars().take(size).collect();
        let preview_result: String = result.chars().take(size).collect();
        print_trace!(
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
                ActionRun::Value => "val",
                _ => "llm",
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
