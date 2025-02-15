
pub use crate::wit::comp::llm_infer::{
    self, EmbeddingsResult, EmbeddingsUsage, Error, InferencingParams, InferencingResult,
    InferencingUsage,
};



impl Default for InferencingParams {
    fn default() -> Self {
        Self {
            max_tokens: 256,
            temperature: 0.8,
            top_k: 40,
            top_p: 0.9,
            penalty_repeat: 1.1,
            penalty_last_n: 64,
            penalty_freq: 0.0,
            penalty_present: 0.0,


        }
    }
}

/// Perform inferencing using the provided model and prompt
pub fn infer(model_id: &str, prompt: &str) -> Result<InferencingResult, Error> {
    llm_infer::infer(model_id, prompt, None)
}

/// Perform inferencing using the provided model, prompt, and options
pub fn infer_with_options(
    model_id: &str,
    prompt: &str,
    options: InferencingParams,
) -> Result<InferencingResult, Error> {
    llm_infer::infer(model_id, prompt, Some(options))
}


/// Generate embeddings using the provided model and collection of text
pub fn generate_embeddings(
    model_id: &str,
    text: &[String],
) -> Result<llm_infer::EmbeddingsResult, Error> {
    llm_infer::generate_embeddings(model_id, text)
}
