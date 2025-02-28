use serde::{Deserialize, Serialize};

use crate::error::APIError;
use crate::ollama_rs::Ollama;
/// A stream of `PushModelStatus` objects.

#[cfg(feature = "stream")]
pub type PushModelStatusStream = std::pin::Pin<
    Box<dyn tokio_stream::Stream<Item = crate::ollama_rs::error::Result<PushModelStatus>> + Send>,
>;

impl Ollama {
    #[cfg(feature = "stream")]
    /// Upload a model to a model library. Requires registering for ollama.ai and adding a public key first.
    /// Push a model with streaming, meaning that each new status will be streamed.
    /// - `model_name` - The name of the model to push in the form of `<namespace>/<model>:<tag>`.
    /// - `allow_insecure` - Allow insecure connections to the library. Only use this if you are pushing to your library during development.
    pub fn push_model_stream(
        &self,
        model_name: String,
        allow_insecure: bool,
    ) -> crate::ollama_rs::error::Result<PushModelStatusStream> {

    }

    /// Upload a model to a model library. Requires registering for ollama.ai and adding a public key first.
    /// Push a model with a single response, only the final status will be returned.
    /// - `model_name` - The name of the model to push in the form of `<namespace>/<model>:<tag>`.
    /// - `allow_insecure` - Allow insecure connections to the library. Only use this if you are pushing to your library during development.
    pub fn push_model(
        &self,
        model_name: String,
        allow_insecure: bool,
    ) -> Result<PushModelStatus, APIError> {
        let request = PushModelRequest {
            model_name,
            allow_insecure,
            stream: false,
        };

        self.http_client.post("api/push", &request)
    }
}

/// A push model request to Ollama.
#[derive(Debug, Clone, Serialize)]
struct PushModelRequest {
    #[serde(rename = "name")]
    model_name: String,
    #[serde(rename = "insecure")]
    allow_insecure: bool,
    stream: bool,
}

/// A push model status response from Ollama.
#[derive(Debug, Clone, Deserialize)]
pub struct PushModelStatus {
    #[serde(rename = "status")]
    pub message: String,
    pub digest: Option<String>,
    pub total: Option<u64>,
}
