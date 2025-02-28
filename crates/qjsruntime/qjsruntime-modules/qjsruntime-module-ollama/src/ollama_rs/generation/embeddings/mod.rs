pub mod request;

use serde::Deserialize;

use crate::ollama_rs::Ollama;
use crate::error::APIError;

use self::request::GenerateEmbeddingsRequest;

impl Ollama {
    /// Generate embeddings from a model
    /// * `model_name` - Name of model to generate embeddings from
    /// * `prompt` - Prompt to generate embeddings for
    pub fn generate_embeddings(
        &self,
        request: GenerateEmbeddingsRequest,
    ) -> Result<GenerateEmbeddingsResponse, APIError> {
        self.http_client.post("api/embed", &request)
    }
}

/// An embeddings generation response from Ollama.
#[derive(Debug, Deserialize, Clone)]
pub struct GenerateEmbeddingsResponse {
    #[allow(dead_code)]
    pub embeddings: Vec<Vec<f32>>,
}
