use serde::Serialize;

use crate::error::APIError;
use crate::ollama_rs::Ollama;

impl Ollama {
    /// Copy a model. Creates a model with another name from an existing model.
    pub fn copy_model(
        &self,
        source: String,
        destination: String,
    ) -> Result<(), APIError> {
        let request = CopyModelRequest {
            source,
            destination,
        };

        self.http_client.post("api/copy", &request)

    }
}

/// A copy model request to Ollama.
#[derive(Serialize)]
struct CopyModelRequest {
    source: String,
    destination: String,
}
