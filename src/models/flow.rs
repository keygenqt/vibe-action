//! Flow model — one YAML action file.
//! Defines a pipeline with trigger and preparation steps.

use anyhow::Result;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, HashSet},
    fs,
    path::PathBuf,
};

use crate::models::{action::ActionModel, arg::ActionArg};

/// One action flow: name, args, trigger, steps.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlowModel {
    /// Action name (used as CLI subcommand).
    pub name: String,
    /// Short description for help.
    pub about: String,
    /// CLI arguments.
    #[serde(default)]
    pub args: Vec<ActionArg>,
    /// Main action executed last.
    pub trigger: ActionModel,
    /// Preparation steps executed before trigger.
    #[serde(default)]
    pub actions: Vec<ActionModel>,
    /// File path this flow was loaded from (for save/reload).
    #[serde(skip)]
    pub path: PathBuf,
}

impl FlowModel {
    /// Load and validate a FlowModel from a YAML file.
    pub fn load(path: &PathBuf) -> Result<Self> {
        let content = fs::read_to_string(path)?;
        let mut flow: Self = yaml_serde::from_str(&content)
            .map_err(|e| anyhow::anyhow!("Failed to parse {}: {}", path.display(), e))?;
        flow.path = path.clone();
        flow.validate()?;
        Ok(flow)
    }

    /// Apply CLI arguments by replacing {arg} placeholders in all action strings.
    pub fn apply_args(&mut self, args: &HashMap<String, String>) {
        for action in &mut self.actions {
            for (key, value) in args {
                action.action = action.action.replace(&format!("{{{}}}", key), value);
            }
        }
        for (key, value) in args {
            self.trigger.action = self.trigger.action.replace(&format!("{{{}}}", key), value);
        }
    }

    /// Validate the flow: name, tags, references, dependencies.
    pub fn validate(&self) -> Result<()> {
        // Check name is not empty.
        if self.name.trim().is_empty() {
            anyhow::bail!("Flow has no name. Add a name for the CLI command.");
        }
        // Validate args.
        for arg in &self.args {
            arg.validate()?;
        }
        // Validate trigger.
        self.trigger.validate()?;
        // Validate each action.
        for action in &self.actions {
            action.validate()?;
        }
        // Check trigger tag is set.
        if self.trigger.tag.is_empty() {
            anyhow::bail!("Trigger must have a tag.");
        }
        // Collect all valid tags: args + trigger.tag + actions[].tag.
        let mut tags: HashSet<&str> = HashSet::new();
        for arg in &self.args {
            if !tags.insert(arg.name.as_str()) {
                anyhow::bail!("Duplicate argument: '{}'", arg.name);
            }
        }
        tags.insert(self.trigger.tag.as_str());
        for action in &self.actions {
            if action.tag.is_empty() {
                anyhow::bail!("Action must have a tag.");
            }
            if !tags.insert(action.tag.as_str()) {
                anyhow::bail!("Duplicate tag: '{}'", action.tag);
            }
        }
        // Check all {tag} references in trigger and actions exist.
        let re = Regex::new(r"\{(\w+)\}").unwrap();
        for action in &self.actions {
            Self::validate_tag_references(&action.action, &tags, &re)?;
        }
        Self::validate_tag_references(&self.trigger.action, &tags, &re)?;
        // Check for circular dependencies via {tag}.
        self.validate_no_cycles(&tags, &re)?;
        Ok(())
    }

    /// Check that all {tag} references in text point to valid tags.
    fn validate_tag_references(text: &str, valid_tags: &HashSet<&str>, re: &Regex) -> Result<()> {
        for cap in re.captures_iter(text) {
            let tag = cap.get(1).unwrap().as_str();
            if !valid_tags.contains(tag) {
                anyhow::bail!("Unknown tag '{{{}}}' referenced in action", tag);
            }
        }
        Ok(())
    }

    /// Check for circular dependencies between tags.
    fn validate_no_cycles(&self, tags: &HashSet<&str>, re: &Regex) -> Result<()> {
        let mut graph: HashMap<&str, Vec<&str>> = HashMap::new();
        for tag in tags.iter() {
            graph.insert(*tag, vec![]);
        }
        let trigger_deps = Self::extract_tag_refs(&self.trigger.action, re);
        graph.insert(
            self.trigger.tag.as_str(),
            trigger_deps
                .into_iter()
                .filter(|d| tags.contains(d))
                .collect(),
        );
        for action in &self.actions {
            let deps = Self::extract_tag_refs(&action.action, re);
            graph.insert(
                action.tag.as_str(),
                deps.into_iter().filter(|d| tags.contains(d)).collect(),
            );
        }
        let mut visited = HashSet::new();
        let mut stack = HashSet::new();
        for tag in tags {
            Self::dfs(tag, &graph, &mut visited, &mut stack)?;
        }
        Ok(())
    }

    /// Extract {tag} references from a string.
    fn extract_tag_refs<'a>(text: &'a str, re: &Regex) -> Vec<&'a str> {
        re.captures_iter(text)
            .map(|c| c.get(1).unwrap().as_str())
            .collect()
    }

    /// Depth-first search for cycle detection in tag dependencies.
    fn dfs<'a>(
        node: &'a str,
        graph: &HashMap<&'a str, Vec<&'a str>>,
        visited: &mut HashSet<&'a str>,
        stack: &mut HashSet<&'a str>,
    ) -> Result<()> {
        if stack.contains(node) {
            anyhow::bail!("Circular dependency detected involving tag: '{}'", node);
        }
        if visited.contains(node) {
            return Ok(());
        }
        visited.insert(node);
        stack.insert(node);
        if let Some(deps) = graph.get(node) {
            for dep in deps {
                Self::dfs(dep, graph, visited, stack)?;
            }
        }
        stack.remove(node);
        Ok(())
    }
}
