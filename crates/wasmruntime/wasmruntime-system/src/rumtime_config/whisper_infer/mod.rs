use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;

use tokio::sync::Mutex;

use wasmruntime_factor_whisper_infer::engine::{LocalWhisperInferEngine, WhisperInferEngine};
use wasmruntime_factor_whisper_infer::runtime_config::RuntimeConfig;
use wasmruntime_factor_whisper_infer::LOCAL_WHISPER_MODELS;

pub fn config(_working_dir: &PathBuf, app: &spin_app::App) -> anyhow::Result<RuntimeConfig> {
    let models = get_engine_models(app)?;

    let engine = LocalWhisperInferEngine::init(models)?;

    let engine = Arc::new(Mutex::new(engine)) as Arc<Mutex<dyn WhisperInferEngine>>;

    Ok(RuntimeConfig { engine })
}

fn get_engine_models(app: &spin_app::App) -> anyhow::Result<HashMap<String, String>> {
    let mut engine_models: HashMap<String, String> = HashMap::new();

    for component in app.components() {
        let models = component
            .get_metadata(LOCAL_WHISPER_MODELS)?
            .unwrap_or_default()
            .into_iter()
            .collect::<HashMap<String, String>>();

        for (k, v) in models {
            engine_models.insert(k, v.clone());
        }
    }

    Ok(engine_models)
}
