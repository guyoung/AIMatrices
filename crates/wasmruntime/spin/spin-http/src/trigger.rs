use serde::{Deserialize, Serialize};

use wasmtime::component::Component;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Metadata {
    // The based url
    #[serde(default = "default_base")]
    pub base: String,
}

pub fn default_base() -> String {
    "/".into()
}

/// The type of http handler export used by a component.
#[derive(Clone, Copy)]
pub enum HandlerType {
    Wagi,
    Wasi0_2,
}

/// The `incoming-handler` export prefix for all `wasi:http` 0.2 versions
pub const WASI_HTTP_EXPORT_0_2_PREFIX: &str = "wasi:http/incoming-handler@0.2";


impl HandlerType {
    /// Determine the handler type from the exports of a component.
    pub fn from_component(
        engine: &wasmtime::Engine,
        component: &Component,
    ) -> anyhow::Result<HandlerType> {
        let mut handler_ty = None;

        let mut set = |ty: HandlerType| {
            if handler_ty.is_none() {
                handler_ty = Some(ty);
                Ok(())
            } else {
                Err(anyhow::anyhow!(
                    "component exports multiple different handlers but \
                     it's expected to export only one"
                ))
            }
        };
        let ty = component.component_type();
        for (name, _) in ty.exports(engine) {
            match name {
                name if name.starts_with(WASI_HTTP_EXPORT_0_2_PREFIX) => set(HandlerType::Wasi0_2)?,
                _ => {}
            }
        }

        handler_ty.ok_or_else(|| {
            anyhow::anyhow!(
                r"Expected component to export  \
                `{WASI_HTTP_EXPORT_0_2_PREFIX}.*`, \
                but it exported none of those"
            )
        })
    }
}
