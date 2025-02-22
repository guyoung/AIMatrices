pub mod key_value;

pub use wasmruntime_comp_macro::*;

pub mod wit {
    wit_bindgen::generate!({
        world: "platform",
        path: "./wit",
        with: {
            "wasi:io/error@0.2.0": wasmruntime_comp_executor::bindings::wasi::io::error,
            "wasi:io/streams@0.2.0": wasmruntime_comp_executor::bindings::wasi::io::streams,
            "wasi:io/poll@0.2.0": wasmruntime_comp_executor::bindings::wasi::io::poll,


        }
    });

    pub use wasmruntime::comp;
}


/// Needed by the export macro
///
/// See [this commit](https://github.com/bytecodealliance/wit-bindgen/pull/394/commits/9d2ea88f986f4a883ba243449e3a070cac18958e) for more info.
#[cfg(target_arch = "wasm32")]
pub use wit::__link_section;

pub mod http_internal;

pub use http_internal as http;

pub use wit::comp::variables;

pub use wit_bindgen;

#[cfg(feature = "dbs")]
pub mod dbs;

#[cfg(feature = "llm-infer")]
pub mod llm_infer;
#[cfg(feature = "sd-infer")]
pub mod sd_infer;
