use std::{collections::HashMap, sync::Arc};

use anyhow::Context;

use async_trait::async_trait;

use wasmtime::component::{Component, Instance};
use wasmtime::Engine as WasmtimeEngine;

use spin_app::{App, AppComponent};
use spin_factors::{
    AsInstanceState, ConfiguredApp, Factor, HasInstanceBuilder, RuntimeFactors,
    RuntimeFactorsInstanceState,
};

/// A FactorsExecutor manages execution of a Spin app.
///
/// It is generic over the spin-executor's [`RuntimeFactors`]. Additionally, it
/// holds any other per-instance state needed by the caller.
pub struct FactorsExecutor<T: RuntimeFactors, U = ()> {
    core_engine: spin_core::Engine<InstanceState<T::InstanceState, U>>,
    factors: T,
    hooks: Vec<Box<dyn ExecutorHooks<T, U>>>,
}

impl<T: RuntimeFactors, U: Send + 'static> FactorsExecutor<T, U> {
    /// Constructs a new spin-executor.
    pub fn new(
        mut core_engine_builder: spin_core::EngineBuilder<
            InstanceState<<T as RuntimeFactors>::InstanceState, U>,
        >,
        mut factors: T,
    ) -> anyhow::Result<Self> {
        factors
            .init(core_engine_builder.linker())
            .context("failed to initialize factors")?;
        Ok(Self {
            factors,
            core_engine: core_engine_builder.build(),
            hooks: Default::default(),
        })
    }

    pub fn core_engine(&self) -> &spin_core::Engine<InstanceState<T::InstanceState, U>> {
        &self.core_engine
    }

    // Adds the given [`ExecutorHooks`] to this spin-executor.
    ///
    /// Hooks are run in the order they are added.
    pub fn add_hooks(&mut self, hooks: impl ExecutorHooks<T, U> + 'static) {
        self.hooks.push(Box::new(hooks));
    }

    /// Loads a App with this spin-executor.
    pub async fn load_app(
        self: Arc<Self>,
        app: App,
        components: Vec<String>,
        component_loader: &impl ComponentLoader,
    ) -> anyhow::Result<FactorsExecutorApp<T, U>> {
        let configured_app = self
            .factors
            .configure_app(app)
            .context("failed to configure app")?;

        for hooks in &self.hooks {
            hooks.configure_app(&configured_app).await?;
        }

        let mut component_instance_pres = HashMap::new();

        for app_component in configured_app.app().components() {
            if components.contains(&app_component.id().to_string()) {
                let component = component_loader
                    .load_component(self.core_engine.as_ref(), &app_component)
                    .await?;
                let instance_pre = self.core_engine.instantiate_pre(&component)?;

                component_instance_pres.insert(app_component.id().to_string(), instance_pre);
            }
        }

        Ok(FactorsExecutorApp {
            executor: self.clone(),
            configured_app,
            component_instance_pres,
        })
    }
}

#[async_trait]
pub trait ExecutorHooks<T, U>: Send + Sync
where
    T: RuntimeFactors,
{
    /// Configure app hooks run immediately after [`RuntimeFactors::configure_app`].
    async fn configure_app(&self, configured_app: &ConfiguredApp<T>) -> anyhow::Result<()> {
        let _ = configured_app;
        Ok(())
    }

    /// Prepare instance hooks run immediately before [`FactorsExecutorApp::prepare`] returns.
    fn prepare_instance(&self, builder: &mut FactorsInstanceBuilder<T, U>) -> anyhow::Result<()> {
        let _ = builder;
        Ok(())
    }
}

/// A ComponentLoader is responsible for loading Wasmtime [`Component`]s.
#[async_trait]
pub trait ComponentLoader {
    /// Loads a [`Component`] for the given [`AppComponent`].
    async fn load_component(
        &self,
        engine: &wasmtime::Engine,
        component: &AppComponent,
    ) -> anyhow::Result<Component>;
}

type InstancePre<T, U> =
    wasmtime::component::InstancePre<InstanceState<<T as RuntimeFactors>::InstanceState, U>>;

/// A FactorsExecutorApp represents a loaded Spin app, ready for instantiation.
///
/// It is generic over the spin-executor's [`RuntimeFactors`] and any ad-hoc additional
/// per-instance state needed by the caller.
///

pub struct FactorsExecutorApp<T: RuntimeFactors, U> {
    executor: Arc<FactorsExecutor<T, U>>,
    configured_app: ConfiguredApp<T>,
    // Maps component IDs -> InstancePres
    component_instance_pres: HashMap<String, InstancePre<T, U>>,
}

impl<T: RuntimeFactors, U: Send + 'static> FactorsExecutorApp<T, U> {
    pub fn engine(&self) -> &spin_core::Engine<InstanceState<T::InstanceState, U>> {
        &self.executor.core_engine
    }

    pub fn configured_app(&self) -> &ConfiguredApp<T> {
        &self.configured_app
    }

    pub fn app(&self) -> &App {
        self.configured_app.app()
    }

    pub fn get_component(&self, component_id: &str) -> anyhow::Result<&Component> {
        let instance_pre = self
            .component_instance_pres
            .get(component_id)
            .with_context(|| format!("no such component {component_id:?}"))?;
        Ok(instance_pre.component())
    }

    /// Returns an instance builder for the given component ID.
    pub fn prepare(&self, component_id: &str) -> anyhow::Result<FactorsInstanceBuilder<T, U>> {
        let app_component = self
            .configured_app
            .app()
            .get_component(component_id)
            .with_context(|| format!("no such component {component_id:?}"))?;

        let instance_pre = self.component_instance_pres.get(component_id).unwrap();

        let factor_builders = self
            .executor
            .factors
            .prepare(&self.configured_app, component_id)?;

        let store_builder = self.executor.core_engine.store_builder();

        let mut builder = FactorsInstanceBuilder {
            store_builder,
            factor_builders,
            instance_pre,
            app_component,
            factors: &self.executor.factors,
        };

        for hooks in &self.executor.hooks {
            hooks.prepare_instance(&mut builder)?;
        }

        Ok(builder)
    }
}

/// A FactorsInstanceBuilder manages the instantiation of a Spin component instance.
///
/// It is generic over the spin-executor's [`RuntimeFactors`] and any ad-hoc additional
/// per-instance state needed by the caller.
pub struct FactorsInstanceBuilder<'a, F: RuntimeFactors, U> {
    app_component: AppComponent<'a>,
    store_builder: spin_core::StoreBuilder,
    factor_builders: F::InstanceBuilders,
    instance_pre: &'a InstancePre<F, U>,
    factors: &'a F,
}

impl<'a, T: RuntimeFactors, U> FactorsInstanceBuilder<'a, T, U> {
    /// Returns the app component for the instance.
    pub fn app_component(&self) -> &AppComponent {
        &self.app_component
    }

    /// Returns the store builder for the instance.
    pub fn store_builder(&mut self) -> &mut spin_core::StoreBuilder {
        &mut self.store_builder
    }

    /// Returns the factor instance builders for the instance.
    pub fn factor_builders(&mut self) -> &mut T::InstanceBuilders {
        &mut self.factor_builders
    }

    /// Returns the specific instance builder for the given factor.
    pub fn factor_builder<F: Factor>(&mut self) -> Option<&mut F::InstanceBuilder> {
        self.factor_builders().for_factor::<F>()
    }

    /// Returns the underlying wasmtime engine for the instance.
    pub fn wasmtime_engine(&self) -> &WasmtimeEngine {
        self.instance_pre.engine()
    }

    /// Returns the compiled component for the instance.
    pub fn component(&self) -> &Component {
        self.instance_pre.component()
    }
}

impl<'a, T: RuntimeFactors, U: Send> FactorsInstanceBuilder<'a, T, U> {
    /// Instantiates the instance with the given spin-executor instance state
    pub async fn instantiate(
        self,
        executor_instance_state: U,
    ) -> anyhow::Result<(
        Instance,
        spin_core::Store<InstanceState<T::InstanceState, U>>,
    )> {
        let instance_state = InstanceState {
            core: Default::default(),
            factors: self.factors.build_instance_state(self.factor_builders)?,
            executor: executor_instance_state,
        };
        let mut store = self.store_builder.build(instance_state)?;
        let instance = self.instance_pre.instantiate_async(&mut store).await?;
        Ok((instance, store))
    }
}

/// InstanceState is the [`spin_core::Store`] `data` for an instance.
///
/// It is generic over the [`RuntimeFactors::InstanceState`] and any ad-hoc
/// data needed by the caller.
pub struct InstanceState<T, U> {
    core: spin_core::State,
    factors: T,
    executor: U,
}

impl<T, U> InstanceState<T, U> {
    /// Provides access to the [`spin_core::State`].
    pub fn core_state(&self) -> &spin_core::State {
        &self.core
    }

    /// Provides mutable access to the [`spin_core::State`].
    pub fn core_state_mut(&mut self) -> &mut spin_core::State {
        &mut self.core
    }

    /// Provides access to the [`RuntimeFactors::InstanceState`].
    pub fn factors_instance_state(&self) -> &T {
        &self.factors
    }

    /// Provides mutable access to the [`RuntimeFactors::InstanceState`].
    pub fn factors_instance_state_mut(&mut self) -> &mut T {
        &mut self.factors
    }

    /// Provides access to the ad-hoc spin-executor instance state.
    pub fn executor_instance_state(&self) -> &U {
        &self.executor
    }

    /// Provides mutable access to the ad-hoc spin-executor instance state.
    pub fn executor_instance_state_mut(&mut self) -> &mut U {
        &mut self.executor
    }
}

impl<T, U> spin_core::AsState for InstanceState<T, U> {
    fn as_state(&mut self) -> &mut spin_core::State {
        &mut self.core
    }
}

impl<T: RuntimeFactorsInstanceState, U> AsInstanceState<T> for InstanceState<T, U> {
    fn as_instance_state(&mut self) -> &mut T {
        &mut self.factors
    }
}
