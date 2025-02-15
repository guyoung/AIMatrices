//! Operations on URLs

use anyhow::{anyhow, Context};

use std::path::PathBuf;

/// Parse the path from a 'file:' URL
pub fn parse_file_url(url: &str) -> anyhow::Result<PathBuf> {
    url::Url::parse(url)
        .with_context(|| format!("Invalid URL: {url:?}"))?
        .to_file_path()
        .map_err(|_| anyhow!("Invalid file URL path: {url:?}"))
}

/// Remove the credentials from a URL string
pub fn remove_credentials(url: &str) -> anyhow::Result<String> {
    let mut url = url::Url::parse(url).with_context(|| format!("Invalid URL: {url:?}"))?;
    url.set_username("")
        .map_err(|_| anyhow!("Could not remove username"))?;
    url.set_password(None)
        .map_err(|_| anyhow!("Could not remove password"))?;
    Ok(url.to_string())
}
