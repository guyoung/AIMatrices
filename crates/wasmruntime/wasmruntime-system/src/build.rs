use anyhow::Context as _;

use spin_factors_executor::FactorsExecutor;

use wasmruntime_core::{stdio::StdioLoggingExecutorHooks, FactorsConfig, RuntimeFactorsBuilder};
use wasmruntime_key_value::initial_kv_setter::InitialKvSetterHook;
use wasmruntime_key_value::summary::KeyValueDefaultStoreSummaryHook;

use crate::TriggerAppArgs;
use crate::TriggerFactors;

#[derive(Default)]
pub struct FactorsBuilder {
    args: TriggerAppArgs,
}

impl RuntimeFactorsBuilder for FactorsBuilder {
    type Factors = TriggerFactors;

    fn new() -> Self {
        FactorsBuilder {
            args: TriggerAppArgs::default(),
        }
    }

    fn build(
        &mut self,
        config: &FactorsConfig,
    ) -> anyhow::Result<Self::Factors> {
        let factors = TriggerFactors::new(
            Some(config.log_dir.clone()),
            config.working_dir.clone(),
            self.args.allow_transient_write,
        )
        .context("failed to create factors")?;
        Ok(factors)
    }

    fn configure_app<U: Send + 'static>(
        &mut self,
        executor: &mut FactorsExecutor<Self::Factors, U>,
        config: &FactorsConfig,
    ) -> anyhow::Result<()> {

        executor.add_hooks(StdioLoggingExecutorHooks::new(
            config.follow_components.clone(),
            Some(config.log_dir.clone()),
        ));

        executor.add_hooks(InitialKvSetterHook::new(self.args.key_values.clone()));

        executor.add_hooks(KeyValueDefaultStoreSummaryHook);

        Ok(())
    }
}
