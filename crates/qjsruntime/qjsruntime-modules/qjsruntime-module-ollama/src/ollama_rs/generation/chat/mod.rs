pub mod request;

use serde::{Deserialize, Serialize};

use crate::ollama_rs::{history::ChatHistory, Ollama};
use crate::error::APIError;

use super::{images::Image, tools::ToolCall};
use request::ChatMessageRequest;

#[cfg(feature = "stream")]
use async_stream::stream;
#[cfg(feature = "stream")]
use std::sync::{Arc, Mutex};


#[cfg(feature = "stream")]
/// A stream of `ChatMessageResponse` objects
pub type ChatMessageResponseStream =
    std::pin::Pin<Box<dyn tokio_stream::Stream<Item = Result<ChatMessageResponse, ()>> + Send>>;

impl Ollama {
    #[cfg(feature = "stream")]
    /// Chat message generation with streaming.
    /// Returns a stream of `ChatMessageResponse` objects
    pub fn send_chat_messages_stream(
        &self,
        request: ChatMessageRequest,
    ) -> crate::ollama_rs::error::Result<ChatMessageResponseStream> {

    }

    /// Chat message generation.
    /// Returns a `ChatMessageResponse` object
    pub fn send_chat_messages(
        &self,
        request: ChatMessageRequest,
    ) -> Result<ChatMessageResponse, APIError> {
        let mut request = request;
        request.stream = false;

        self.http_client.post("api/chat", &request)
    }
}

impl Ollama {
    #[cfg(feature = "stream")]
    pub fn send_chat_messages_with_history_stream<C: ChatHistory + Send + 'static>(
        &self,
        history: Arc<Mutex<C>>,
        mut request: ChatMessageRequest,
    ) -> crate::ollama_rs::error::Result<ChatMessageResponseStream> {

    }

    /// Chat message generation
    /// Returns a `ChatMessageResponse` object
    pub fn send_chat_messages_with_history<C: ChatHistory>(
        &mut self,
        history: &mut C,
        mut request: ChatMessageRequest,
    ) -> Result<ChatMessageResponse, APIError> {
        // The request is modified to include the current chat messages
        for m in request.messages {
            history.push(m);
        }

        request.messages = history.messages().to_vec();

        let result = self.send_chat_messages(request.clone());

        if let Ok(result) = result {
            history.push(result.message.clone());

            return Ok(result);
        }

        result
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessageResponse {
    /// The name of the model used for the completion.
    pub model: String,
    /// The creation time of the completion, in such format: `2023-08-04T08:52:19.385406455-07:00`.
    pub created_at: String,
    /// The generated chat message.
    pub message: ChatMessage,
    pub done: bool,
    #[serde(flatten)]
    /// The final data of the completion. This is only present if the completion is done.
    pub final_data: Option<ChatMessageFinalResponseData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessageFinalResponseData {
    /// Time spent generating the response
    pub total_duration: u64,
    /// Number of tokens in the prompt
    pub prompt_eval_count: u16,
    /// Time spent in nanoseconds evaluating the prompt
    pub prompt_eval_duration: u64,
    /// Number of tokens the response
    pub eval_count: u16,
    /// Time in nanoseconds spent generating the response
    pub eval_duration: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: MessageRole,
    pub content: String,
    #[serde(default)]
    pub tool_calls: Vec<ToolCall>,
    pub images: Option<Vec<Image>>,
}

impl ChatMessage {
    pub fn new(role: MessageRole, content: String) -> Self {
        Self {
            role,
            content,
            tool_calls: vec![],
            images: None,
        }
    }

    pub fn user(content: String) -> Self {
        Self::new(MessageRole::User, content)
    }

    pub fn assistant(content: String) -> Self {
        Self::new(MessageRole::Assistant, content)
    }

    pub fn system(content: String) -> Self {
        Self::new(MessageRole::System, content)
    }

    pub fn tool(content: String) -> Self {
        Self::new(MessageRole::Tool, content)
    }

    pub fn with_images(mut self, images: Vec<Image>) -> Self {
        self.images = Some(images);
        self
    }

    pub fn add_image(mut self, image: Image) -> Self {
        if let Some(images) = self.images.as_mut() {
            images.push(image);
        } else {
            self.images = Some(vec![image]);
        }
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum MessageRole {
    #[serde(rename = "user")]
    User,
    #[serde(rename = "assistant")]
    Assistant,
    #[serde(rename = "system")]
    System,
    #[serde(rename = "tool")]
    Tool,
}
