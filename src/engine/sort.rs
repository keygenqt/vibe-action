//! Topological sort for action dependencies.
//! Resolves {tag} references (with optional |modifier) to determine execution order.

use std::collections::{HashMap, HashSet, VecDeque};

use anyhow::Result;
use regex::Regex;

use crate::{
    models::{action::ActionModel, flow::FlowModel},
    utils::constants,
};

pub struct TopologicalSort;

impl TopologicalSort {
    /// Sort actions by {tag} dependencies using Kahn's algorithm.
    /// Returns actions in execution order (dependencies first).
    pub fn sort(flow: &FlowModel) -> Result<Vec<ActionModel>> {
        // Supports modifiers with special chars: {tag|trim:-}, {tag|join}, {tag|upper}
        let re = Regex::new(constants::TAG_PLACEHOLDER_PATTERN).unwrap();

        // Map tag -> action for quick lookup.
        let mut tag_to_action: HashMap<&str, ActionModel> = HashMap::new();
        for action in &flow.actions {
            tag_to_action.insert(&action.tag, action.clone());
        }

        let mut in_degree: HashMap<&str, usize> = HashMap::new();
        let mut deps: HashMap<&str, Vec<&str>> = HashMap::new();

        for action in &flow.actions {
            in_degree.entry(&action.tag).or_insert(0);
            deps.entry(&action.tag).or_default();

            // Collect unique {tag} references (avoid double counting).
            let mut unique_deps: HashSet<&str> = HashSet::new();
            for cap in re.captures_iter(&action.action) {
                let dep_tag = cap.get(1).unwrap().as_str(); // tag without modifier
                if dep_tag != action.tag {
                    unique_deps.insert(dep_tag);
                }
            }

            for dep_tag in unique_deps {
                if tag_to_action.contains_key(dep_tag) {
                    *in_degree.entry(&action.tag).or_insert(0) += 1;
                    deps.entry(dep_tag).or_default().push(&action.tag);
                }
            }
        }

        // Kahn's algorithm for topological sort.
        let mut queue: VecDeque<&str> = in_degree
            .iter()
            .filter(|(_, deg)| **deg == 0)
            .map(|(tag, _)| *tag)
            .collect();

        let mut order: Vec<ActionModel> = Vec::with_capacity(flow.actions.len());

        while let Some(tag) = queue.pop_front() {
            if let Some(action) = tag_to_action.get(tag) {
                order.push(action.clone());
            }
            if let Some(dependents) = deps.get(tag) {
                for dep in dependents {
                    if let Some(deg) = in_degree.get_mut(dep) {
                        *deg -= 1;
                        if *deg == 0 {
                            queue.push_back(dep);
                        }
                    }
                }
            }
        }

        // Check for cycles.
        if order.len() != flow.actions.len() {
            anyhow::bail!(
                "Circular dependency detected in actions. Expected {} steps, resolved {}.",
                flow.actions.len(),
                order.len()
            );
        }

        Ok(order)
    }
}
