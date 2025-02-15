use std::sync::Arc;
use tokio::sync::RwLock;
use crate::SdInferEngine;

pub struct RuntimeConfig {
    pub engine: Arc<RwLock<dyn SdInferEngine>>,
}