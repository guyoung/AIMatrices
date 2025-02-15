use wasmruntime_comp_sdk::http::{IntoResponse, Response};
use wasmruntime_comp_sdk::http_component;


#[http_component]
fn hello_world(_req: http::Request<()>) -> anyhow::Result<impl IntoResponse> {


    Ok(Response::new(200, "Hello, world!"))
}
