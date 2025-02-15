use std::sync::Arc;
use tokio::sync::RwLock;

use crate::engine::LlmInferEngine;


/// The runtime configuration for the llm infer factor.
#[derive(Clone)]
pub struct RuntimeConfig {
    pub engine: Arc<RwLock<dyn LlmInferEngine>>
}