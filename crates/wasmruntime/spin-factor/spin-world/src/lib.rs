#![allow(missing_docs)]
#![allow(non_camel_case_types)] // bindgen emits Host_Pre and Host_Indices

pub use async_trait::async_trait;

wasmtime::component::bindgen!({
    inline: r#"
    package wasmruntime:runtime;
    world host {
        include wasmruntime:comp/platform@1.0.0;
        include wasi:keyvalue/imports@0.2.0-draft2;
    }
    "#,

    path: "../../wit",
    async: true,
    // The following is a roundabout way of saying "the host implementations for these interfaces don't trap"
    trappable_error_type: {
        "wasmruntime:comp/variables@1.0.0/error" => comp::variables::Error,
        "wasi:config/store@0.2.0-draft-2024-09-27/error" => wasi::config::store::Error,
        "wasi:keyvalue/store/error" => wasi::keyvalue::store::Error,
        "wasi:keyvalue/atomics/cas-error" => wasi::keyvalue::atomics::CasError,
    },
    trappable_imports: true,
});

pub use wasmruntime::comp as comp;
