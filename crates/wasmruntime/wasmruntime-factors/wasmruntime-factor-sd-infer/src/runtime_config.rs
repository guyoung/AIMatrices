use crate::SdInferEngine;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct RuntimeConfig {
    pub engine: Arc<Mutex<dyn SdInferEngine>>,
}
