//! PipelineModel validation.
//! See [`crate::validate`] module-level docs for validation rules.

use std::collections::HashSet;

use anyhow::Result;

use crate::models::pipeline::PipelineModel;
use crate::query::query::QueryKey;
use crate::system::system::SystemKey;
use crate::utils::constants;
use crate::validate::ValidateTrait;

impl ValidateTrait for PipelineModel {
    /// Validate the pipeline: name, tags, val data references.
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

        // Reserve query_ and system_ prefixes — users cannot define custom tags with these.
        for action in &self.actions {
            if action.tag.starts_with("query_") || action.tag.starts_with("system_") {
                anyhow::bail!(
                    "Action tag '{}' uses a reserved prefix (query_*/system_*).",
                    action.tag
                );
            }
        }
        for arg in &self.args {
            if arg.name.starts_with("query_") || arg.name.starts_with("system_") {
                anyhow::bail!(
                    "Argument '{}' uses a reserved prefix (query_*/system_*).",
                    arg.name
                );
            }
        }

        // Validate val candidate data tags: must reference a known tag, not self.
        for action in &self.actions {
            if let Some(candidates) = &action.val {
                for c in candidates {
                    let Some(data_tag) = c.data.as_deref() else {
                        continue; // no data source — mods-only candidate (e.g. screenshot)
                    };
                    if data_tag.is_empty() {
                        continue; // caught by ActionModel::validate
                    }
                    if data_tag == action.tag {
                        anyhow::bail!(
                            "Action '{}': val '{}' references its own tag.",
                            action.tag,
                            c.name
                        );
                    }
                    if data_tag == "query" {
                        anyhow::bail!(
                            "Action '{}': val '{}' uses bare 'query' — use 'query_raw' instead.",
                            action.tag,
                            c.name
                        );
                    }
                    if QueryKey::from_str(data_tag).is_some()
                        || SystemKey::from_str(data_tag).is_some()
                    {
                        continue;
                    }
                }
            }
        }

        // Validate off guard data: must reference a known tag, not self.
        for action in &self.actions {
            let Some(off) = &action.off else {
                continue;
            };
            if off.data == action.tag {
                anyhow::bail!("Action '{}': off references its own tag.", action.tag);
            }
            if off.data == "query" {
                anyhow::bail!(
                    "Action '{}': off uses bare 'query' — use 'query_raw' instead.",
                    action.tag
                );
            }
            if QueryKey::from_str(&off.data).is_none()
                && SystemKey::from_str(&off.data).is_none()
                && !tags.contains(off.data.as_str())
            {
                anyhow::bail!(
                    "Action '{}': off references unknown tag '{}'.",
                    action.tag,
                    off.data
                );
            }
        }

        Ok(())
    }
}
