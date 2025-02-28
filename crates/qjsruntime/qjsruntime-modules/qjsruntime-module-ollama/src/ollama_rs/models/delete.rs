use serde::Serialize;

use crate::error::APIError;
use crate::ollama_rs::Ollama;

impl Ollama {
    /// Delete a model and its data.
    pub fn delete_model(&self, model_name: String) -> Result<(), APIError> {
        let request = DeleteModelRequest { model_name };

        self.http_client.post("api/delete", &request)

    }
}

/// A delete model request to Ollama.
#[derive(Serialize)]
struct DeleteModelRequest {
    #[serde(rename = "name")]
    model_name: String,
}
