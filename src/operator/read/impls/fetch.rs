//! Fetch operator — download URLs to temp files or resolve local paths.

use crate::operator::operator::Operator;
use crate::operator::operator::OperatorKey;
use crate::operator::operator::map_items;
use crate::operator::read::read::ReadKey;
use crate::utils;
use anyhow::Result;
use anyhow::anyhow;
use std::path::Path;
use std::path::PathBuf;
use tokio::runtime::Handle;
use tokio::task::block_in_place;

pub struct FetchOperator;

impl FetchOperator {
    /// Download remote stream into a deterministic temp file and return its path.
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

    /// Process a single value: URL → download, existing file → resolve, else pass through.
    fn process_one(s: &str) -> Result<String> {
        if s.starts_with("http://") || s.starts_with("https://") {
            let path = block_in_place(|| Handle::current().block_on(Self::download_to_temp(s)))?;
            return Ok(path.to_string_lossy().to_string());
        }
        if let Ok(path) = utils::path::resolve(Path::new(s)) {
            if path.exists() && !path.is_dir() {
                return Ok(path.to_string_lossy().to_string());
            }
        }
        Ok(s.to_string())
    }
}

impl Operator for FetchOperator {
    fn key(&self) -> OperatorKey {
        ReadKey::Fetch.key()
    }

    fn apply(&self, value: &str, _arg: &str) -> Result<String> {
        map_items(value, Self::process_one)
    }
}
