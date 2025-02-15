pub mod loader;
pub mod stdio;

use std::path::PathBuf;

use wasmtime::component::Linker;

use spin_factors::RuntimeFactors;
use spin_factors_executor::{FactorsExecutor, FactorsExecutorApp, FactorsInstanceBuilder};

use stdio::FollowComponents;

/// Type alias for a [`spin_factors_executor::FactorsExecutorApp`] specialized to a [`Trigger`].
pub type TriggerApp<T, F> = FactorsExecutorApp<F, <T as Trigger<F>>::InstanceState>;

/// Type alias for a [`spin_factors_executor::FactorsInstanceBuilder`] specialized to a [`Trigger`].
pub type TriggerInstanceBuilder<'a, T, F> =
    FactorsInstanceBuilder<'a, F, <T as Trigger<F>>::InstanceState>;

/// Type alias for a [`spin_core::Store`] specialized to a [`Trigger`].
pub type Store<T, F> = spin_core::Store<TriggerInstanceState<T, F>>;

/// Type alias for [`spin_factors_executor::InstanceState`] specialized to a [`Trigger`].
pub type TriggerInstanceState<T, F> = spin_factors_executor::InstanceState<
    <F as RuntimeFactors>::InstanceState,
    <T as Trigger<F>>::InstanceState,
>;


/// Configuration options that are common to all triggers.
#[derive(Debug, Default)]
pub struct FactorsConfig {
    /// The Spin working directory.
    pub working_dir: PathBuf,
    /// Path to the runtime config file.
    pub runtime_config_file: Option<PathBuf>,
    /// Path to the state directory.
    pub state_dir: PathBuf,
    /// Which components should have their logs followed.
    pub follow_components: FollowComponents,
    /// Log directory for component stdout/stderr.
    pub log_dir: PathBuf,
}

/// A wasmruntime-core for a Spin runtime.
pub trait Trigger<F: RuntimeFactors>: Sized + Send {
    /// A unique identifier for this wasmruntime-core.
    const TYPE: &'static str;

    /// The instance state for this wasmruntime-core.
    type InstanceState: Send + 'static;

    /// Update the [`spin_core::Config`] for this wasmruntime-core.
    ///
    /// !!!Warning!!! This is unsupported; many configurations are likely to
    /// cause errors or unexpected behavior, especially in future versions.

    fn update_core_config(&mut self, config: &mut spin_core::Config) -> anyhow::Result<()>;

    /// Update the [`Linker`] for this wasmruntime-core.
    fn add_to_linker(
        &mut self,
        linker: &mut Linker<TriggerInstanceState<Self, F>>,
    ) -> anyhow::Result<()>;

    /// Returns a list of host requirements supported by this wasmruntime-core specifically.
    ///
    /// See [`App::ensure_needs_only`].
    fn supported_host_requirements() -> Vec<&'static str> {
        Vec::new()
    }
}

/// A builder for runtime factors.
pub trait RuntimeFactorsBuilder {
    /// The factors type to build.
    type Factors: RuntimeFactors;

    fn new() -> Self;

    /// Build the factors and runtime config from the given options.
    fn build(
        &mut self,
        config: &FactorsConfig,
    ) -> anyhow::Result<Self::Factors>;

    /// Configure the factors in the spin-executor.
    fn configure_app<U: Send + 'static>(
        &mut self,
        executor: &mut FactorsExecutor<Self::Factors, U>,
        config: &FactorsConfig,
    ) -> anyhow::Result<()> {
        let _ = (executor, config);
        Ok(())
    }
}
