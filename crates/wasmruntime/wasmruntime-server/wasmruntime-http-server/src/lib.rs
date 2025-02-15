mod server2;

use std::error::Error;
use std::net::{Ipv4Addr, SocketAddr, ToSocketAddrs};
use std::path::PathBuf;
use std::sync::Arc;

use anyhow::Context;

use wasmtime_wasi_http::bindings::http::types::ErrorCode;

use spin_app::App;
use spin_http::config::HttpTriggerConfig;
use spin_factors_executor::FactorsExecutor;

use wasmruntime_core::{ FactorsConfig, Trigger, RuntimeFactorsBuilder};
use wasmruntime_core::loader::ComponentLoader as ComponentLoaderImpl;
use wasmruntime_system::{FactorsBuilder, TriggerFactors};
use wasmruntime_http_trigger::http_trigger::{HttpTrigger, TriggerApp};


pub type HttpTriggerApp = TriggerApp<TriggerFactors>;

#[derive(Debug, PartialEq)]
enum NotFoundRouteKind {
    Normal(String),
    WellKnown,
}

pub fn parse_listen_addr(addr: &str) -> anyhow::Result<SocketAddr> {
    let addrs: Vec<SocketAddr> = addr.to_socket_addrs()?.collect();
    // Prefer 127.0.0.1 over e.g. [::1] because CHANGE IS HARD
    if let Some(addr) = addrs
        .iter()
        .find(|addr| addr.is_ipv4() && addr.ip() == Ipv4Addr::LOCALHOST)
    {
        return Ok(*addr);
    }
    // Otherwise, take the first addr (OS preference)
    addrs.into_iter().next().context("couldn't resolve address")
}

/// Translate a [`hyper::Error`] to a wasi-http `ErrorCode` in the context of a request.
pub fn hyper_request_error(err: hyper::Error) -> ErrorCode {
    // If there's a source, we might be able to extract a wasi-http error from it.
    if let Some(cause) = err.source() {
        if let Some(err) = cause.downcast_ref::<ErrorCode>() {
            return err.clone();
        }
    }

    tracing::warn!("hyper request error: {err:?}");

    ErrorCode::HttpProtocolError
}

pub fn dns_error(rcode: String, info_code: u16) -> ErrorCode {
    ErrorCode::DnsError(wasmtime_wasi_http::bindings::http::types::DnsErrorPayload {
        rcode: Some(rcode),
        info_code: Some(info_code),
    })
}



pub async fn start(
    listen_addr: SocketAddr,
    trigger_app: Arc<HttpTriggerApp>,
    user: Option<String>,
    pass: Option<String>
) -> anyhow::Result<()> {
    let server = server2::HttpServer::new(
        listen_addr,
        trigger_app,
        user,
        pass
    )?;



    server.serve().await?;

    Ok(())
}


pub async fn crate_http_trigger_app(
    app: &App,
    common_options: &FactorsConfig,
    disable_cache: bool,
    cache: &Option<PathBuf>,
    disable_pooling: bool,
) -> anyhow::Result<crate::HttpTriggerApp> {

    let mut http_trigger = HttpTrigger::new(&app)?;

    let mut engine_config = spin_core::Config::default();

    // Apply --cache / --disable-cache
    if !disable_cache {
        engine_config.enable_cache(cache)?;
    }

    if disable_pooling {
        engine_config.disable_pooling();
    }


    let mut core_engine_builder = {
        <HttpTrigger as Trigger<TriggerFactors>>::update_core_config(
            &mut http_trigger,
            &mut engine_config,
        )?;

        spin_core::Engine::builder(&engine_config)?
    };

    <HttpTrigger as Trigger<TriggerFactors>>::add_to_linker(
        &mut http_trigger,
        core_engine_builder.linker(),
    )?;

    let mut runtime_factors_builder = FactorsBuilder::new();

    let factors = runtime_factors_builder.build(&common_options)?;

    let mut executor = FactorsExecutor::new(core_engine_builder, factors)?;
    runtime_factors_builder.configure_app(&mut executor, &common_options)?;
    let executor = Arc::new(executor);

    let components = Vec::from_iter(
        app
            .trigger_configs::<HttpTriggerConfig>("http")?
            .into_iter()
            .map(|(_, config)| config.component),
    );

    let http_app = {
        executor
            .load_app(
                app.clone(),
                components,
                &ComponentLoaderImpl::new(),
            )
            .await?
    };

    Ok(http_app)
}