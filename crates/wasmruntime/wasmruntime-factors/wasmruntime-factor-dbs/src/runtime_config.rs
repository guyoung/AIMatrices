use std::sync::Arc;

use crate::engine::SurrealDbEngine;

/// The runtime configuration for the dbs factor.
#[derive(Clone)]
pub struct RuntimeConfig {
    pub engine: Arc<SurrealDbEngine>,
}
