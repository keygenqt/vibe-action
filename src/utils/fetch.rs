//! HTTP download: bytes or temp file (deduped by URL hash).
//! See [`crate::utils`] module-level docs for summary.

use anyhow::{Result, anyhow};
use std::path::PathBuf;
use tokio::runtime::Handle;
use tokio::task::block_in_place;

fn build_client() -> Result<reqwest::Client> {
    reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36")
        .timeout(std::time::Duration::from_secs(60))
        .build()
        .map_err(|e| anyhow!("Failed to build HTTP client: {}", e))
}

/// Download URL bytes (blocking on the current tokio runtime).
pub fn download_bytes(url: &str) -> Result<Vec<u8>> {
    block_in_place(|| {
        Handle::current().block_on(async {
            let response = build_client()?
                .get(url)
                .send()
                .await
                .map_err(|e| anyhow!("Failed to fetch URL '{}': {}", url, e))?;
            let bytes = response
                .bytes()
                .await
                .map_err(|e| anyhow!("Failed to read stream bytes from '{}': {}", url, e))?;
            Ok(bytes.to_vec())
        })
    })
}

/// Download URL to a deterministic temp file (deduped by URL hash), return its path.
pub fn download_to_temp(url: &str) -> Result<PathBuf> {
    block_in_place(|| {
        Handle::current().block_on(async {
            let response = build_client()?
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
        })
    })
}
