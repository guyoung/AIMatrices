use serde::{Serialize, Deserialize};

use crate::error::APIError;
use crate::ollama_rs::Ollama;

use super::LocalModel;

impl Ollama {
    pub fn list_local_models(&self) -> Result<ListLocalModelsResponse, APIError> {
        let res = self.http_client.get("api/tags");

        res
    }
}

/// A response from Ollama containing a list of local models.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListLocalModelsResponse {
    models: Vec<LocalModel>,
}
