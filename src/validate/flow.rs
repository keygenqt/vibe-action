//! FlowModel validation.
//! Checks name, tags, references, and circular dependencies.

use std::collections::{HashMap, HashSet};

use anyhow::Result;
use regex::Regex;

use crate::{models::flow::FlowModel, validate::ValidateTrait};

impl ValidateTrait for FlowModel {
    /// Validate the flow: name, tags, references, dependencies.
    fn validate(&self) -> Result<()> {
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
            validate_tag_references(&action.action, &tags, &re)?;
        }
        validate_tag_references(&self.trigger.action, &tags, &re)?;
        // Check for circular dependencies via {tag}.
        validate_no_cycles(self, &tags, &re)?;
        Ok(())
    }
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
fn validate_no_cycles(flow: &FlowModel, tags: &HashSet<&str>, re: &Regex) -> Result<()> {
    let mut graph: HashMap<&str, Vec<&str>> = HashMap::new();
    for tag in tags.iter() {
        graph.insert(*tag, vec![]);
    }
    let trigger_deps = extract_tag_refs(&flow.trigger.action, re);
    graph.insert(
        flow.trigger.tag.as_str(),
        trigger_deps
            .into_iter()
            .filter(|d| tags.contains(d))
            .collect(),
    );
    for action in &flow.actions {
        let deps = extract_tag_refs(&action.action, re);
        graph.insert(
            action.tag.as_str(),
            deps.into_iter().filter(|d| tags.contains(d)).collect(),
        );
    }
    let mut visited = HashSet::new();
    let mut stack = HashSet::new();
    for tag in tags {
        dfs(tag, &graph, &mut visited, &mut stack)?;
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
            dfs(dep, graph, visited, stack)?;
        }
    }
    stack.remove(node);
    Ok(())
}
