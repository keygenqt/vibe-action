//! Engine orchestrator — resolves tags, executes steps, validates results.

use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::sync::OnceLock;

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
                    let data = match candidate.data.as_deref() {
                        Some(d) if !d.is_empty() => d,
                        _ => continue,
                    };
                    if data == action.tag {
                        continue;
                    }
                    if let Some(&dep_idx) = tag_to_idx.get(data) {
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

    fn apply_ops(&self, mut current: String, mods: &str) -> Result<String> {
        for mod_str in utils::escape::split_escaped(mods, '|') {
            let mod_str = mod_str.trim();
            if mod_str.is_empty() {
                continue;
            }

            let (name, arg) = match utils::escape::find_unescaped(mod_str, ':') {
                Some(idx) => (mod_str[..idx].trim(), mod_str[idx + 1..].trim()),
                None => (mod_str, ""),
            };
            let arg = self.interpolate_tags(arg)?;

            if let Some(key) = OperatorKey::from_str(name) {
                current = self.operators.apply(key, &current, &arg)?;
            }
        }

        Ok(current)
    }

    /// Replace `{name}` in an operator arg with the tag value when `name` is
    /// a known tag. Known tag not ready yet → error (declare it via `data:`).
    fn interpolate_tags(&self, s: &str) -> Result<String> {
        static RE: OnceLock<Regex> = OnceLock::new();
        let re = RE.get_or_init(|| Regex::new(r"\{(\w+)\}").unwrap());
        let mut out = String::with_capacity(s.len());
        let mut last = 0;
        for caps in re.captures_iter(s) {
            let whole = caps.get(0).unwrap();
            let name = caps.get(1).unwrap().as_str();
            if !self.is_tag(name) {
                continue;
            }
            let value = self.available.get(name).ok_or_else(|| {
                anyhow::anyhow!(
                    "Tag '{name}' is not resolved yet — declare it via `data:` \
                     to order the dependency."
                )
            })?;
            out.push_str(&s[last..whole.start()]);
            out.push_str(value);
            last = whole.end();
        }
        out.push_str(&s[last..]);
        Ok(out)
    }

    /// True if `name` is a declared action tag or an input tag.
    fn is_tag(&self, name: &str) -> bool {
        self.actions.iter().any(|a| a.tag == name) || self.available.contains_key(name)
    }

    /// Resolve a "tag|mods" reference: look up tag in available, apply operators, return String.
    fn resolve_value(&self, tag: &str, mods: &str) -> Result<String> {
        let current = self
            .available
            .get(tag)
            .cloned()
            .unwrap_or_else(|| tag.to_string());
        self.apply_ops(current, mods)
    }

    /// Resolve data values from available into a fresh copy of actions.
    fn resolve_data(&self) -> Result<Vec<ActionModel>> {
        let mut out = Vec::with_capacity(self.actions.len());
        for action in &self.actions {
            let mut action = action.clone();
            // Skip done actions — don't re-resolve their candidates.
            if self.available.contains_key(&action.tag) {
                out.push(action);
                continue;
            }
            if let Some(candidates) = &mut action.val {
                let mut seen_names: HashSet<&str> = HashSet::new();

                for candidate in candidates.iter_mut() {
                    // A previous candidate with same name already won — skip.
                    if seen_names.contains(candidate.name.as_str()) {
                        candidate.resolved = Some(String::new());
                        continue;
                    }

                    let mods = candidate.mods.clone().unwrap_or_default();
                    let data = candidate.data.as_deref().unwrap_or("");

                    // Check when guard before resolving — don't run operators
                    // (e.g. screenshot) for candidates that will be filtered out.
                    if let Some(when) = &candidate.when {
                        let passed = self
                            .resolve_value(data, when)
                            .map(|v| truthy(&v))
                            .unwrap_or(false);
                        if !passed {
                            candidate.resolved = Some(String::new());
                            continue;
                        }
                    }

                    seen_names.insert(candidate.name.as_str());

                    // Resolve data + mods. Defer if data is an unresolved tag.
                    let resolved = if data.is_empty() {
                        self.resolve_value("", &mods)?
                    } else if self.available.contains_key(data)
                        || !self.actions.iter().any(|a| a.tag == data)
                    {
                        self.resolve_value(data, &mods)?
                    } else {
                        candidate.resolved = None;
                        continue;
                    };

                    // Post-mods hard check: bail if `fail` predicate is false.
                    if let Some(fail) = &candidate.fail {
                        let passed = self
                            .apply_ops(resolved.clone(), fail)
                            .map(|v| truthy(&v))
                            .unwrap_or(false);
                        if !passed {
                            anyhow::bail!(
                                "Action '{}': val '{}' failed check '{}'.",
                                action.tag,
                                candidate.name,
                                fail
                            );
                        }
                    }

                    candidate.resolved = Some(resolved);
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
                    let data = candidate.data.as_deref().unwrap_or("");
                    let passed = self
                        .resolve_value(data, when)
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

    /// Expand {name} placeholders and fan out via each.split.
    /// Returns (items, merge_sep). Public entry point for the handler.
    pub fn expand(&self, action: &ActionModel) -> Result<(Vec<String>, String)> {
        let is_cmd = matches!(action.run, ActionRun::Cmd);
        Ok(self.expand_placeholders(action, is_cmd))
    }

    /// Execute a single item, apply check regex. Returns the output.
    pub async fn exec_item(&self, action: &ActionModel, item: &str) -> Result<String> {
        let out = match &action.run {
            ActionRun::Cmd => Shell::exec(item).await?,
            ActionRun::Value => item.to_string(),
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

        let out = out.replace(ITEM_SEP, "\n");

        if let Some(check) = &action.reg {
            let re = Regex::new(check)?;
            if !re.is_match(&out) {
                anyhow::bail!("Action '{}' output failed check: \n{}", action.tag, out);
            }
        }

        Ok(out)
    }

    /// Join results, store under tag, return final value.
    pub fn store_result(&mut self, tag: &str, results: Vec<String>, merge_sep: &str) -> String {
        let result = results.join(merge_sep);
        self.available.insert(tag.to_string(), result.clone());
        result
    }
}
