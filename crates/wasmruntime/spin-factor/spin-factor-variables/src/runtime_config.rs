use std::sync::Arc;

use spin_expressions::Provider;

/// The runtime configuration for the variables factor.
#[derive(Default, Clone)]
pub struct RuntimeConfig {
    pub providers: Vec<Arc<dyn Provider>>,
}

impl IntoIterator for RuntimeConfig {
    type Item = Arc<dyn Provider>;
    type IntoIter = std::vec::IntoIter<Arc<dyn Provider>>;

    fn into_iter(self) -> Self::IntoIter {
        self.providers.into_iter()
    }
}
