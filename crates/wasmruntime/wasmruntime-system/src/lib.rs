mod build;
mod rumtime_config;
mod trigger_factors;

use std::any::{Any, TypeId};

use wasmtime::component::ResourceTable;

use spin_factor_wasi::WasiFactor;
use spin_factors::{
    AsInstanceState, Factor, FactorInstanceState, HasInstanceBuilder, RuntimeFactorsInstanceState,
};

use spin_factor_key_value::KeyValueFactor;
use spin_factor_variables::VariablesFactor;

use spin_factor_outbound_http::OutboundHttpFactor;
use spin_factor_outbound_networking::OutboundNetworkingFactor;

#[cfg(feature = "dbs")]
use wasmruntime_factor_dbs::DbsFactor;
#[cfg(feature = "llm-infer")]
use wasmruntime_factor_llm_infer::LlmInferFactor;
#[cfg(feature = "sd-infer")]
use wasmruntime_factor_sd_infer::SdInferFactor;
#[cfg(feature = "whisper-infer")]
use wasmruntime_factor_whisper_infer::WhisperInferFactor;

pub use build::FactorsBuilder;
pub use trigger_factors::TriggerFactors;

/// Options for building a TriggerFactors.

pub struct TriggerAppArgs {
    /// Set the static assets of the components in the temporary directory as writable.
    pub allow_transient_write: bool,
    /// Set a key/value pair (key=value) in the application's
    /// default store. Any existing value will be overwritten.
    /// Can be used multiple times.
    pub key_values: Vec<(String, String)>,
}

impl Default for TriggerAppArgs {
    fn default() -> Self {
        TriggerAppArgs {
            allow_transient_write: true,
            key_values: Vec::new(),
        }
    }
}

pub struct TriggerFactorsAppState {
    pub wasi: Option<<WasiFactor as Factor>::AppState>,
    pub variables: Option<<VariablesFactor as Factor>::AppState>,
    pub key_value: Option<<KeyValueFactor as Factor>::AppState>,
    pub outbound_networking: Option<<OutboundNetworkingFactor as Factor>::AppState>,
    pub outbound_http: Option<<OutboundHttpFactor as Factor>::AppState>,
    /*** ***/
    #[cfg(feature = "dbs")]
    pub dbs: Option<<DbsFactor as Factor>::AppState>,
    #[cfg(feature = "llm-infer")]
    pub llm_infer: Option<<LlmInferFactor as Factor>::AppState>,
    #[cfg(feature = "sd-infer")]
    pub sd_infer: Option<<SdInferFactor as Factor>::AppState>,
    #[cfg(feature = "whisper-infer")]
    pub whisper_infer: Option<<WhisperInferFactor as Factor>::AppState>,
    /*** ***/
}
pub struct TriggerFactorsInstanceBuilders {
    wasi: Option<<WasiFactor as Factor>::InstanceBuilder>,
    variables: Option<<VariablesFactor as Factor>::InstanceBuilder>,
    key_value: Option<<KeyValueFactor as Factor>::InstanceBuilder>,
    outbound_networking: Option<<OutboundNetworkingFactor as Factor>::InstanceBuilder>,
    outbound_http: Option<<OutboundHttpFactor as Factor>::InstanceBuilder>,
    /*** ***/
    #[cfg(feature = "dbs")]
    dbs: Option<<DbsFactor as Factor>::InstanceBuilder>,
    #[cfg(feature = "llm-infer")]
    llm_infer: Option<<LlmInferFactor as Factor>::InstanceBuilder>,
    #[cfg(feature = "sd-infer")]
    sd_infer: Option<<SdInferFactor as Factor>::InstanceBuilder>,
    #[cfg(feature = "whisper-infer")]
    whisper_infer: Option<<WhisperInferFactor as Factor>::InstanceBuilder>,
    /*** ***/
}

#[allow(dead_code)]
impl TriggerFactorsInstanceBuilders {
    pub fn wasi(&mut self) -> &mut <WasiFactor as Factor>::InstanceBuilder {
        self.wasi.as_mut().unwrap()
    }

    pub fn variables(&mut self) -> &mut <VariablesFactor as Factor>::InstanceBuilder {
        self.variables.as_mut().unwrap()
    }

    pub fn key_value(&mut self) -> &mut <KeyValueFactor as Factor>::InstanceBuilder {
        self.key_value.as_mut().unwrap()
    }

    pub fn outbound_networking(
        &mut self,
    ) -> &mut <OutboundNetworkingFactor as Factor>::InstanceBuilder {
        self.outbound_networking.as_mut().unwrap()
    }

    pub fn outbound_http(&mut self) -> &mut <OutboundHttpFactor as Factor>::InstanceBuilder {
        self.outbound_http.as_mut().unwrap()
    }

    /*** ***/
    #[cfg(feature = "dbs")]
    pub fn dbs(&mut self) -> &mut <DbsFactor as Factor>::InstanceBuilder {
        self.dbs.as_mut().unwrap()
    }
    #[cfg(feature = "llm-infer")]
    pub fn llm_infer(&mut self) -> &mut <LlmInferFactor as Factor>::InstanceBuilder {
        self.llm_infer.as_mut().unwrap()
    }

    #[cfg(feature = "sd-infer")]
    pub fn sd_infer(&mut self) -> &mut <SdInferFactor as Factor>::InstanceBuilder {
        self.sd_infer.as_mut().unwrap()
    }
    #[cfg(feature = "whisper-infer")]
    pub fn whisper_infer(&mut self) -> &mut <WhisperInferFactor as Factor>::InstanceBuilder {
        self.whisper_infer.as_mut().unwrap()
    }
    /*** ***/
}

impl HasInstanceBuilder for TriggerFactorsInstanceBuilders {
    fn for_factor<F: Factor>(&mut self) -> Option<&mut F::InstanceBuilder> {
        let type_id = TypeId::of::<F::InstanceBuilder>();

        if type_id == TypeId::of::<<WasiFactor as Factor>::InstanceBuilder>() {
            let builder = self.wasi.as_mut().unwrap();
            return Some(<dyn Any>::downcast_mut(builder).unwrap());
        }

        if type_id == TypeId::of::<<VariablesFactor as Factor>::InstanceBuilder>() {
            let builder = self.variables.as_mut().unwrap();
            return Some(<dyn Any>::downcast_mut(builder).unwrap());
        }

        if type_id == TypeId::of::<<KeyValueFactor as Factor>::InstanceBuilder>() {
            let builder = self.key_value.as_mut().unwrap();
            return Some(<dyn Any>::downcast_mut(builder).unwrap());
        }

        if type_id == TypeId::of::<<OutboundNetworkingFactor as Factor>::InstanceBuilder>() {
            let builder = self.outbound_networking.as_mut().unwrap();
            return Some(<dyn Any>::downcast_mut(builder).unwrap());
        }

        if type_id == TypeId::of::<<OutboundHttpFactor as Factor>::InstanceBuilder>() {
            let builder = self.outbound_http.as_mut().unwrap();
            return Some(<dyn Any>::downcast_mut(builder).unwrap());
        }

        /*** ***/
        #[cfg(feature = "dbs")]
        {
            if type_id == TypeId::of::<<DbsFactor as Factor>::InstanceBuilder>() {
                let builder = self.dbs.as_mut().unwrap();
                return Some(<dyn Any>::downcast_mut(builder).unwrap());
            }
        }

        #[cfg(feature = "llm-infer")]
        {
            if type_id == TypeId::of::<<LlmInferFactor as Factor>::InstanceBuilder>() {
                let builder = self.llm_infer.as_mut().unwrap();
                return Some(<dyn Any>::downcast_mut(builder).unwrap());
            }
        }

        #[cfg(feature = "sd-infer")]
        {
            if type_id == TypeId::of::<<SdInferFactor as Factor>::InstanceBuilder>() {
                let builder = self.sd_infer.as_mut().unwrap();
                return Some(<dyn Any>::downcast_mut(builder).unwrap());
            }
        }

        #[cfg(feature = "whisper-infer")]
        {
            if type_id == TypeId::of::<<WhisperInferFactor as Factor>::InstanceBuilder>() {
                let builder = self.whisper_infer.as_mut().unwrap();
                return Some(<dyn Any>::downcast_mut(builder).unwrap());
            }
        }
        /*** ***/

        None
    }
}

pub struct TriggerFactorsInstanceState {
    __table: ResourceTable,
    pub wasi: FactorInstanceState<WasiFactor>,
    pub variables: FactorInstanceState<VariablesFactor>,
    pub key_value: FactorInstanceState<KeyValueFactor>,
    pub outbound_networking: FactorInstanceState<OutboundNetworkingFactor>,
    pub outbound_http: FactorInstanceState<OutboundHttpFactor>,
    /*** ***/
    #[cfg(feature = "dbs")]
    pub dbs: FactorInstanceState<DbsFactor>,
    #[cfg(feature = "llm-infer")]
    pub llm_infer: FactorInstanceState<LlmInferFactor>,
    #[cfg(feature = "sd-infer")]
    pub sd_infer: FactorInstanceState<SdInferFactor>,
    #[cfg(feature = "whisper-infer")]
    pub whisper_infer: FactorInstanceState<WhisperInferFactor>,
    /*** ***/
}

impl RuntimeFactorsInstanceState for TriggerFactorsInstanceState {
    fn get_with_table<F: Factor>(
        &mut self,
    ) -> Option<(&mut FactorInstanceState<F>, &mut ResourceTable)> {
        if let Some(state) = (&mut self.wasi as &mut (dyn Any + Send)).downcast_mut() {
            return Some((state, &mut self.__table));
        }

        if let Some(state) = (&mut self.variables as &mut (dyn Any + Send)).downcast_mut() {
            return Some((state, &mut self.__table));
        }

        if let Some(state) = (&mut self.key_value as &mut (dyn Any + Send)).downcast_mut() {
            return Some((state, &mut self.__table));
        }

        if let Some(state) = (&mut self.outbound_networking as &mut (dyn Any + Send)).downcast_mut()
        {
            return Some((state, &mut self.__table));
        }

        if let Some(state) = (&mut self.outbound_http as &mut (dyn Any + Send)).downcast_mut() {
            return Some((state, &mut self.__table));
        }

        /*** ***/
        #[cfg(feature = "dbs")]
        {
            if let Some(state) = (&mut self.dbs as &mut (dyn Any + Send)).downcast_mut() {
                return Some((state, &mut self.__table));
            }
        }

        #[cfg(feature = "llm-infer")]
        {
            if let Some(state) = (&mut self.llm_infer as &mut (dyn Any + Send)).downcast_mut() {
                return Some((state, &mut self.__table));
            }
        }

        #[cfg(feature = "sd-infer")]
        {
            if let Some(state) = (&mut self.sd_infer as &mut (dyn Any + Send)).downcast_mut() {
                return Some((state, &mut self.__table));
            }
        }

        #[cfg(feature = "whisper-infer")]
        {
            if let Some(state) = (&mut self.whisper_infer as &mut (dyn Any + Send)).downcast_mut() {
                return Some((state, &mut self.__table));
            }
        }
        /*** ***/

        None
    }

    fn table(&self) -> &ResourceTable {
        &self.__table
    }

    fn table_mut(&mut self) -> &mut ResourceTable {
        &mut self.__table
    }
}

impl AsInstanceState<TriggerFactorsInstanceState> for TriggerFactorsInstanceState {
    fn as_instance_state(&mut self) -> &mut Self {
        self
    }
}
