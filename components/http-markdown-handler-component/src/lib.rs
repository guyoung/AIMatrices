use serde_json::Value;

use wasmruntime_comp_sdk::http::{IntoResponse, Request, ResponseBuilder};
use wasmruntime_comp_sdk::http_component;

#[http_component]
fn handle(req: Request) -> anyhow::Result<impl IntoResponse> {
    Ok(Response::new(200, "Hello, world!"))
}
