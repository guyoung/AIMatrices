use std::net::SocketAddr;

use anyhow::{anyhow, Context, Result};

use futures::TryFutureExt;
use tokio::{sync::oneshot, task};

use http::{HeaderName, HeaderValue};
use hyper::{Request, Response};

use wasmtime_wasi_http::bindings::http::types::Scheme;
use wasmtime_wasi_http::{bindings::Proxy, body::HyperIncomingBody as Body, WasiHttpView};

use spin_factors::RuntimeFactors;
use spin_http::routes::RouteMatch;
use spin_http::trigger::HandlerType;

use crate::{headers::prepare_request_headers, http_trigger::TriggerInstanceBuilder, HttpExecutor};

/// An [`HttpExecutor`] that uses the `wasi:http/incoming-handler` interface.
#[derive(Clone)]
pub struct WasiHttpExecutor {
    pub handler_type: HandlerType,
}

impl HttpExecutor for WasiHttpExecutor {
    async fn execute<F: RuntimeFactors>(
        &self,
        instance_builder: TriggerInstanceBuilder<'_, F>,
        route_match: &RouteMatch,
        mut req: Request<Body>,
        client_addr: SocketAddr,
    ) -> Result<Response<Body>> {
        let component_id = route_match.component_id();

        tracing::trace!(
            "Executing request using the Wasi spin-executor for component {component_id}"
        );

        let (instance, mut store) = instance_builder.instantiate(()).await?;

        let headers = prepare_request_headers(&req, route_match, client_addr)?;
        req.headers_mut().clear();
        req.headers_mut()
            .extend(headers.into_iter().filter_map(|(n, v)| {
                let Ok(name) = n.parse::<HeaderName>() else {
                    return None;
                };
                let Ok(value) = HeaderValue::from_bytes(v.as_bytes()) else {
                    return None;
                };
                Some((name, value))
            }));

        let mut wasi_http = spin_factor_outbound_http::OutboundHttpFactor::get_wasi_http_impl(
            store.data_mut().factors_instance_state_mut(),
        )
        .context("missing OutboundHttpFactor")?;

        let (parts, body) = req.into_parts();
        let body = wasmtime_wasi_http::body::HostIncomingBody::new(
            body,
            std::time::Duration::from_secs(600),
        );
        let request = wasmtime_wasi_http::types::HostIncomingRequest::new(
            &mut wasi_http,
            parts,
            Scheme::Http,
            Some(body),
        )?;
        let request = wasi_http.table().push(request)?;

        let (response_tx, response_rx) = oneshot::channel();
        let response = wasi_http.new_response_outparam(response_tx)?;

        drop(wasi_http);

        enum Handler {
            Latest(Proxy),
        }

        let handler = match self.handler_type {
            HandlerType::Wasi0_2 => Handler::Latest(Proxy::new(&mut store, &instance)?),
            HandlerType::Wagi => unreachable!("should have used WagiExecutor instead"),
        };

        let handle = task::spawn(async move {
            let result = match handler {
                Handler::Latest(handler) => {
                    handler
                        .wasi_http_incoming_handler()
                        .call_handle(&mut store, request, response)
                        .await
                }
            };

            tracing::trace!(
                "wasi-http memory consumed: {}",
                store.data().core_state().memory_consumed()
            );

            result
        });

        match response_rx.await {
            Ok(response) => {
                task::spawn(
                    async move {
                        handle
                            .await
                            .context("guest invocation panicked")?
                            .context("guest invocation failed")?;

                        Ok(())
                    }
                    .map_err(|e: anyhow::Error| {
                        tracing::warn!("component error after response: {e:?}");
                    }),
                );

                Ok(response.context("guest failed to produce a response")?)
            }

            Err(_) => {
                handle
                    .await
                    .context("guest invocation panicked")?
                    .context("guest invocation failed")?;

                Err(anyhow!(
                    "guest failed to produce a response prior to returning"
                ))
            }
        }
    }
}
