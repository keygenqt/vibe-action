//! Topological sort for action dependencies.
//! Resolves {tag} references (with optional |modifier) to determine execution order.

use anyhow::Result;
use std::collections::{HashMap, HashSet, VecDeque};

use crate::models::{action::ActionModel, flow::FlowModel};

pub struct TopologicalSort;

impl TopologicalSort {
    /// Sort actions by {tag} dependencies using Kahn's algorithm.
    /// Returns actions in execution order (dependencies first).
    pub fn sort(flow: &FlowModel) -> Result<Vec<ActionModel>> {
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

            // Accumulate parsed dependencies safely as allocated strings
            let mut unique_deps: HashSet<String> = HashSet::new();

            for action_text in action.actions() {
                // Stream tokens natively through our new standalone parser module
                for mat in crate::engine::parser::TagIterator::new(action_text) {
                    if mat.base_tag != action.tag {
                        unique_deps.insert(mat.base_tag);
                    }
                }
            }

            // Bridge lifetimes back to long-lived strings inside the map
            for dep_tag in unique_deps {
                if let Some((&stable_tag, _)) = tag_to_action.get_key_value(dep_tag.as_str()) {
                    *in_degree.entry(&action.tag).or_insert(0) += 1;
                    deps.entry(stable_tag).or_default().push(&action.tag);
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
