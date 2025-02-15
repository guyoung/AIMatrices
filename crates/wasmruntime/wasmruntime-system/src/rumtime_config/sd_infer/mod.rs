use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;

use wasmruntime_factor_sd_infer::engine::{SdInferEngine, LocalSdInferEngine };
use wasmruntime_factor_sd_infer::LOCAL_SD_MODELS;
use wasmruntime_factor_sd_infer::runtime_config::RuntimeConfig;

pub fn config(_working_dir: &PathBuf, app: &spin_app::App) -> anyhow::Result<RuntimeConfig> {
    let models = get_engine_models(app)?;

    let engine = LocalSdInferEngine::init(models)?;

    let engine = Arc::new(RwLock::new(engine)) as Arc<RwLock<dyn SdInferEngine>>;

    Ok(RuntimeConfig {
        engine,
    })
}

fn get_engine_models(app: &spin_app::App) -> anyhow::Result<HashMap<String, String>> {
    let mut engine_models: HashMap<String, String> = HashMap::new();

    for component in app.components() {
        let models = component
            .get_metadata(LOCAL_SD_MODELS)?
            .unwrap_or_default()
            .into_iter()
            .collect::<HashMap<String, String>>();

        for (k, v) in models {
            engine_models.insert(k, v.clone());
        }
    }

    Ok(engine_models)
}