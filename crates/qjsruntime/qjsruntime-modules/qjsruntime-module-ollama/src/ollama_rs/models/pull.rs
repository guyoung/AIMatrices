use serde::{Deserialize, Serialize};

use crate::error::APIError;
use crate::ollama_rs::Ollama;

/// A stream of `PullModelStatus` objects.
#[cfg(feature = "stream")]
pub type PullModelStatusStream = std::pin::Pin<
    Box<dyn tokio_stream::Stream<Item = crate::ollama_rs::error::Result<PullModelStatus>> + Send>,
>;

impl Ollama {
    #[cfg(feature = "stream")]
    /// Pull a model with streaming, meaning that each new status will be streamed.
    /// - `model_name` - The name of the model to pull.
    /// - `allow_insecure` - Allow insecure connections to the library. Only use this if you are pulling from your own library during development.
    pub fn pull_model_stream(
        &self,
        model_name: String,
        allow_insecure: bool,
    ) -> crate::ollama_rs::error::Result<PullModelStatusStream> {

    }

    /// Pull a model with a single response, only the final status will be returned.
    /// - `model_name` - The name of the model to pull.
    /// - `allow_insecure` - Allow insecure connections to the library. Only use this if you are pulling from your own library during development.
    pub fn pull_model(
        &self,
        model_name: String,
        allow_insecure: bool,
    ) -> Result<PullModelStatus, APIError> {
        let request = PullModelRequest {
            model_name,
            allow_insecure,
            stream: false,
        };

        self.http_client.post("api/pull", &request)
    }
}

/// A pull model request to Ollama.
#[derive(Debug, Clone, Serialize)]
struct PullModelRequest {
    #[serde(rename = "name")]
    model_name: String,
    #[serde(rename = "insecure")]
    allow_insecure: bool,
    stream: bool,
}

/// A pull model status response from Ollama.
#[derive(Debug, Clone, Deserialize)]
pub struct PullModelStatus {
    #[serde(rename = "status")]
    pub message: String,
    pub digest: Option<String>,
    pub total: Option<u64>,
    pub completed: Option<u64>,
}
