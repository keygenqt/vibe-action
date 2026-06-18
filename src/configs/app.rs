//! Application configuration.
//! YAML-based config for cluster settings and action runtime.

use anyhow::Result;
use serde::Deserialize;
use serde::Serialize;
use std::fs;
use std::sync::OnceLock;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::AtomicU16;
use std::sync::atomic::Ordering;
use vibe_cluster::Cluster;
use vibe_cluster::ConnectionParams;
use vibe_cluster::Provider;

use crate::configs::action::ActionConfig;
use crate::configs::cluster::ClusterConfig;
use crate::models::actions::FlowsModel;
use crate::models::flow::FlowModel;
use crate::utils;
use crate::utils::constants;
use crate::utils::path;
use crate::utils::yaml::YamlComment;
use crate::validate::ValidateTrait;

/// Global debug mode flag.
static DEBUG_MODE: AtomicBool = AtomicBool::new(false);

/// Global level mode flag.
static LEVEL_MODE: AtomicU16 = AtomicU16::new(1);

/// Global config instance, loaded once at startup.
static GLOBAL_CONFIG: OnceLock<AppConfig> = OnceLock::new();

/// Main application configuration structure.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    /// Configuration version.
    pub version: String,
    /// Action runtime configuration.
    pub action: ActionConfig,
    /// LLM cluster nodes (local and cloud models).
    pub cluster: Vec<ClusterConfig>,
    /// Loaded actions model (not serialized).
    #[serde(skip)]
    pub flows: Option<FlowsModel>,
}

impl Default for AppConfig {
    /// Creates default configuration with safe defaults.
    fn default() -> Self {
        Self {
            version: constants::CONFIG_VERSION.to_string(),
            action: ActionConfig::default(),
            cluster: vec![
                ClusterConfig::default(),
                // @todo
                ClusterConfig {
                    host: "http://192.168.1.10:11434".to_string(),
                    ..ClusterConfig::default()
                },
            ],
            flows: None,
        }
    }
}

impl AppConfig {
    /// Returns true if debug mode is enabled.
    pub fn is_debug() -> bool {
        DEBUG_MODE.load(Ordering::Relaxed)
    }

    /// Returns true if the application is running in test mode (level 6).
    pub fn is_test() -> bool {
        LEVEL_MODE.load(Ordering::Relaxed) == 6
    }

    /// Get the global config instance (loads if not cached).
    pub fn instance() -> Result<&'static Self> {
        // Return cached config if already loaded.
        if let Some(config) = GLOBAL_CONFIG.get() {
            return Ok(config);
        }
        anyhow::bail!("Config not initialized. Call AppConfig::init() first.")
    }

    /// Initialize configuration, creating default files if missing.
    pub fn init(debug: String, level: String) -> Result<()> {
        // Set debug mode before any logs.
        let debug = debug == "1" || debug.to_lowercase() == "true";
        DEBUG_MODE.store(debug, Ordering::Relaxed);

        // Set debug mode before any logs.
        let level = level.parse::<u16>().unwrap_or(0);
        LEVEL_MODE.store(level, Ordering::Relaxed);

        // Initialize tracing if debug enabled.
        if debug {
            let level_name = match level {
                1 => "error",
                2 => "warn",
                3 => "info",
                4 => "debug",
                5 => "trace",
                6 => "test",
                _ => "debug",
            };
            let filter = if level_name == "test" {
                format!("vibe_action=error")
            } else {
                format!("vibe_action={}", level_name)
            };
            let _ = tracing_subscriber::fmt().with_env_filter(filter).try_init();
        }

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

        // Load actions from the configured path.
        // Save defaults only if using the default path (not overridden by env).
        let actions_path = &path::actions_dir();
        let is_save_default = std::env::var("VIBE_ACTION_PATH").is_err();
        config.flows = Some(FlowsModel::load(&actions_path, is_save_default)?);

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
                "cluster",
                vec![
                    "LLM cluster nodes (local and cloud models).",
                    "",
                    "provider - Provider type: ollama, deepseek, qwen",
                    "host - API endpoint",
                    "model - LLM model name",
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

    /// Find a flow by name from the loaded actions model.
    pub fn find_flow(&self, name: &str) -> Result<FlowModel> {
        let flows = self
            .flows
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("No actions loaded."))?;
        flows
            .find(name)
            .cloned()
            .ok_or_else(|| anyhow::anyhow!("Unknown action: {}", name))
    }

    /// Create vibe-cluster from all configured nodes.
    pub fn create_cluster(&self) -> Result<Cluster> {
        let connections: Vec<ConnectionParams> = self
            .cluster
            .iter()
            .map(|c| {
                let provider = match c.provider.as_str() {
                    "ollama" => Provider::Ollama,
                    "deepseek" => Provider::DeepSeek,
                    "qwen" => Provider::Qwen,
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
            anyhow::bail!("No cluster nodes found in configuration.");
        }

        Cluster::new(connections).map_err(|e| anyhow::anyhow!(e.to_string()))
    }
}
