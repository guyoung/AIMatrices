use std::any::{Any, TypeId};
use std::collections::HashSet;
use std::path::PathBuf;

use wasmtime::component::{Linker, ResourceTable};

use spin_factor_key_value::KeyValueFactor;
use spin_factor_outbound_http::OutboundHttpFactor;
use spin_factor_outbound_networking::OutboundNetworkingFactor;
use spin_factor_variables::VariablesFactor;
use spin_factor_wasi::spin::SpinFilesMounter;
use spin_factor_wasi::WasiFactor;
use spin_factors::{
    AsInstanceState, Error, Factor, FactorInstanceBuilder, InitContext, PrepareContext,
    RuntimeFactors,
};

#[cfg(feature = "dbs")]
use wasmruntime_factor_dbs::DbsFactor;
#[cfg(any(feature = "llm-infer-v1", feature = "llm-infer-v2"))]
use wasmruntime_factor_llm_infer::LlmInferFactor;
#[cfg(feature = "sd-infer")]
use wasmruntime_factor_sd_infer::SdInferFactor;

use crate::{TriggerFactorsAppState, TriggerFactorsInstanceBuilders, TriggerFactorsInstanceState};

pub struct TriggerFactors {
    pub working_dir: PathBuf,
    pub wasi: WasiFactor,
    pub variables: VariablesFactor,
    pub key_value: KeyValueFactor,
    pub outbound_networking: OutboundNetworkingFactor,
    pub outbound_http: OutboundHttpFactor,
    /*** ***/
    #[cfg(feature = "dbs")]
    pub dbs: DbsFactor,
    #[cfg(any(feature = "llm-infer-v1", feature = "llm-infer-v2"))]
    pub llm_infer: LlmInferFactor,
    #[cfg(feature = "sd-infer")]
    pub sd_infer: SdInferFactor,
    /*** ***/
}

impl TriggerFactors {
    pub fn new(
        _state_dir: Option<PathBuf>,
        working_dir: PathBuf,
        allow_transient_writes: bool,
    ) -> anyhow::Result<Self> {
        // println!("TriggerFactors new");

        Ok(Self {
            working_dir: working_dir.clone(),
            wasi: wasi_factor(working_dir, allow_transient_writes),
            variables: VariablesFactor::default(),
            key_value: KeyValueFactor::new(),
            outbound_networking: outbound_networking_factor(),
            outbound_http: OutboundHttpFactor::default(),
            /*** ***/
            #[cfg(feature = "dbs")]
            dbs: DbsFactor::new(),
            #[cfg(any(feature = "llm-infer-v1", feature = "llm-infer-v2"))]
            llm_infer: LlmInferFactor::new(),
            #[cfg(feature = "sd-infer")]
            sd_infer: SdInferFactor::new(),
            /*** ***/
        })
    }
}

impl RuntimeFactors for TriggerFactors {
    type AppState = TriggerFactorsAppState;
    type InstanceBuilders = TriggerFactorsInstanceBuilders;
    type InstanceState = TriggerFactorsInstanceState;

    fn init<T: AsInstanceState<Self::InstanceState> + Send + 'static>(
        &mut self,
        linker: &mut Linker<T>,
    ) -> spin_factors::Result<()> {
        let factor_type_ids = [
            (
                stringify!(WasiFactor),
                TypeId::of::<(
                    <WasiFactor as Factor>::InstanceBuilder,
                    <WasiFactor as Factor>::AppState,
                )>(),
            ),
            (
                stringify!(VariablesFactor),
                TypeId::of::<(
                    <VariablesFactor as Factor>::InstanceBuilder,
                    <VariablesFactor as Factor>::AppState,
                )>(),
            ),
            (
                stringify!(KeyValueFactor),
                TypeId::of::<(
                    <KeyValueFactor as Factor>::InstanceBuilder,
                    <KeyValueFactor as Factor>::AppState,
                )>(),
            ),
            (
                stringify!(OutboundNetworkingFactor),
                TypeId::of::<(
                    <OutboundNetworkingFactor as Factor>::InstanceBuilder,
                    <OutboundNetworkingFactor as Factor>::AppState,
                )>(),
            ),
            (
                stringify!(OutboundHttpFactor),
                TypeId::of::<(
                    <OutboundHttpFactor as Factor>::InstanceBuilder,
                    <OutboundHttpFactor as Factor>::AppState,
                )>(),
            ),
            /*** ***/
            #[cfg(feature = "dbs")]
            (
                stringify!(DbsFactor),
                TypeId::of::<(
                    <DbsFactor as Factor>::InstanceBuilder,
                    <DbsFactor as Factor>::AppState,
                )>(),
            ),
            #[cfg(any(feature = "llm-infer-v1", feature = "llm-infer-v2"))]
            (
                stringify!(LlmInferFactor),
                TypeId::of::<(
                    <LlmInferFactor as Factor>::InstanceBuilder,
                    <LlmInferFactor as Factor>::AppState,
                )>(),
            ),
            #[cfg(feature = "sd-infer")]
            (
                stringify!(SdInferFactor),
                TypeId::of::<(
                    <SdInferFactor as Factor>::InstanceBuilder,
                    <SdInferFactor as Factor>::AppState,
                )>(),
            ),
            /*** ***/
        ];

        let mut unique = HashSet::new();

        for (name, type_id) in factor_type_ids {
            if !unique.insert(type_id) {
                return Err(Error::DuplicateFactorTypes(name.to_owned()));
            }
        }

        Factor::init::<T>(
            &mut self.wasi,
            InitContext::<T, WasiFactor>::new(
                linker,
                |data| &mut data.as_instance_state().wasi,
                |data| {
                    let state = data.as_instance_state();
                    (&mut state.wasi, &mut state.__table)
                },
            ),
        )
        .map_err(Error::factor_init_error::<WasiFactor>)?;

        Factor::init::<T>(
            &mut self.variables,
            InitContext::<T, VariablesFactor>::new(
                linker,
                |data| &mut data.as_instance_state().variables,
                |data| {
                    let state = data.as_instance_state();
                    (&mut state.variables, &mut state.__table)
                },
            ),
        )
        .map_err(Error::factor_init_error::<VariablesFactor>)?;

        Factor::init::<T>(
            &mut self.key_value,
            InitContext::<T, KeyValueFactor>::new(
                linker,
                |data| &mut data.as_instance_state().key_value,
                |data| {
                    let state = data.as_instance_state();
                    (&mut state.key_value, &mut state.__table)
                },
            ),
        )
        .map_err(Error::factor_init_error::<KeyValueFactor>)?;

        Factor::init::<T>(
            &mut self.outbound_networking,
            InitContext::<T, OutboundNetworkingFactor>::new(
                linker,
                |data| &mut data.as_instance_state().outbound_networking,
                |data| {
                    let state = data.as_instance_state();
                    (&mut state.outbound_networking, &mut state.__table)
                },
            ),
        )
        .map_err(Error::factor_init_error::<OutboundNetworkingFactor>)?;

        Factor::init::<T>(
            &mut self.outbound_http,
            InitContext::<T, OutboundHttpFactor>::new(
                linker,
                |data| &mut data.as_instance_state().outbound_http,
                |data| {
                    let state = data.as_instance_state();
                    (&mut state.outbound_http, &mut state.__table)
                },
            ),
        )
        .map_err(Error::factor_init_error::<OutboundHttpFactor>)?;

        /*** ***/
        #[cfg(feature = "dbs")]
        Factor::init::<T>(
            &mut self.dbs,
            InitContext::<T, DbsFactor>::new(
                linker,
                |data| &mut data.as_instance_state().dbs,
                |data| {
                    let state = data.as_instance_state();
                    (&mut state.dbs, &mut state.__table)
                },
            ),
        )
        .map_err(Error::factor_init_error::<DbsFactor>)?;

        #[cfg(any(feature = "llm-infer-v1", feature = "llm-infer-v2"))]
        Factor::init::<T>(
            &mut self.llm_infer,
            InitContext::<T, LlmInferFactor>::new(
                linker,
                |data| &mut data.as_instance_state().llm_infer,
                |data| {
                    let state = data.as_instance_state();
                    (&mut state.llm_infer, &mut state.__table)
                },
            ),
        )
        .map_err(Error::factor_init_error::<LlmInferFactor>)?;

        #[cfg(feature = "sd-infer")]
        Factor::init::<T>(
            &mut self.sd_infer,
            InitContext::<T, SdInferFactor>::new(
                linker,
                |data| &mut data.as_instance_state().sd_infer,
                |data| {
                    let state = data.as_instance_state();
                    (&mut state.sd_infer, &mut state.__table)
                },
            ),
        )
        .map_err(Error::factor_init_error::<SdInferFactor>)?;
        /*** ***/

        Ok(())
    }
    fn configure_app(
        &self,
        app: spin_app::App,
    ) -> spin_factors::Result<::spin_factors::ConfiguredApp<Self>> {
        let mut app_state = TriggerFactorsAppState {
            wasi: None,
            variables: None,
            key_value: None,
            outbound_networking: None,
            outbound_http: None,
            /*** ***/
            #[cfg(feature = "dbs")]
            dbs: None,
            #[cfg(any(feature = "llm-infer-v1", feature = "llm-infer-v2"))]
            llm_infer: None,
            #[cfg(feature = "sd-infer")]
            sd_infer: None,
            /*** ***/
        };
        app_state.wasi = Some(
            Factor::configure_app(
                &self.wasi,
                spin_factors::ConfigureAppContext::<Self, WasiFactor>::new(&app, &app_state, None)?,
            )
            .map_err(Error::factor_configure_app_error::<WasiFactor>)?,
        );

        {
            let variables_runtime_config =
                crate::rumtime_config::get_context(&self.working_dir, &app)
                    .variablese_runtime_config
                    .clone();

            app_state.variables = Some(
                Factor::configure_app(
                    &self.variables,
                    spin_factors::ConfigureAppContext::<Self, VariablesFactor>::new(
                        &app,
                        &app_state,
                        variables_runtime_config,
                    )?,
                )
                .map_err(Error::factor_configure_app_error::<VariablesFactor>)?,
            );
        }

        {
            let key_value_runtime_config =
                crate::rumtime_config::get_context(&self.working_dir, &app)
                    .key_value_runtime_config
                    .clone();

            app_state.key_value = Some(
                Factor::configure_app(
                    &self.key_value,
                    spin_factors::ConfigureAppContext::<Self, KeyValueFactor>::new(
                        &app,
                        &app_state,
                        key_value_runtime_config,
                    )?,
                )
                .map_err(Error::factor_configure_app_error::<KeyValueFactor>)?,
            );
        }

        app_state.outbound_networking = Some(
            Factor::configure_app(
                &self.outbound_networking,
                spin_factors::ConfigureAppContext::<Self, OutboundNetworkingFactor>::new(
                    &app, &app_state, None,
                )?,
            )
            .map_err(Error::factor_configure_app_error::<OutboundNetworkingFactor>)?,
        );

        app_state.outbound_http = Some(
            Factor::configure_app(
                &self.outbound_http,
                spin_factors::ConfigureAppContext::<Self, OutboundHttpFactor>::new(
                    &app, &app_state, None,
                )?,
            )
            .map_err(Error::factor_configure_app_error::<OutboundHttpFactor>)?,
        );

        /*** ***/
        #[cfg(feature = "dbs")]
        {
            let dbs_runtime_config = crate::rumtime_config::get_context(&self.working_dir, &app)
                .dbs_runtime_config
                .clone();

            app_state.dbs = Some(
                Factor::configure_app(
                    &self.dbs,
                    spin_factors::ConfigureAppContext::<Self, DbsFactor>::new(
                        &app,
                        &app_state,
                        dbs_runtime_config,
                    )?,
                )
                .map_err(Error::factor_configure_app_error::<DbsFactor>)?,
            );
        }

        #[cfg(any(feature = "llm-infer-v1", feature = "llm-infer-v2"))]
        {
            let llm_infer_runtime_config =
                crate::rumtime_config::get_context(&self.working_dir, &app)
                    .llm_infer_runtime_config
                    .clone();

            app_state.llm_infer = Some(
                Factor::configure_app(
                    &self.llm_infer,
                    spin_factors::ConfigureAppContext::<Self, LlmInferFactor>::new(
                        &app,
                        &app_state,
                        llm_infer_runtime_config,
                    )?,
                )
                .map_err(Error::factor_configure_app_error::<LlmInferFactor>)?,
            );
        }

        #[cfg(feature = "sd-infer")]
        {
            let sd_infer_runtime_config =
                crate::rumtime_config::get_context(&self.working_dir, &app)
                    .sd_infer_runtime_config
                    .clone();

            app_state.sd_infer = Some(
                Factor::configure_app(
                    &self.sd_infer,
                    spin_factors::ConfigureAppContext::<Self, SdInferFactor>::new(
                        &app,
                        &app_state,
                        sd_infer_runtime_config,
                    )?,
                )
                .map_err(Error::factor_configure_app_error::<SdInferFactor>)?,
            );
        }
        /*** ***/

        Ok(spin_factors::ConfiguredApp::new(app, app_state))
    }
    fn prepare(
        &self,
        configured_app: &::spin_factors::ConfiguredApp<Self>,
        component_id: &str,
    ) -> spin_factors::Result<Self::InstanceBuilders> {
        let app_component = configured_app
            .app()
            .get_component(component_id)
            .ok_or_else(|| Error::UnknownComponent(component_id.to_string()))?;

        let mut builders = TriggerFactorsInstanceBuilders {
            wasi: None,
            variables: None,
            key_value: None,
            outbound_networking: None,
            outbound_http: None,
            /*** ***/
            #[cfg(feature = "dbs")]
            dbs: None,
            #[cfg(any(feature = "llm-infer-v1", feature = "llm-infer-v2"))]
            llm_infer: None,
            #[cfg(feature = "sd-infer")]
            sd_infer: None,
            /*** ***/
        };

        builders.wasi = Some(
            Factor::prepare::<Self>(
                &self.wasi,
                PrepareContext::new(
                    configured_app.app_state::<WasiFactor>().unwrap(),
                    &app_component,
                    &mut builders,
                ),
            )
            .map_err(Error::factor_prepare_error::<WasiFactor>)?,
        );

        builders.variables = Some(
            Factor::prepare::<Self>(
                &self.variables,
                PrepareContext::new(
                    configured_app.app_state::<VariablesFactor>().unwrap(),
                    &app_component,
                    &mut builders,
                ),
            )
            .map_err(Error::factor_prepare_error::<VariablesFactor>)?,
        );

        builders.key_value = Some(
            Factor::prepare::<Self>(
                &self.key_value,
                PrepareContext::new(
                    configured_app.app_state::<KeyValueFactor>().unwrap(),
                    &app_component,
                    &mut builders,
                ),
            )
            .map_err(Error::factor_prepare_error::<KeyValueFactor>)?,
        );

        builders.outbound_networking = Some(
            Factor::prepare::<Self>(
                &self.outbound_networking,
                PrepareContext::new(
                    configured_app
                        .app_state::<OutboundNetworkingFactor>()
                        .unwrap(),
                    &app_component,
                    &mut builders,
                ),
            )
            .map_err(Error::factor_prepare_error::<OutboundNetworkingFactor>)?,
        );

        builders.outbound_http = Some(
            Factor::prepare::<Self>(
                &self.outbound_http,
                PrepareContext::new(
                    configured_app.app_state::<OutboundHttpFactor>().unwrap(),
                    &app_component,
                    &mut builders,
                ),
            )
            .map_err(Error::factor_prepare_error::<OutboundHttpFactor>)?,
        );

        /*** ***/
        #[cfg(feature = "dbs")]
        {
            builders.dbs = Some(
                Factor::prepare::<Self>(
                    &self.dbs,
                    PrepareContext::new(
                        configured_app.app_state::<DbsFactor>().unwrap(),
                        &app_component,
                        &mut builders,
                    ),
                )
                .map_err(Error::factor_prepare_error::<DbsFactor>)?,
            );
        }

        #[cfg(any(feature = "llm-infer-v1", feature = "llm-infer-v2"))]
        {
            builders.llm_infer = Some(
                Factor::prepare::<Self>(
                    &self.llm_infer,
                    PrepareContext::new(
                        configured_app.app_state::<LlmInferFactor>().unwrap(),
                        &app_component,
                        &mut builders,
                    ),
                )
                .map_err(Error::factor_prepare_error::<LlmInferFactor>)?,
            );
        }

        #[cfg(feature = "sd-infer")]
        {
            builders.sd_infer = Some(
                Factor::prepare::<Self>(
                    &self.sd_infer,
                    PrepareContext::new(
                        configured_app.app_state::<SdInferFactor>().unwrap(),
                        &app_component,
                        &mut builders,
                    ),
                )
                .map_err(Error::factor_prepare_error::<SdInferFactor>)?,
            );
        }
        /*** ***/

        Ok(builders)
    }
    fn build_instance_state(
        &self,
        builders: Self::InstanceBuilders,
    ) -> spin_factors::Result<Self::InstanceState> {
        Ok(TriggerFactorsInstanceState {
            __table: ResourceTable::new(),

            wasi: FactorInstanceBuilder::build(builders.wasi.unwrap())
                .map_err(Error::factor_build_error::<WasiFactor>)?,

            variables: FactorInstanceBuilder::build(builders.variables.unwrap())
                .map_err(Error::factor_build_error::<VariablesFactor>)?,

            key_value: FactorInstanceBuilder::build(builders.key_value.unwrap())
                .map_err(Error::factor_build_error::<KeyValueFactor>)?,

            outbound_networking: FactorInstanceBuilder::build(
                builders.outbound_networking.unwrap(),
            )
            .map_err(Error::factor_build_error::<OutboundNetworkingFactor>)?,

            outbound_http: FactorInstanceBuilder::build(builders.outbound_http.unwrap())
                .map_err(Error::factor_build_error::<OutboundHttpFactor>)?,

            /*** ***/
            #[cfg(feature = "dbs")]
            dbs: FactorInstanceBuilder::build(builders.dbs.unwrap())
                .map_err(Error::factor_build_error::<DbsFactor>)?,

            #[cfg(any(feature = "llm-infer-v1", feature = "llm-infer-v2"))]
            llm_infer: FactorInstanceBuilder::build(builders.llm_infer.unwrap())
                .map_err(Error::factor_build_error::<LlmInferFactor>)?,

            #[cfg(feature = "sd-infer")]
            sd_infer: FactorInstanceBuilder::build(builders.sd_infer.unwrap())
                .map_err(Error::factor_build_error::<SdInferFactor>)?,
            /*** ***/
        })
    }

    fn app_state<F: Factor>(app_state: &Self::AppState) -> Option<&F::AppState> {
        if let Some(state) = &app_state.wasi {
            if let Some(state) = <dyn Any>::downcast_ref(state) {
                return Some(state);
            }
        }

        if let Some(state) = &app_state.variables {
            if let Some(state) = <dyn Any>::downcast_ref(state) {
                return Some(state);
            }
        }

        if let Some(state) = &app_state.key_value {
            if let Some(state) = <dyn Any>::downcast_ref(state) {
                return Some(state);
            }
        }

        if let Some(state) = &app_state.outbound_networking {
            if let Some(state) = <dyn Any>::downcast_ref(state) {
                return Some(state);
            }
        }
        if let Some(state) = &app_state.outbound_http {
            if let Some(state) = <dyn Any>::downcast_ref(state) {
                return Some(state);
            }
        }

        /*** ***/
        #[cfg(feature = "dbs")]
        if let Some(state) = &app_state.dbs {
            if let Some(state) = <dyn Any>::downcast_ref(state) {
                return Some(state);
            }
        }

        #[cfg(any(feature = "llm-infer-v1", feature = "llm-infer-v2"))]
        if let Some(state) = &app_state.llm_infer {
            if let Some(state) = <dyn Any>::downcast_ref(state) {
                return Some(state);
            }
        }

        #[cfg(feature = "sd-infer")]
        if let Some(state) = &app_state.sd_infer {
            if let Some(state) = <dyn Any>::downcast_ref(state) {
                return Some(state);
            }
        }
        /*** ***/

        None
    }
    fn instance_builder_mut<F: Factor>(
        builders: &mut Self::InstanceBuilders,
    ) -> Option<Option<&mut F::InstanceBuilder>> {
        let type_id = TypeId::of::<(F::InstanceBuilder, F::AppState)>();

        if type_id
            == TypeId::of::<(
                <WasiFactor as Factor>::InstanceBuilder,
                <WasiFactor as Factor>::AppState,
            )>()
        {
            return Some(
                builders
                    .wasi
                    .as_mut()
                    .map(|builder| <dyn Any>::downcast_mut(builder).unwrap()),
            );
        }

        if type_id
            == TypeId::of::<(
                <VariablesFactor as Factor>::InstanceBuilder,
                <VariablesFactor as Factor>::AppState,
            )>()
        {
            return Some(
                builders
                    .variables
                    .as_mut()
                    .map(|builder| <dyn Any>::downcast_mut(builder).unwrap()),
            );
        }

        if type_id
            == TypeId::of::<(
                <KeyValueFactor as Factor>::InstanceBuilder,
                <KeyValueFactor as Factor>::AppState,
            )>()
        {
            return Some(
                builders
                    .key_value
                    .as_mut()
                    .map(|builder| <dyn Any>::downcast_mut(builder).unwrap()),
            );
        }

        if type_id
            == TypeId::of::<(
                <OutboundNetworkingFactor as Factor>::InstanceBuilder,
                <OutboundNetworkingFactor as Factor>::AppState,
            )>()
        {
            return Some(
                builders
                    .outbound_networking
                    .as_mut()
                    .map(|builder| <dyn Any>::downcast_mut(builder).unwrap()),
            );
        }

        if type_id
            == TypeId::of::<(
                <OutboundHttpFactor as Factor>::InstanceBuilder,
                <OutboundHttpFactor as Factor>::AppState,
            )>()
        {
            return Some(
                builders
                    .outbound_http
                    .as_mut()
                    .map(|builder| <dyn Any>::downcast_mut(builder).unwrap()),
            );
        }

        /*** ***/
        #[cfg(feature = "dbs")]
        if type_id
            == TypeId::of::<(
                <DbsFactor as Factor>::InstanceBuilder,
                <DbsFactor as Factor>::AppState,
            )>()
        {
            return Some(
                builders
                    .dbs
                    .as_mut()
                    .map(|builder| <dyn Any>::downcast_mut(builder).unwrap()),
            );
        }

        #[cfg(any(feature = "llm-infer-v1", feature = "llm-infer-v2"))]
        if type_id
            == TypeId::of::<(
                <LlmInferFactor as Factor>::InstanceBuilder,
                <LlmInferFactor as Factor>::AppState,
            )>()
        {
            return Some(
                builders
                    .llm_infer
                    .as_mut()
                    .map(|builder| <dyn Any>::downcast_mut(builder).unwrap()),
            );
        }

        #[cfg(feature = "sd-infer")]
        if type_id
            == TypeId::of::<(
                <SdInferFactor as Factor>::InstanceBuilder,
                <SdInferFactor as Factor>::AppState,
            )>()
        {
            return Some(
                builders
                    .sd_infer
                    .as_mut()
                    .map(|builder| <dyn Any>::downcast_mut(builder).unwrap()),
            );
        }
        /*** ***/

        None
    }
}

fn wasi_factor(working_dir: impl Into<PathBuf>, allow_transient_writes: bool) -> WasiFactor {
    WasiFactor::new(SpinFilesMounter::new(working_dir, allow_transient_writes))
}

fn outbound_networking_factor() -> OutboundNetworkingFactor {
    fn disallowed_host_handler(scheme: &str, authority: &str) {
        let host_pattern = format!("{scheme}://{authority}");

        tracing::error!("Outbound network destination not allowed: {host_pattern}");

        if scheme.starts_with("http") && authority == "self" {
            println!("A component tried to make an HTTP request to its own app but it does not have permission.");
        } else {
            println!(
                "A component tried to make an outbound network connection to disallowed destination '{host_pattern}'."
            );
        };

        eprintln!("To allow this request, add 'allowed_outbound_hosts = [\"{host_pattern}\"]' to the manifest component section.");
    }

    let mut factor = OutboundNetworkingFactor::new();
    factor.set_disallowed_host_handler(disallowed_host_handler);
    factor
}
