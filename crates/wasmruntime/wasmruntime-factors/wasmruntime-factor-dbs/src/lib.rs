mod host;
pub mod runtime_config;
pub mod engine;


use std::sync::Arc;

use spin_factors::{
    ConfigureAppContext, Factor, PrepareContext, RuntimeFactors, SelfInstanceBuilder,
};


pub struct DbsFactor {

}

impl DbsFactor {
    pub fn new() -> Self {
        Self {

        }
    }
}

impl Factor for DbsFactor {
    type RuntimeConfig = runtime_config::RuntimeConfig;
    type AppState = AppState;
    type InstanceBuilder = InstanceState;

    fn init<T: Send + 'static>(
        &mut self,
        mut ctx: spin_factors::InitContext<T, Self>,
    ) -> anyhow::Result<()> {
        ctx.link_bindings(spin_world::comp::dbs::add_to_linker)?;
        Ok(())
    }

    fn configure_app<T: RuntimeFactors>(
        &self,
        mut ctx: ConfigureAppContext<T, Self>,
    ) -> anyhow::Result<Self::AppState> {
        let engine = ctx
            .take_runtime_config()
            .map(|c| c.engine.clone());

        if engine.is_none() {
            return Err(anyhow::Error::msg("No dbs engine specified."));
        }

        Ok(AppState {
            engine: engine.unwrap(),
        })
    }

    fn prepare<T: RuntimeFactors>(
        &self,
        ctx: PrepareContext<T, Self>,
    ) -> anyhow::Result<Self::InstanceBuilder> {
        let component_id = ctx.app_component().id();

        Ok(InstanceState {
            engine: ctx.app_state().engine.clone(),
            namespace: component_id.to_string(),
            connections: spin_resource_table::Table::new(1024),
        })
    }
}




pub struct AppState {
    engine: Arc<engine::SurrealDbEngine>,
}

pub struct InstanceState {
    engine: Arc<engine::SurrealDbEngine>,
    namespace: String,
    connections: spin_resource_table::Table<(String, String)>,
}



impl SelfInstanceBuilder for InstanceState {}


