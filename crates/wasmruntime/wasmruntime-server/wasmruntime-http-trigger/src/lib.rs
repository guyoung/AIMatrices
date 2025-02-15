//! Implementation for the Spin HTTP engine.

pub mod headers;
pub mod http_trigger;

pub mod wagi;
pub mod wasi;

use std::future::Future;
use std::net::SocketAddr;

use http::{Request, Response};

pub use wasmtime_wasi_http::body::HyperIncomingBody as Body;

use spin_factors::RuntimeFactors;
use spin_http::routes::RouteMatch;

use http_trigger::TriggerInstanceBuilder;

/// An HTTP spin-executor.
pub trait HttpExecutor: Clone + Send + Sync + 'static {
    fn execute<F: RuntimeFactors>(
        &self,
        instance_builder: TriggerInstanceBuilder<F>,
        route_match: &RouteMatch,
        req: Request<Body>,
        client_addr: SocketAddr,
    ) -> impl Future<Output = anyhow::Result<Response<Body>>>;
}
