//! Load modifier — handles transport logistics. Fetches URLs to temp files or resolves local paths.
use super::modifier::{Modifier, ModifierKey};
use crate::models::context::ContextModel;
use crate::utils;
use anyhow::{Result, anyhow};
use std::io::Write;
use std::path::{Path, PathBuf};
use tokio::runtime::Handle;
use tokio::task::block_in_place;

pub struct LoadModifier;

impl LoadModifier {
    /// Download remote stream directly into a safe temp file layout and return its absolute path.
    async fn download_to_temp(url: &str) -> Result<PathBuf> {
        let client = reqwest::Client::builder()
            .user_agent("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36")
            .timeout(std::time::Duration::from_secs(60))
            .build()
            .map_err(|e| anyhow!("Failed to build HTTP client: {}", e))?;

        // Immediately dispatch GET request and fetch the stream response bytes
        let response = client
            .get(url)
            .send()
            .await
            .map_err(|e| anyhow!("Failed to fetch URL '{}': {}", url, e))?;

        // Extract content type descriptor from incoming response headers block
        let content_type = response
            .headers()
            .get(reqwest::header::CONTENT_TYPE)
            .and_then(|v| v.to_str().ok())
            .unwrap_or("");

        // Dynamically resolve proper file extension suffix mapping utilizing mime2ext lookup db
        let ext = mime2ext::mime2ext(content_type).unwrap_or("tmp");
        let suffix = format!(".{}", ext);

        let bytes = response
            .bytes()
            .await
            .map_err(|e| anyhow!("Failed to read stream bytes from '{}': {}", url, e))?;

        // Create a temporary file that survives after the NamedTempFile object is dropped
        let temp_file = tempfile::Builder::new()
            .prefix("vibe-")
            .suffix(&suffix)
            .tempfile()?;

        let (mut file, path_buf) = temp_file.keep()?;
        file.write_all(&bytes)?;
        file.flush()?;

        Ok(path_buf)
    }
}

impl Modifier for LoadModifier {
    fn key(&self) -> ModifierKey {
        ModifierKey::Load
    }

    fn apply(&self, value: &ContextModel, _arg: &str) -> Result<ContextModel> {
        match value {
            ContextModel::String(s) => {
                // URL — download to temp
                if s.starts_with("http://") || s.starts_with("https://") {
                    let path =
                        block_in_place(|| Handle::current().block_on(Self::download_to_temp(s)))?;
                    return Ok(ContextModel::String(path.to_string_lossy().to_string()));
                }
                // Path — resolve and validate
                if let Ok(path) = utils::path::resolve(Path::new(s)) {
                    if path.exists() && !path.is_dir() {
                        return Ok(ContextModel::String(path.to_string_lossy().to_string()));
                    }
                }
                // If nothing to resolve, return as-is
                Ok(ContextModel::String(s.clone()))
            }
            ContextModel::List(items) => {
                let results: Vec<String> = items
                    .iter()
                    .map(
                        |i| match self.apply(&ContextModel::String(i.clone()), "")? {
                            ContextModel::String(s) => Ok(s),
                            _ => unreachable!(),
                        },
                    )
                    .collect::<Result<Vec<_>>>()?;
                Ok(ContextModel::List(results))
            }
        }
    }
}
