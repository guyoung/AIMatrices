use serde::Deserialize;
use thiserror::Error;

/// A result type for operations in the ollama-rs crate.
///
/// This type is used throughout the crate to represent the result of an operation,
/// which may be successful or result in an `OllamaError`.
pub type Result<T> = std::result::Result<T, OllamaError>;

/// An error type for the ollama-rs crate.
///
/// This enum represents the various errors that can occur within the crate.
/// Each variant corresponds to a different kind of error.
#[derive(Error, Debug)]
pub enum OllamaError {
    #[error("Error calling tool")]
    ToolCallError(#[from] ToolCallError),
    #[error("Ollama JSON error")]
    JsonError(#[from] serde_json::Error),
    #[error("Internal Ollama error")]
    InternalError(InternalOllamaError),
    #[error("Error in Ollama")]
    Other(String),
}

/// Represents an internal error within the Ollama service.
///
/// This struct is used to deserialize error messages returned by the service.
#[derive(Deserialize, Debug)]
pub struct InternalOllamaError {
    #[serde(rename = "error")]
    pub message: String,
}

/// An error type for tool call operations.
///
/// This enum represents errors that can occur when calling tools within the Ollama service.
/// Each variant corresponds to a different kind of tool call error.
#[derive(Error, Debug)]
pub enum ToolCallError {
    #[error("Ollama attempted to call a tool with a name we do not recognize")]
    UnknownToolName,
    #[error(
        "Could not convert tool arguments from Ollama into what the tool expected, or vice versa"
    )]
    InvalidToolArguments(#[from] serde_json::Error),
    #[error("Tool errored internally when it was called")]
    InternalToolError(#[from] Box<dyn std::error::Error>),
}
