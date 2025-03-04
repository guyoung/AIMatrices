use std::{io::Cursor, net::SocketAddr};

use anyhow::{Context, Result};

use http_body_util::BodyExt;
use hyper::{Request, Response};

use wasmtime_wasi::pipe::MemoryOutputPipe;
use wasmtime_wasi_http::body::HyperIncomingBody as Body;

use spin_factor_wasi::WasiFactor;
use spin_factors::RuntimeFactors;
use spin_http::{config::WagiTriggerConfig, routes::RouteMatch, wagi};

use crate::{headers::compute_default_headers, http_trigger::TriggerInstanceBuilder, HttpExecutor};

#[derive(Clone)]
pub struct WagiHttpExecutor {
    pub wagi_config: WagiTriggerConfig,
}

impl HttpExecutor for WagiHttpExecutor {
    async fn execute<F: RuntimeFactors>(
        &self,
        mut instance_builder: TriggerInstanceBuilder<'_, F>,
        route_match: &RouteMatch,
        req: Request<Body>,
        client_addr: SocketAddr,
    ) -> Result<Response<Body>> {
        let component = route_match.component_id();

        tracing::trace!(
            "Executing request using the Wagi spin-executor for component {}",
            component
        );

        let uri_path = req.uri().path();

        // Build the argv array by starting with the config for `argv` and substituting in
        // script name and args where appropriate.
        let script_name = uri_path.to_string();

        /***
        let args = req.uri().query().unwrap_or_default().replace('&', " ");
        let argv = self
            .wagi_config
            .argv
            .clone()
            .replace("${SCRIPT_NAME}", &script_name)
            .replace("${ARGS}", &args);

        ***/

        let mut args: Vec<String> = Vec::new();
        args.push(script_name);

        let parsed_url = url::Url::parse(req.uri().to_string().as_str())?;

        for (key, value) in parsed_url.query_pairs() {
            args.push(key.to_string());

            if !value.is_empty() {
                args.push(value.to_string());
            }
        }

        let (parts, body) = req.into_parts();

        let body = body.collect().await?.to_bytes().to_vec();
        let len = body.len();

        // TODO
        // The default host and TLS fields are currently hard-coded.
        let mut headers =
            wagi::build_headers(route_match, &parts, len, client_addr, "default_host", false);

        let default_host = http::HeaderValue::from_str("localhost")?;
        let host = std::str::from_utf8(
            parts
                .headers
                .get("host")
                .unwrap_or(&default_host)
                .as_bytes(),
        )?;

        // Add the default Spin headers.
        // This sets the current environment variables Wagi expects (such as
        // `PATH_INFO`, or `X_FULL_URL`).
        // Note that this overrides any existing headers previously set by Wagi.
        for (keys, val) in compute_default_headers(&parts.uri, host, route_match, client_addr)? {
            headers.insert(keys[1].to_string(), val);
        }

        let stdout = MemoryOutputPipe::new(usize::MAX);

        let wasi_builder = instance_builder
            .factor_builder::<WasiFactor>()
            .context("The wagi HTTP trigger was configured without the required wasi support")?;

        // Set up Wagi environment
        /***
        wasi_builder.args(argv.split(' '));
        ***/

        wasi_builder.args(args);

        wasi_builder.env(headers);
        wasi_builder.stdin_pipe(Cursor::new(body));
        wasi_builder.stdout(stdout.clone());

        let (instance, mut store) = instance_builder.instantiate(()).await?;

        let command = wasmtime_wasi::bindings::Command::new(&mut store, &instance)?;

        tracing::trace!("Calling Wasm entry point");

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

        wagi::compose_response(&stdout)
    }
}

fn ignore_successful_proc_exit_trap(guest_err: anyhow::Error) -> Result<Result<(), ()>> {
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
