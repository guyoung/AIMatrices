use serde::{Deserialize, Serialize};

use crate::error::APIError;
use crate::ollama_rs::Ollama;

/// A stream of `CreateModelStatus` objects

#[cfg(feature = "stream")]
pub type CreateModelStatusStream = std::pin::Pin<
    Box<dyn tokio_stream::Stream<Item = crate::ollama_rs::error::Result<CreateModelStatus>> + Send>,
>;

impl Ollama {
    #[cfg(feature = "stream")]
    /// Create a model with streaming, meaning that each new status will be streamed.
    pub fn create_model_stream(
        &self,
        mut request: CreateModelRequest,
    ) -> crate::ollama_rs::error::Result<CreateModelStatusStream> {

    }

    /// Create a model with a single response, only the final status will be returned.
    pub fn create_model(
        &self,
        request: CreateModelRequest,
    ) -> Result<CreateModelStatus, APIError> {
        self.http_client.post("api/create", &request)
    }
}

/// A create model request to Ollama.
#[derive(Serialize)]
pub struct CreateModelRequest {
    #[serde(rename = "name")]
    model_name: String,
    path: Option<String>,
    modelfile: Option<String>,
    stream: bool,
}

impl CreateModelRequest {
    /// Create a model described in the Modelfile at `path`.
    pub fn path(model_name: String, path: String) -> Self {
        Self {
            model_name,
            path: Some(path),
            modelfile: None,
            stream: false,
        }
    }

    /// Create a model described by the Modelfile contents passed to `modelfile`.
    pub fn modelfile(model_name: String, modelfile: String) -> Self {
        Self {
            model_name,
            path: None,
            modelfile: Some(modelfile),
            stream: false,
        }
    }
}

/// A create model status response from Ollama.
#[derive(Deserialize, Debug)]
pub struct CreateModelStatus {
    #[serde(rename = "status")]
    pub message: String,
}
