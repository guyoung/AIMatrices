use bytes::Bytes;

use wasmruntime_comp_sdk::http_component;

use qjsruntime_core::vm::Vm;

#[http_component]
fn hello_world(req: http::Request<Bytes>) -> anyhow::Result<http::Response<Bytes>> {
    let code = std::fs::read("/codes/index.js");

    if code.is_err() {
        let res =  http::Response::builder()
            .status(http::status::StatusCode::INTERNAL_SERVER_ERROR)
            .body("Code file not exist".into())?;

        return Ok(res);
    }

    let code = String::from_utf8_lossy(&code.unwrap()).to_string();

    let vm = Vm::new().unwrap();

    let res = vm.http_request_run(req, code.as_bytes(), false);

    match res {
        Ok(res) => Ok(res),
        Err(e) => {
            let res =  http::Response::builder()
                .status(http::status::StatusCode::INTERNAL_SERVER_ERROR)
                .body(format!("{:?}",e).into())?;

            Ok(res)
        },
    }
}
