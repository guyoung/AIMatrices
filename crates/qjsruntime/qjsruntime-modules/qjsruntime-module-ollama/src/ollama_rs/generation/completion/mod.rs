pub mod request;

use serde::{Deserialize, Serialize};

use crate::ollama_rs::Ollama;
use crate::error::APIError;

use request::GenerationRequest;




#[cfg(feature = "stream")]
/// A stream of `GenerationResponse` objects
pub type GenerationResponseStream = std::pin::Pin<
    Box<
        dyn tokio_stream::Stream<Item = crate::ollama_rs::error::Result<GenerationResponseStreamChunk>> + Send,
    >,
>;
pub type GenerationResponseStreamChunk = Vec<GenerationResponse>;

impl Ollama {

    #[cfg(feature = "stream")]
    /// Completion generation with streaming.
    /// Returns a stream of `GenerationResponse` objects
    pub fn generate_stream(
        &self,
        request: GenerationRequest<'_>,
    ) -> crate::ollama_rs::error::Result<GenerationResponseStream> {

    }

    /// Completion generation with a single response.
    /// Returns a single `GenerationResponse` object
    pub fn generate(
        &self,
        request: GenerationRequest<'_>,
    ) -> Result<GenerationResponse, APIError> {
        let mut request = request;
        request.stream = false;

        self.http_client.post("api/generate", &request)
    }
}

/// An encoding of a conversation returned by Ollama after a completion request, this can be sent in a new request to keep a conversational memory.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationContext(pub Vec<i32>);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationResponse {
    /// The name of the model used for the completion.
    pub model: String,
    /// The creation time of the completion, in such format: `2023-08-04T08:52:19.385406455-07:00`.
    pub created_at: String,
    /// The response of the completion. This can be the entire completion or only a token if the completion is streaming.
    pub response: String,
    /// Whether the completion is done. If the completion is streaming, this will be false until the last response.
    pub done: bool,
    /// An encoding of the conversation used in this response, this can be sent in the next request to keep a conversational memory
    pub context: Option<GenerationContext>,
    /// Time spent generating the response
    pub total_duration: Option<u64>,
    /// Number of tokens in the prompt
    pub prompt_eval_count: Option<u16>,
    /// Time spent in nanoseconds evaluating the prompt
    pub prompt_eval_duration: Option<u64>,
    /// Number of tokens in the response
    pub eval_count: Option<u16>,
    /// Time spent in nanoseconds generating the response
    pub eval_duration: Option<u64>,
}
