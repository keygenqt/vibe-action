//! Root application config and global singleton.
//! See [`crate::configs`] module-level docs for the initialization order and cluster routing.

use anyhow::Result;
use serde::Deserialize;
use serde::Serialize;
use std::fs;
use std::sync::OnceLock;
use vibe_cluster::Cluster;
use vibe_cluster::ConnectionParams;
use vibe_cluster::Provider;

use crate::configs::action::ActionConfig;
use crate::configs::cluster::ClusterConfig;
use crate::configs::cluster::ClusterRole;
use crate::configs::group::GroupConfig;
use crate::models::action::ActionRun;
use crate::models::pipeline::PipelineModel;
use crate::models::pipelines::PipelinesModel;
use crate::output::output::OutputRegistry;
use crate::utils;
use crate::utils::constants;
use crate::utils::path;
use crate::utils::yaml::YamlComment;
use crate::validate::ValidateTrait;

/// Global config instance, loaded once at startup.
static GLOBAL_CONFIG: OnceLock<AppConfig> = OnceLock::new();

/// Global output registry.
static OUTPUT: OnceLock<OutputRegistry> = OnceLock::new();

/// Main application configuration structure.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    /// Configuration version.
    pub version: String,
    /// Action runtime configuration.
    pub action: ActionConfig,
    /// External action groups (nested CLI commands: vibe <group> <action>).
    #[serde(default)]
    pub groups: Vec<GroupConfig>,
    /// LLM cluster nodes (local and cloud models).
    pub cluster: Vec<ClusterConfig>,
    /// Loaded actions model (not serialized).
    #[serde(skip)]
    pub pipelines: Option<PipelinesModel>,
}

impl Default for AppConfig {
    /// Creates default configuration with safe defaults.
    fn default() -> Self {
        Self {
            version: constants::CONFIG_VERSION.to_string(),
            action: ActionConfig::default(),
            groups: vec![
                GroupConfig {
                    name: "code".to_string(),
                    about: "Group for working with code. Code modification in the editor via pipelines.".to_string(),
                    path: Some("/code".to_string()),
                    ..GroupConfig::default()
                },
                GroupConfig {
                    name: "data".to_string(),
                    about: "Fetch and extract external data - from the user or the internet.".to_string(),
                    path: Some("/data".to_string()),
                    ..GroupConfig::default()
                },
                GroupConfig {
                    name: "gen".to_string(),
                    about: "Generate new data without direct code modification.".to_string(),
                    path: Some("/gen".to_string()),
                    ..GroupConfig::default()
                },
                GroupConfig {
                    name: "project".to_string(),
                    about: "Tasks working with the whole project, not individual code sections.".to_string(),
                    path: Some("/project".to_string()),
                    ..GroupConfig::default()
                },
                GroupConfig {
                    name: "text".to_string(),
                    about: "Natural language text. Tasks unrelated to code.".to_string(),
                    path: Some("/text".to_string()),
                    ..GroupConfig::default()
                },
                GroupConfig {
                    name: "vision".to_string(),
                    about: "Group with vision models for tasks requiring images.".to_string(),
                    path: Some("/vision".to_string()),
                    ..GroupConfig::default()
                },
            ],
            cluster: vec![
                ClusterConfig::default(),
                ClusterConfig {
                    model: "qwen2.5-coder:7b-instruct".to_string(),
                    role: Some(ClusterRole::Medium),
                    temperature: 0.2,
                    timeout_secs: 60,
                    ..ClusterConfig::default()
                },
                ClusterConfig {
                    model: "qwen2.5-coder:14b-instruct".to_string(),
                    role: Some(ClusterRole::Large),
                    temperature: 0.3,
                    timeout_secs: 120,
                    ..ClusterConfig::default()
                },
                ClusterConfig {
                    model: "qwen2.5vl:7b".to_string(),
                    role: Some(ClusterRole::Vision),
                    temperature: 0.4,
                    num_ctx: 8192,
                    num_predict: 8192,
                    timeout_secs: 120,
                    ..ClusterConfig::default()
                },
            ],
            pipelines: None,
        }
    }
}

impl AppConfig {
    /// Get the global output registry.
    pub fn output() -> &'static OutputRegistry {
        OUTPUT
            .get()
            .expect("Output not initialized. Call AppConfig::init() first.")
    }

    /// Get the global config instance.
    pub fn instance() -> Result<&'static Self> {
        GLOBAL_CONFIG
            .get()
            .ok_or_else(|| anyhow::anyhow!("Config not initialized. Call AppConfig::init() first."))
    }

    // Initialize configuration and output.
    pub fn init() -> Result<()> {
        // Set up output from env vars.
        let log_type = std::env::var("VIBE_LOG_TYPE").unwrap_or_else(|_| "cli".to_string());
        let trace_level = std::env::var("VIBE_TRACE_LEVEL").unwrap_or_else(|_| "info".to_string());
        let _ = OUTPUT.set(OutputRegistry::new(&log_type, &trace_level));

        // Load config from file or create default.
        let config_path = path::config_path();
        let mut config: AppConfig = if config_path.exists() && config_path.is_file() {
            yaml_serde::from_str(&fs::read_to_string(&config_path)?)
                .map_err(|e| anyhow::anyhow!("Failed to parse config: {}", e))?
        } else {
            let config = AppConfig::default();
            config.save()?;
            config
        };

        config.validate()?;

        // Load pipelines (default actions dir + configured groups).
        config.pipelines = Some(PipelinesModel::load(&config.groups)?);

        // Cache globally.
        GLOBAL_CONFIG.set(config).ok();
        Ok(())
    }

    /// Save config to file with comments.
    pub fn save(&self) -> Result<()> {
        let dir = path::config_dir();
        if !dir.exists() {
            fs::create_dir_all(&dir)?;
        }
        let commits = vec![
            YamlComment::Field("version", vec!["Configuration version (do not modify)"]),
            YamlComment::Field(
                "action",
                vec![
                    "Action runtime configuration.",
                    "",
                    "system  - Global system prompt applied to all LLM requests",
                    "retries - Number of retries for failed LLM steps (0 = no retries)",
                ],
            ),
            YamlComment::Field(
                "groups",
                vec![
                    "External action groups (nested CLI commands: vibe <group> <action>).",
                    "",
                    "git   - repository URL (cloned once; re-pull via `clean`)",
                    "path  - local dir with YAML actions, or subfolder inside",
                    "        the git clone ('/' = repo root) [optional with git]",
                    "ref   - branch, tag, or commit; git only [optional]",
                    "name  - CLI group name",
                    "about - short description for help",
                ],
            ),
            YamlComment::Field(
                "cluster",
                vec![
                    "LLM cluster nodes (local and cloud models).",
                    "",
                    "provider - Provider type: ollama, deepseek, qwen, kimi, zhipu",
                    "host - API endpoint",
                    "model - LLM model name",
                    "role - Node role: small, medium, large (optional, default: all)",
                    "timeout_secs - Request timeout in seconds",
                    "temperature - Sampling temperature (0.0 - 1.0)",
                    "seed - Random seed for reproducibility",
                    "num_ctx - Context window size",
                    "num_predict - Maximum tokens to generate",
                    "api_key - API key for cloud providers",
                    "parallel - Number of parallel connections (default 1)",
                ],
            ),
        ];
        let yaml = yaml_serde::to_string(self)?;
        let content = utils::yaml::add_comments(yaml, commits);
        fs::write(path::config_path(), content)?;
        Ok(())
    }

    /// Find a pipeline by group and name from the loaded actions model.
    pub fn find_pipeline(&self, group: Option<&str>, name: &str) -> Result<PipelineModel> {
        let pipelines = self
            .pipelines
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("No actions loaded."))?;
        let found = match group {
            Some(g) => pipelines.find_in_group(g, name),
            None => pipelines.find(name),
        };
        found.cloned().ok_or_else(|| match group {
            Some(g) => anyhow::anyhow!("Unknown action: {} {}", g, name),
            None => anyhow::anyhow!("Unknown action: {}", name),
        })
    }

    /// Check if any node has the given role.
    pub fn model_role_exist(&self, role: &ClusterRole) -> bool {
        self.cluster.iter().any(|c| c.role.as_ref() == Some(role))
    }

    /// Check if pipeline references roles that don't exist in cluster.
    pub fn check_role_mismatch(&self, pipeline: &PipelineModel) -> bool {
        for action in &pipeline.actions {
            let missing = match action.run {
                ActionRun::Tiny => !self.model_role_exist(&ClusterRole::Tiny),
                ActionRun::Small => !self.model_role_exist(&ClusterRole::Small),
                ActionRun::Medium => !self.model_role_exist(&ClusterRole::Medium),
                ActionRun::Large => !self.model_role_exist(&ClusterRole::Large),
                ActionRun::Vision => !self.model_role_exist(&ClusterRole::Vision),
                _ => false,
            };
            if missing {
                return true;
            }
        }
        false
    }

    /// Create vibe-cluster filtered by role.
    pub fn create_cluster_filtered(&self, run: &ActionRun) -> Result<Cluster> {
        let target_role = match run {
            ActionRun::Tiny => Some(ClusterRole::Tiny),
            ActionRun::Small => Some(ClusterRole::Small),
            ActionRun::Medium => Some(ClusterRole::Medium),
            ActionRun::Large => Some(ClusterRole::Large),
            ActionRun::Vision => Some(ClusterRole::Vision),
            _ => None,
        };

        let effective_role = match &target_role {
            Some(role) if !self.model_role_exist(role) => None,
            _ => target_role,
        };

        let connections: Vec<ConnectionParams> = self
            .cluster
            .iter()
            .filter(|c| match (&effective_role, &c.role) {
                (Some(target), Some(node_role)) => node_role == target,
                (None, _) => true,
                _ => false,
            })
            .map(|c| {
                let provider = match c.provider.as_str() {
                    "ollama" => Provider::Ollama,
                    "deepseek" => Provider::DeepSeek,
                    "qwen" => Provider::Qwen,
                    "kimi" => Provider::Kimi,
                    "zhipu" => Provider::Zhipu,
                    other => anyhow::bail!("Unknown provider: {}", other),
                };
                Ok(ConnectionParams {
                    provider,
                    host: c.host.clone(),
                    model: c.model.clone(),
                    temperature: Some(c.temperature),
                    seed: Some(c.seed),
                    num_ctx: Some(c.num_ctx),
                    num_predict: Some(c.num_predict),
                    timeout_secs: Some(c.timeout_secs),
                    api_key: c.api_key.clone(),
                    parallel: c.parallel,
                })
            })
            .collect::<Result<Vec<_>>>()?;

        if connections.is_empty() {
            anyhow::bail!("No cluster nodes found for role: {:?}", effective_role);
        }

        Cluster::new(connections).map_err(|e| anyhow::anyhow!(e.to_string()))
    }
}
