use anyhow::Context;
use serde::{Deserialize, Serialize};

use wasmtime::component::Linker;
use wasmtime_wasi::pipe::MemoryOutputPipe;

use spin_app::App;

use spin_factors::RuntimeFactors;
use spin_factor_wasi::WasiFactor;



use wasmruntime_core::{Trigger, TriggerInstanceState};

pub type TriggerApp<F> = wasmruntime_core::TriggerApp<TaskTrigger, F>;

pub type TriggerInstanceBuilder<'a, F> =
    wasmruntime_core::TriggerInstanceBuilder<'a, TaskTrigger, F>;

pub struct TaskTriggerArgs {}

pub struct TaskTrigger {}

impl<F: RuntimeFactors> Trigger<F> for TaskTrigger {
    const TYPE: &'static str = "task";

    type InstanceState = ();

    fn update_core_config(&mut self, config: &mut spin_core::Config) -> anyhow::Result<()> {
        let _ = config;
        Ok(())
    }

    fn add_to_linker(
        &mut self,
        linker: &mut Linker<TriggerInstanceState<Self, F>>,
    ) -> anyhow::Result<()> {
        let _ = linker;
        Ok(())
    }

    fn supported_host_requirements() -> Vec<&'static str> {
        vec![]
    }
}

impl TaskTrigger {
    pub fn new(_app: &App) -> anyhow::Result<Self> {
        Ok(Self {})
    }
}

/// Configuration for the Task trigger
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TaskTriggerConfig {
    pub component: String,

    pub schedule: String,

}

#[derive(Clone)]
pub struct TaskExecutor {

}

impl TaskExecutor {
    pub async fn execute<F: RuntimeFactors>(
        &self,
        mut instance_builder: TriggerInstanceBuilder<'_, F>,
        _component: &str,
    ) -> anyhow::Result<Vec<u8>> {
        let stdout = MemoryOutputPipe::new(usize::MAX);

        let wasi_builder = instance_builder
            .factor_builder::<WasiFactor>()
            .context("The wagi HTTP trigger was configured without the required wasi support")?;

        // Set up Wagi environment
        // wasi_builder.args();
        // wasi_builder.env();
        // wasi_builder.stdin_pipe();
        wasi_builder.stdout(stdout.clone());

        let (instance, mut store) = instance_builder.instantiate(()).await?;

        let command = wasmtime_wasi::bindings::Command::new(&mut store, &instance)?;

        if let Err(()) = command
            .wasi_cli_run()
            .call_run(&mut store)
            .await
            .or_else(ignore_successful_proc_exit_trap)?
        {
            tracing::error!("Wagi main function returned unsuccessful result");
        }
        tracing::info!("Wagi execution complete");

        // Drop the store so we're left with a unique reference to `stdout`:
        drop(store);

        let stdout = stdout.try_into_inner().unwrap();


        Ok(stdout.to_vec())
    }
}



fn ignore_successful_proc_exit_trap(guest_err: anyhow::Error) -> anyhow::Result<anyhow::Result<(), ()>> {
    match guest_err
        .root_cause()
        .downcast_ref::<wasmtime_wasi::I32Exit>()
    {
        Some(trap) => match trap.0 {
            0 => Ok(Ok(())),
            _ => Err(guest_err),
        },
        None => Err(guest_err),
    }
}
