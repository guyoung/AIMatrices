pub mod engine;
mod host;
pub mod runtime_config;

use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use tokio::sync::Mutex;

use spin_factors::{
    ConfigureAppContext, Factor, PrepareContext, RuntimeFactors, SelfInstanceBuilder,
};
use spin_locked_app::MetadataKey;

use engine::WhisperInferEngine;

pub const LOCAL_WHISPER_MODELS: MetadataKey<HashMap<String, String>> =
    MetadataKey::new("local_whisper_models");

pub struct WhisperInferFactor {}

impl WhisperInferFactor {
    pub fn new() -> Self {
        Self {}
    }
}

impl Factor for WhisperInferFactor {
    type RuntimeConfig = runtime_config::RuntimeConfig;
    type AppState = AppState;
    type InstanceBuilder = InstanceState;

    fn init<T: Send + 'static>(
        &mut self,
        mut ctx: spin_factors::InitContext<T, Self>,
    ) -> anyhow::Result<()> {
        ctx.link_bindings(spin_world::comp::whisper_infer::add_to_linker)?;

        Ok(())
    }

    fn configure_app<T: RuntimeFactors>(
        &self,
        mut ctx: ConfigureAppContext<T, Self>,
    ) -> anyhow::Result<Self::AppState> {
        let mut component_allowed_models: HashMap<String, Arc<HashSet<String>>> = HashMap::new();

        let components = ctx.app().components();

        for component in components {
            let component_id = component.id().to_string();

            let models = component
                .get_metadata(LOCAL_WHISPER_MODELS)?
                .unwrap_or_default()
                .into_iter()
                .collect::<HashMap<String, String>>();

            let mut models2: HashSet<String> = HashSet::new();

            for (k, _v) in models {
                models2.insert(k);
            }

            component_allowed_models.insert(component_id, Arc::new(models2));
        }

        let engine = ctx.take_runtime_config().map(|c| c.engine.clone());

        if engine.is_none() {
            return Err(anyhow::Error::msg("No whisper infer engine specified."));
        }

        Ok(AppState {
            engine: engine.unwrap(),
            component_allowed_models,
        })
    }

    fn prepare<T: RuntimeFactors>(
        &self,
        ctx: PrepareContext<T, Self>,
    ) -> anyhow::Result<Self::InstanceBuilder> {
        let allowed_models = ctx
            .app_state()
            .component_allowed_models
            .get(ctx.app_component().id())
            .cloned()
            .unwrap_or_default();
        let engine = ctx.app_state().engine.clone();

        Ok(InstanceState {
            engine,
            allowed_models,
        })
    }
}

/// The application state for the Whisper factor.
pub struct AppState {
    engine: Arc<Mutex<dyn WhisperInferEngine>>,
    component_allowed_models: HashMap<String, Arc<HashSet<String>>>,
}

/// The instance state for the Whisper factor.
pub struct InstanceState {
    engine: Arc<Mutex<dyn WhisperInferEngine>>,
    pub allowed_models: Arc<HashSet<String>>,
}

/// The runtime configuration for the Whisper factor.

impl SelfInstanceBuilder for InstanceState {}
