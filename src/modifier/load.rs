//! Load modifier — handles transport logistics. Fetches URLs to temp files or resolves local paths.
use super::modifier::Modifier;
use super::modifier::ModifierKey;
use crate::modifier::modifier::ITEM_SEP;
use crate::utils;
use anyhow::Result;
use anyhow::anyhow;
use std::path::Path;
use std::path::PathBuf;
use tokio::runtime::Handle;
use tokio::task::block_in_place;

pub struct LoadModifier;

impl LoadModifier {
    /// Download remote stream into a deterministic temp file and return its absolute path.
    async fn download_to_temp(url: &str) -> Result<PathBuf> {
        let client = reqwest::Client::builder()
            .user_agent("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36")
            .timeout(std::time::Duration::from_secs(60))
            .build()
            .map_err(|e| anyhow!("Failed to build HTTP client: {}", e))?;

        let response = client
            .get(url)
            .send()
            .await
            .map_err(|e| anyhow!("Failed to fetch URL '{}': {}", url, e))?;

        let content_type = response
            .headers()
            .get(reqwest::header::CONTENT_TYPE)
            .and_then(|v| v.to_str().ok())
            .unwrap_or("");

        let ext = mime2ext::mime2ext(content_type).unwrap_or("tmp");

        // Deterministic file name from URL hash
        let url_hash = md5::compute(url.as_bytes());
        let file_name = format!("vibe-{:x}.{}", url_hash, ext);
        let temp_path = std::env::temp_dir().join(file_name);

        let bytes = response
            .bytes()
            .await
            .map_err(|e| anyhow!("Failed to read stream bytes from '{}': {}", url, e))?;

        std::fs::write(&temp_path, &bytes)?;

        Ok(temp_path)
    }
}

impl Modifier for LoadModifier {
    fn key(&self) -> ModifierKey {
        ModifierKey::Load
    }

    fn apply(&self, value: &str, _arg: &str) -> Result<String> {
        let results: Vec<String> = value
            .split(ITEM_SEP)
            .map(|s| {
                if s.starts_with("http://") || s.starts_with("https://") {
                    let path =
                        block_in_place(|| Handle::current().block_on(Self::download_to_temp(s)))?;
                    return Ok(path.to_string_lossy().to_string());
                }
                if let Ok(path) = utils::path::resolve(Path::new(s)) {
                    if path.exists() && !path.is_dir() {
                        return Ok(path.to_string_lossy().to_string());
                    }
                }
                Ok(s.to_string())
            })
            .collect::<Result<Vec<_>>>()?;
        Ok(results.join(ITEM_SEP))
    }
}
