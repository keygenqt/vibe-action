//! FlowModel validation.
//! Checks name, tags, references, and circular dependencies.
//! Supports {tag|modifier} syntax with special chars like {tag|trim:-}.

use std::collections::{HashMap, HashSet};

use anyhow::Result;
use regex::Regex;

use crate::models::action::SwitchCase;
use crate::{models::flow::FlowModel, utils::constants, validate::ValidateTrait};

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
        // Validate each action.
        for action in &self.actions {
            action.validate()?;
        }
        // Collect all valid tags: args + actions[].tag.
        let mut tags: HashSet<&str> = HashSet::new();
        for arg in &self.args {
            if !tags.insert(arg.name.as_str()) {
                anyhow::bail!("Duplicate argument: '{}'", arg.name);
            }
        }
        for action in &self.actions {
            if action.tag.is_empty() {
                anyhow::bail!("Action must have a tag.");
            }
            if !tags.insert(action.tag.as_str()) {
                anyhow::bail!("Duplicate tag: '{}'", action.tag);
            }
        }
        // Supports modifiers with special chars: {tag|trim:-}, {tag|join}, {tag|upper}
        let re = Regex::new(constants::TAG_PLACEHOLDER_PATTERN).unwrap();
        for action in &self.actions {
            if let Some(ref act) = action.action {
                validate_tag_references(act, &tags, &re)?;
            }
            if let Some(ref switch) = action.switch {
                for branch in switch {
                    match branch {
                        SwitchCase::Case {
                            case,
                            action: branch_action,
                        } => {
                            validate_tag_references(case, &tags, &re)?;
                            validate_tag_references(branch_action, &tags, &re)?;
                        }
                        SwitchCase::Else {
                            action: branch_action,
                        } => {
                            validate_tag_references(branch_action, &tags, &re)?;
                        }
                    }
                }
            }
        }
        // Validate match regex if present.
        if let Some(pattern) = &self.check {
            Regex::new(pattern).map_err(|e| anyhow::anyhow!("Invalid check regex: {}", e))?;
        }
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
    for action in &flow.actions {
        let mut deps = Vec::new();
        if let Some(ref act) = action.action {
            deps.extend(extract_tag_refs(act, re));
        }
        if let Some(ref switch) = action.switch {
            for branch in switch {
                match branch {
                    SwitchCase::Case {
                        case,
                        action: branch_action,
                    } => {
                        deps.extend(extract_tag_refs(case, re));
                        deps.extend(extract_tag_refs(branch_action, re));
                    }
                    SwitchCase::Else {
                        action: branch_action,
                    } => {
                        deps.extend(extract_tag_refs(branch_action, re));
                    }
                }
            }
        }
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

/// Extract {tag} references from a string (ignores modifiers).
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
