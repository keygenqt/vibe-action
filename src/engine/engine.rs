//! Engine orchestrator — resolves tags, executes steps, validates results.

use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

use anyhow::Result;
use regex::Regex;

use crate::engine::cluster::Cluster;
use crate::engine::log::log_action;
use crate::engine::shell::Shell;
use crate::models::action::ActionModel;
use crate::models::action::ActionRun;
use crate::models::pipeline::PipelineModel;
use crate::operator::operator::ITEM_SEP;
use crate::operator::operator::OperatorKey;
use crate::operator::operator::OperatorRegistry;
use crate::operator::operator::truthy;
use crate::utils;
use crate::utils::yaml::expand_escapes;

pub struct Engine {
    system: String,
    retries: u32,
    actions: Vec<ActionModel>,
    available: HashMap<String, String>,
    operators: OperatorRegistry,
}

impl Engine {
    /// Create a new engine from a pipeline, sorting actions by dependencies.
    pub fn new(system: &str, retries: u32, pipeline: &PipelineModel) -> Result<Self> {
        let mut available = HashMap::new();
        for (name, value) in &pipeline.input_tags {
            available.insert(name.clone(), value.clone());
        }
        Ok(Self {
            system: system.to_string(),
            retries,
            actions: pipeline.actions.clone(),
            available,
            operators: OperatorRegistry::new(),
        })
    }

    /// Build a topologically ordered list of actions by data deps.
    fn ordered(actions: &[ActionModel]) -> Result<Vec<&ActionModel>> {
        let mut tag_to_idx: HashMap<&str, usize> = HashMap::new();
        for (i, action) in actions.iter().enumerate() {
            tag_to_idx.insert(&action.tag, i);
        }

        let mut in_degree = vec![0usize; actions.len()];
        let mut deps: Vec<Vec<usize>> = vec![Vec::new(); actions.len()];

        for (i, action) in actions.iter().enumerate() {
            if let Some(candidates) = &action.val {
                for candidate in candidates {
                    if candidate.data.is_empty() || candidate.data == action.tag {
                        continue;
                    }
                    if let Some(&dep_idx) = tag_to_idx.get(candidate.data.as_str()) {
                        in_degree[i] += 1;
                        deps[dep_idx].push(i);
                    }
                }
            }
        }

        let mut queue: VecDeque<usize> = in_degree
            .iter()
            .enumerate()
            .filter(|(_, deg)| **deg == 0)
            .map(|(i, _)| i)
            .collect();

        let mut order = Vec::with_capacity(actions.len());

        while let Some(idx) = queue.pop_front() {
            order.push(&actions[idx]);
            for &dep_idx in &deps[idx] {
                in_degree[dep_idx] -= 1;
                if in_degree[dep_idx] == 0 {
                    queue.push_back(dep_idx);
                }
            }
        }

        if order.len() != actions.len() {
            anyhow::bail!("Circular dependency detected in actions");
        }

        Ok(order)
    }

    /// Resolve a "tag|mods" reference: look up tag in available, apply operators, return String.
    fn resolve_value(&self, tag: &str, mods: &str) -> Result<String> {
        let mut current = self
            .available
            .get(tag)
            .cloned()
            .unwrap_or_else(|| tag.to_string());

        for mod_str in mods.split('|') {
            let mod_str = mod_str.trim();
            if mod_str.is_empty() {
                continue;
            }

            let (name, arg) = match mod_str.find(':') {
                Some(idx) => (mod_str[..idx].trim(), mod_str[idx + 1..].trim()),
                None => (mod_str, ""),
            };

            if let Some(key) = OperatorKey::from_str(name) {
                current = self.operators.apply(key, &current, arg)?;
            }
        }

        Ok(current)
    }

    /// Resolve data values from available into a fresh copy of actions.
    fn resolve_data(&self) -> Result<Vec<ActionModel>> {
        let mut out = Vec::with_capacity(self.actions.len());
        for action in &self.actions {
            let mut action = action.clone();
            if let Some(candidates) = &mut action.val {
                for candidate in candidates.iter_mut() {
                    let mods = candidate.mods.clone().unwrap_or_default();
                    if self.available.contains_key(&candidate.data)
                        || !self.actions.iter().any(|a| a.tag == candidate.data)
                    {
                        candidate.resolved = Some(self.resolve_value(&candidate.data, &mods)?);
                    } else {
                        candidate.resolved = None;
                    }
                }
            }
            out.push(action);
        }
        Ok(out)
    }

    pub fn next(&mut self) -> Result<Option<ActionModel>> {
        let resolved = self.resolve_data()?;
        let order = Self::ordered(&resolved)?;

        let mut last_ready: Option<&ActionModel> = None;

        for action in &order {
            // Skip done.
            if self.available.contains_key(&action.tag) {
                continue;
            }

            // First unresolved — stop, take previous.
            let is_resolved = match &action.val {
                Some(candidates) => candidates.iter().all(|c| c.resolved.is_some()),
                None => true,
            };
            if !is_resolved {
                break;
            }
            last_ready = Some(action);
        }

        let action = match last_ready {
            Some(a) => a,
            None => {
                let all_done = self
                    .actions
                    .iter()
                    .all(|a| self.available.contains_key(&a.tag));
                if all_done {
                    return Ok(None);
                }
                anyhow::bail!("First action has unresolved dependencies");
            }
        };

        // Select first candidate per name whose `when` passes (or has none).
        if let Some(candidates) = &action.val {
            let mut winners = Vec::new();
            let mut seen: HashSet<&str> = HashSet::new();

            for candidate in candidates {
                if seen.contains(candidate.name.as_str()) {
                    continue;
                }
                if let Some(when) = &candidate.when {
                    let passed = self
                        .resolve_value(&candidate.data, when)
                        .map(|v| truthy(&v))
                        .unwrap_or(false);
                    if !passed {
                        continue;
                    }
                }
                seen.insert(candidate.name.as_str());
                winners.push(candidate);
            }

            // Every name must have a winner, else dead tag.
            let all_names: HashSet<&str> = candidates.iter().map(|c| c.name.as_str()).collect();
            if winners.len() != all_names.len() {
                self.available.insert(action.tag.clone(), String::new());
                return self.next();
            }

            let mut filtered = action.clone();
            filtered.val = Some(winners.into_iter().cloned().collect());
            return Ok(Some(filtered));
        }

        Ok(Some(action.clone()))
    }

    /// Expand {name} placeholders from resolved candidates into template items.
    /// Returns (items, merge_sep). Sanitizes ITEM_SEP → "\n" and shell-quotes
    /// values when is_cmd is true.
    fn expand_placeholders(&self, action: &ActionModel, is_cmd: bool) -> (Vec<String>, String) {
        let mut items = vec![action.action.clone()];
        let mut merge_sep = String::from("\n");

        if let Some(candidates) = &action.val {
            for candidate in candidates {
                let value = candidate.resolved.clone().unwrap_or_default();
                let values: Vec<String> = match &candidate.each {
                    Some(each) => {
                        let (split_str, merge_str) = each.resolve();
                        merge_sep = expand_escapes(&merge_str);
                        let split_sep = expand_escapes(&split_str);
                        value.split(&split_sep).map(String::from).collect()
                    }
                    None => vec![value],
                };
                let placeholder = format!("{{{}}}", candidate.name);
                let mut next = Vec::new();
                for tmpl in &items {
                    for v in &values {
                        let v = v.replace(ITEM_SEP, "\n");
                        let v = if is_cmd {
                            shell_words::quote(&v).to_string()
                        } else {
                            v
                        };
                        next.push(tmpl.replace(&placeholder, &v));
                    }
                }
                items = next;
            }
        }

        (items, merge_sep)
    }

    /// Execute an action, store its result, and return the value.
    pub async fn exec(&mut self, action: &ActionModel) -> Result<String> {
        // 1. Expand {name}, fan out via each.split → items + merge separator.
        let is_cmd = matches!(action.run, ActionRun::Cmd);
        let (items, merge_sep) = self.expand_placeholders(action, is_cmd);

        // 2. Execute by run type, per item.
        let mut results: Vec<String> = Vec::with_capacity(items.len());
        for item in &items {
            let out = match &action.run {
                ActionRun::Cmd => Shell::exec(item).await?,
                ActionRun::Value => item.clone(),
                ActionRun::Vision => {
                    let (cleaned, images) = utils::format::format_image_prompt(item);
                    Cluster::exec(
                        &self.system,
                        self.retries,
                        &action.run,
                        &cleaned,
                        if images.is_empty() {
                            None
                        } else {
                            Some(images)
                        },
                    )
                    .await?
                    .result
                }
                ActionRun::Tiny | ActionRun::Small | ActionRun::Medium | ActionRun::Large => {
                    Cluster::exec(&self.system, self.retries, &action.run, item, None)
                        .await?
                        .result
                }
            };
            if !matches!(action.run, ActionRun::Value) {
                log_action(&action.tag, &action.run, &action.action, item, &out);
            }
            results.push(out.replace(ITEM_SEP, "\n"));
        }

        let result = results.join(&merge_sep);

        // 3. Validate result against check regex.
        if let Some(check) = &action.check {
            let re = Regex::new(check)?;
            if !re.is_match(&result) {
                anyhow::bail!("Action '{}' output failed check: {}", action.tag, result);
            }
        }

        // 4. Store result and return.
        self.available.insert(action.tag.clone(), result.clone());
        Ok(result)
    }

    /// Expand {name} placeholders from resolved candidates, return joined text.
    /// Like exec() step 1, but without running anything.
    pub fn action_display(&self, action: &ActionModel) -> Result<String> {
        let is_cmd = matches!(action.run, ActionRun::Cmd);
        let (items, merge_sep) = self.expand_placeholders(action, is_cmd);
        Ok(items.join(&merge_sep))
    }
}
