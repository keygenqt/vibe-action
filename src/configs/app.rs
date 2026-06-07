//! Application configuration.
//! YAML-based config for estimator and cluster settings.

use anyhow::Result;
use serde::Deserialize;
use serde::Serialize;
use std::fs;
use std::path::PathBuf;
use std::sync::OnceLock;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use vibe_cluster::Cluster;
use vibe_cluster::ConnectionParams;
use vibe_cluster::Provider;

use crate::configs::cluster::ClusterConfig;
use crate::configs::estimator::EstimatorConfig;
use crate::models::actions::ActionsModel;
use crate::models::flow::FlowModel;
use crate::utils;
use crate::utils::constants;
use crate::utils::path;
use crate::utils::yaml::YamlComment;
use crate::validate::ValidateTrait;

/// Global debug mode flag.
static DEBUG_MODE: AtomicBool = AtomicBool::new(false);

/// Global config instance, loaded once at startup.
static GLOBAL_CONFIG: OnceLock<AppConfig> = OnceLock::new();

/// Main application configuration structure.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    /// Configuration version.
    pub version: String,
    /// Complexity estimator configuration.
    pub estimator: EstimatorConfig,
    /// LLM cluster nodes (local and cloud models).
    pub cluster: Vec<ClusterConfig>,
    /// Loaded actions model (not serialized).
    #[serde(skip)]
    pub actions_model: Option<ActionsModel>,
}

impl Default for AppConfig {
    /// Creates default configuration with safe defaults.
    fn default() -> Self {
        Self {
            version: constants::CONFIG_VERSION.to_string(),
            estimator: EstimatorConfig::default(),
            cluster: vec![ClusterConfig::default()],
            actions_model: None,
        }
    }
}

impl AppConfig {
    /// Returns true if debug mode is enabled.
    pub fn is_debug() -> bool {
        DEBUG_MODE.load(Ordering::Relaxed)
    }

    /// Get the global config instance (loads if not cached).
    pub fn instance() -> Result<&'static Self> {
        Self::load_with_path(None)
    }

    /// Initialize configuration, creating default files if missing.
    pub fn init(path: Option<PathBuf>, debug: bool) -> Result<()> {
        // Set debug mode before any logs.
        DEBUG_MODE.store(debug, Ordering::Relaxed);

        // Initialize tracing if debug enabled.
        if debug {
            let filter = "vibe_action=debug";
            let _ = tracing_subscriber::fmt().with_env_filter(filter).try_init();
        }

        // Initialize config.
        let config = Self::load_with_path(path)?;
        config.validate()?;

        Ok(())
    }

    /// Load config from specified path or default, caching globally.
    pub fn load_with_path(path: Option<PathBuf>) -> Result<&'static Self> {
        // Return cached config if already loaded.
        if let Some(config) = GLOBAL_CONFIG.get() {
            return Ok(config);
        }
        // Load config from file or create default.
        let mut config: AppConfig = if let Some(path) = path {
            if path.exists() && path.is_file() {
                yaml_serde::from_str(&fs::read_to_string(&path)?)
                    .map_err(|e| anyhow::anyhow!("Failed to parse config: {}", e))?
            } else {
                anyhow::bail!("Config file not found: {}", path.display())
            }
        } else {
            let path = path::config_default_path();
            if path.exists() && path.is_file() {
                yaml_serde::from_str(&fs::read_to_string(&path)?)
                    .map_err(|e| anyhow::anyhow!("Failed to parse config: {}", e))?
            } else {
                let config = AppConfig::default();
                config.save()?;
                config
            }
        };
        // Load actions from configured sources.
        let path_actions = &path::actions_dir();
        let actions = ActionsModel::load(&path_actions)?;
        actions.save(&path_actions)?;
        config.actions_model = Some(actions);
        // Cache globally.
        GLOBAL_CONFIG.set(config).ok();
        Ok(GLOBAL_CONFIG.get().unwrap())
    }

    /// Save config to file in JSON5 format.
    pub fn save(&self) -> Result<()> {
        let dir = path::config_dir();
        if !dir.exists() {
            fs::create_dir_all(&dir)?;
        }
        let commits = vec![
            YamlComment::Field("version", vec!["Configuration version (do not modify)"]),
            YamlComment::Field(
                "actions",
                vec![
                    "Action sources — directories with .yaml action files.",
                    "",
                    "Example:",
                    "  - ~/.vibe-action/actions",
                    "  - /usr/share/vibe-actions",
                ],
            ),
            YamlComment::Field(
                "estimator",
                vec![
                    "Complexity estimator configuration.",
                    "",
                    "  CRITICAL ROLE: This model judges prompt difficulty (0.0-1.0) to route",
                    "  requests to the right model. A smarter estimator gives more accurate",
                    "  complexity scores, so use the most capable model you can afford.",
                    "  Results are cached via semantic search — the estimator runs only when",
                    "  a genuinely new prompt pattern is detected.",
                    "",
                    "provider - Provider type: ollama, deepseek, qwen",
                    "host - API endpoint",
                    "model - LLM model name",
                    "parallel - Number of parallel connections (default 1)",
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
                    "complexity_from - Lower bound of complexity range (0.0-1.0)",
                    "complexity_to - Upper bound of complexity range (0.0-1.0)",
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
        fs::write(path::config_default_path(), content)?;
        Ok(())
    }

    /// Find a flow by name from the loaded actions model.
    pub fn find_flow(&self, name: &str) -> Result<FlowModel> {
        let actions = self
            .actions_model
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("No actions loaded."))?;
        actions
            .find(name)
            .cloned()
            .ok_or_else(|| anyhow::anyhow!("Unknown action: {}", name))
    }

    /// Create estimator cluster from config.
    pub fn create_estimator(&self) -> Result<Cluster> {
        let provider = match self.estimator.provider.as_str() {
            "ollama" => Provider::Ollama,
            "deepseek" => Provider::DeepSeek,
            "qwen" => Provider::Qwen,
            other => anyhow::bail!("Unknown provider: {}", other),
        };

        let connection = ConnectionParams {
            provider,
            host: self.estimator.host.clone(),
            model: self.estimator.model.clone(),
            temperature: Some(0.0),
            seed: Some(42),
            num_ctx: Some(2048),
            num_predict: Some(10),
            timeout_secs: Some(30),
            api_key: None,
            parallel: self.estimator.parallel,
        };

        Cluster::new(vec![connection]).map_err(|e| anyhow::anyhow!(e.to_string()))
    }

    /// Create vibe-cluster from config filtered by complexity score.
    pub fn create_cluster(&self, complexity: f32) -> Result<Cluster> {
        let comp_step = (complexity * 100.0).round() as usize;
        let connections: Vec<ConnectionParams> = self
            .cluster
            .iter()
            .filter(|c| {
                let from_step = (c.complexity_from * 100.0).round() as usize;
                let to_step = (c.complexity_to * 100.0).round() as usize;
                comp_step >= from_step && comp_step <= to_step
            })
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
            anyhow::bail!("No nodes found for complexity: {:.2}", complexity);
        }
        Cluster::new(connections).map_err(|e| anyhow::anyhow!(e.to_string()))
    }
}
