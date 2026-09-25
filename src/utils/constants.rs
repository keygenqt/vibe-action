//! Directory names, file names, and version strings.
//! See [`crate::utils`] module-level docs for summary.

/// Config directory name
pub const CONFIG_DIR_NAME: &str = ".vibe-action";

/// Actions directory name
pub const ACTIONS_DIR_NAME: &str = "actions";

/// Config file name
pub const CONFIG_FILE_NAME: &str = "config.yaml";

/// Config version
pub const CONFIG_VERSION: &str = "0.0.3";

/// pipeline version (bump to force-update default pipelines on disk).
pub const PIPELINE_VERSION: &str = "0.0.2";

/// Cache version (bump when pipeline schema or validation logic changes).
pub const CACHE_VERSION: &str = "0.0.3";
