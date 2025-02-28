use serde::Serialize;
use crate::error::APIError;
use crate::ollama_rs::Ollama;

use super::ModelInfo;

impl Ollama {
    /// Show details about a model including modelfile, template, parameters, license, and system prompt.
    pub async fn show_model_info(&self, model_name: String) -> Result<ModelInfo, APIError> {
        let request = ModelInfoRequest {
            model_name
        };

        self.http_client.post("api/show", &request)

    }
}

/// A show model info request to Ollama.
#[derive(Serialize)]
struct ModelInfoRequest {
    #[serde(rename = "name")]
    model_name: String,
}
