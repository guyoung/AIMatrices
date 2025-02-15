use std::net::SocketAddr;

use wasmtime::component::Linker;

use spin_app::App;
use spin_factors::RuntimeFactors;

use wasmruntime_core::{Trigger, TriggerInstanceState};


/// A TriggerApp for the HTTP trigger.
pub type TriggerApp<F> = wasmruntime_core::TriggerApp<HttpTrigger, F>;

/// A TriggerInstanceBuilder for the HTTP trigger.
pub type TriggerInstanceBuilder<'a, F> =
    wasmruntime_core::TriggerInstanceBuilder<'a, HttpTrigger, F>;

pub struct HttpTriggerArgs {
    /// IP address and port to listen on
    pub address: SocketAddr,

}

/// The Spin HTTP trigger.

pub struct HttpTrigger {}

impl<F: RuntimeFactors> Trigger<F> for HttpTrigger {
    const TYPE: &'static str = "http";

    type InstanceState = ();

    fn update_core_config(&mut self, config: &mut spin_core::Config) -> anyhow::Result<()> {
        let _ = config;
        Ok(())
    }

    /// Update the [`Linker`] for this wasmruntime-core.
    fn add_to_linker(
        &mut self,
        linker: &mut Linker<TriggerInstanceState<Self, F>>,
    ) -> anyhow::Result<()> {
        let _ = linker;
        Ok(())
    }

    fn supported_host_requirements() -> Vec<&'static str> {
        vec![spin_app::locked::SERVICE_CHAINING_KEY]
    }
}

impl HttpTrigger {
    /// Create a new `HttpTrigger`.
    pub fn new(app: &App) -> anyhow::Result<Self> {
        Self::validate_app(app)?;

        Ok(Self {})
    }

    fn validate_app(_app: &App) -> anyhow::Result<()> {

        Ok(())
    }
}
