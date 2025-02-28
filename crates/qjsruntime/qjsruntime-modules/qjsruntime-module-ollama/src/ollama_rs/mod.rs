pub mod coordinator;
pub mod error;
pub mod generation;
pub mod history;
pub mod models;

use std::collections::HashMap;

use url::Url;

use crate::http_client::HttpClient;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Ollama {
    url: Url,
    http_client: HttpClient,
    request_headers: HashMap<String, String>,
}

impl Ollama {
    pub fn new(url: &str) -> anyhow::Result<Self> {
        let url = Url::parse(url)?;

        Self::from_url(url)
    }

    pub fn from_url(url: Url) -> anyhow::Result<Self> {
        let http_client = HttpClient::builder(url.to_string()).build()?;

        Ok(Self {
            url,
            http_client,
            request_headers: HashMap::new(),
        })
    }
}

impl Default for Ollama {
    /// Returns a default Ollama instance with the host set to `http://127.0.0.1:11434`.
    fn default() -> Self {
        let url = Url::parse("http://127.0.0.1:11434").unwrap();

        let http_client = HttpClient::builder(url.to_string()).build().unwrap();

        Self {
            url,
            http_client,
            request_headers: HashMap::new(),
        }
    }
}
