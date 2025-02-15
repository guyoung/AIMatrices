mod server;

use std::path::PathBuf;
use std::sync::Arc;

use spin_app::App;
use spin_factors_executor::FactorsExecutor;

use wasmruntime_core::{ FactorsConfig, Trigger, RuntimeFactorsBuilder };
use wasmruntime_core::loader::ComponentLoader as ComponentLoaderImpl;

use wasmruntime_system::{FactorsBuilder, TriggerFactors};
use wasmruntime_task_trigger::{TaskTrigger, TaskTriggerConfig, TriggerApp};



pub type TaskTriggerApp = TriggerApp<TriggerFactors>;

pub async fn start(
    trigger_app: Arc<TaskTriggerApp>,
) -> anyhow::Result<()> {
    // println!("Task server started");

    let server = server::TaskServer::new(
        trigger_app,
    )?;

    server.serve().await?;

    Ok(())
}

pub async fn crate_task_trigger_app(
    app: &App,
    common_options: &FactorsConfig,
    disable_cache: bool,
    cache: &Option<PathBuf>,
    disable_pooling: bool,
) -> anyhow::Result<crate::TaskTriggerApp> {
    let mut task_trigger = TaskTrigger::new(&app)?;



    let mut engine_config = spin_core::Config::default();

    // Apply --cache / --disable-cache
    if !disable_cache {
        engine_config.enable_cache(cache)?;
    }

    if disable_pooling {
        engine_config.disable_pooling();
    }


    let mut core_engine_builder = {
        <TaskTrigger as Trigger<TriggerFactors>>::update_core_config(
            &mut task_trigger,
            &mut engine_config,
        )?;

        spin_core::Engine::builder(&engine_config)?
    };

    <TaskTrigger as Trigger<TriggerFactors>>::add_to_linker(
        &mut task_trigger,
        core_engine_builder.linker(),
    )?;

    let mut runtime_factors_builder = FactorsBuilder::new();

    let factors = runtime_factors_builder.build(&common_options)?;

    let mut executor = FactorsExecutor::new(core_engine_builder, factors)?;
    runtime_factors_builder.configure_app(&mut executor, &common_options)?;
    let executor = Arc::new(executor);


    let components = Vec::from_iter(
        app
            .trigger_configs::<TaskTriggerConfig>("task")?
            .into_iter()
            .map(|(_, config)| config.component),
    );

    let task_app = {
        executor
            .load_app(
                app.clone(),
                components,
                &ComponentLoaderImpl::new(),
            )
            .await?
    };

    Ok(task_app)
}
