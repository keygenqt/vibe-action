//! Application constants and configuration defaults.
//! Defines config file names and version.

/// Config directory name
pub const CONFIG_DIR_NAME: &str = ".vibe-action";

/// Actions directory name
pub const ACTIONS_DIR_NAME: &str = "actions";

/// Config file name
pub const CONFIG_FILE_NAME: &str = "config.yaml";

/// Config version
pub const CONFIG_VERSION: &str = "0.0.2";

/// Regex pattern for {tag|modifier} placeholders.
/// Matches: {tag}, {tag|mod}, {tag|mod:arg}, {tag|mod1|mod2}
pub const TAG_PLACEHOLDER_PATTERN: &str = r"\{(\w+)(?:\|([^}]*))?\}";
