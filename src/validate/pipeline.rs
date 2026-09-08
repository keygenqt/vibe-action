//! PipelineModel validation.
//! Checks name, tags, references, and circular dependencies.
//! Supports {tag|modifier} syntax with special chars like {tag|trim:-}.

use std::collections::HashMap;
use std::collections::HashSet;

use anyhow::Result;
use regex::Regex;

use crate::models::pipeline::PipelineModel;
use crate::system::system::SystemKey;
use crate::utils::constants;
use crate::validate::ValidateTrait;

impl ValidateTrait for PipelineModel {
    /// Validate the pipeline: name, tags, references, dependencies.
    fn validate(&self) -> Result<()> {
        // Check version matches current PIPELINE_VERSION.
        if self.version != constants::PIPELINE_VERSION {
            anyhow::bail!(
                "Pipeline '{}' has version '{}' but expected '{}'.",
                self.name,
                self.version,
                constants::PIPELINE_VERSION
            );
        }
        // Check name is not empty.
        if self.name.trim().is_empty() {
            anyhow::bail!("Pipeline has no name. Add a name for the CLI command.");
        }

        // Validate args.
        for arg in &self.args {
            arg.validate()?;
        }

        // Validate each action.
        for action in &self.actions {
            action.validate()?;
        }

        // Validate api.
        if let Some(api) = &self.api {
            api.validate()?;
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

        // Validate tag references in old-style actions (no `val`).
        // val-based steps reference val-names, validated in ActionModel::validate.
        for action in &self.actions {
            if action.val.as_ref().map_or(false, |v| !v.is_empty()) {
                continue;
            }
            validate_tag_references(&action.action, &tags)?;
        }

        // Validate match regex if present.
        if let Some(pattern) = &self.check {
            Regex::new(pattern).map_err(|e| anyhow::anyhow!("Invalid check regex: {}", e))?;
        }

        // Check for circular dependencies via {tag}.
        validate_no_cycles(self, &tags)?;

        Ok(())
    }
}

/// Check that tags are not used inline inside single-quoted strings (e.g., 'text {tag}').
fn validate_no_inline_tags_in_quotes(text: &str) -> Result<()> {
    let re = Regex::new(r#"'[^']*'"#).unwrap();
    for cap in re.captures_iter(text) {
        let quoted_str = cap.get(0).unwrap().as_str();
        for mat in crate::engine::parser::TagIterator::new(quoted_str) {
            // If the tag is the only thing inside the quotes, it's safe (e.g. '{path}')
            if quoted_str.len() == mat.full_match.len() + 2 {
                continue;
            }
            anyhow::bail!(
                "Tags cannot be used inline inside single-quoted strings: '{}' in '{}'",
                mat.full_match,
                quoted_str
            );
        }
    }
    Ok(())
}

/// Check that all {tag} references in text point to valid tags.
fn validate_tag_references(text: &str, valid_tags: &HashSet<&str>) -> Result<()> {
    validate_no_inline_tags_in_quotes(text)?;

    for mat in crate::engine::parser::TagIterator::new(text) {
        if SystemKey::from_str(&mat.base_tag).is_some() {
            continue;
        }
        if mat.base_tag == "query" {
            continue;
        }
        if !valid_tags.contains(mat.base_tag.as_str()) {
            anyhow::bail!("Unknown tag '{{{}}}' referenced in action", mat.base_tag);
        }
    }
    Ok(())
}

/// Check for circular dependencies between tags.
fn validate_no_cycles(pipeline: &PipelineModel, tags: &HashSet<&str>) -> Result<()> {
    let mut graph: HashMap<&str, Vec<&str>> = HashMap::new();
    for tag in tags.iter() {
        graph.insert(*tag, vec![]);
    }

    for action in &pipeline.actions {
        let mut deps = Vec::new();
        deps.extend(extract_tag_refs(&action.action));

        // Map short-lived extracted strings to long-lived graph reference keys
        let filtered_deps: Vec<&str> = deps
            .into_iter()
            .filter_map(|d| tags.get(d.as_str()).copied())
            .collect();

        graph.insert(action.tag.as_str(), filtered_deps);
    }

    let mut visited = HashSet::new();
    let mut stack = HashSet::new();
    for tag in tags {
        dfs(tag, &graph, &mut visited, &mut stack)?;
    }
    Ok(())
}

/// Extract clean {tag} references from a string (ignores modifiers via parser mapping).
fn extract_tag_refs(text: &str) -> Vec<String> {
    crate::engine::parser::TagIterator::new(text)
        .map(|mat| mat.base_tag)
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
