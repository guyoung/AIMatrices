use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;

use tokio::sync::RwLock;
use spin_app::MetadataKey;
use wasmruntime_factor_llm_infer::engine::{LlmInferEngine, LocalLlmInferEngine};
use wasmruntime_factor_llm_infer::runtime_config::RuntimeConfig;
use wasmruntime_factor_llm_infer::LOCAL_LLM_MODELS;

const MODEL_CACHED_KEY: MetadataKey = MetadataKey::new("model_cached");

pub fn config(_working_dir: &PathBuf, app: &spin_app::App) -> anyhow::Result<RuntimeConfig> {
    let models = get_engine_models(app)?;

    let model_cached = app.get_metadata(MODEL_CACHED_KEY)
        .map(|h| h.unwrap_or_default())
        .unwrap_or_default();

    let model_cached = model_cached.as_str().to_lowercase() == "true";

    let engine = LocalLlmInferEngine::init(models, model_cached)?;

    let engine = Arc::new(RwLock::new(engine)) as Arc<RwLock<dyn LlmInferEngine>>;

    Ok(RuntimeConfig { engine })
}

fn get_engine_models(app: &spin_app::App) -> anyhow::Result<HashMap<String, String>> {
    let mut engine_models: HashMap<String, String> = HashMap::new();

    for component in app.components() {
        let models = component
            .get_metadata(LOCAL_LLM_MODELS)?
            .unwrap_or_default()
            .into_iter()
            .collect::<HashMap<String, String>>();

        for (k, v) in models {
            engine_models.insert(k, v.clone());
        }
    }

    Ok(engine_models)
}
