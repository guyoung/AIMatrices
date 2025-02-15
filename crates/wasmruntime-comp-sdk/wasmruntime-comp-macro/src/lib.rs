use proc_macro::TokenStream;
use quote::quote;

const WIT_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/wit");

#[proc_macro_attribute]
pub fn redis_component(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let func = syn::parse_macro_input!(item as syn::ItemFn);
    let func_name = &func.sig.ident;
    let await_postfix = func.sig.asyncness.map(|_| quote!(.await));
    let preamble = preamble(Export::Redis);

    quote!(
        #func
        mod __wasmruntime_comp_redis {
            mod preamble {
                #preamble
            }
            impl self::preamble::exports::wasmruntime::comp::inbound_redis::Guest for preamble::Comp {
                fn handle_message(msg: self::preamble::exports::wasmruntime::comp::inbound_redis::Payload) -> Result<(), self::preamble::wasmruntime::comp::redis_types::Error> {
                    ::wasmruntime_comp_sdk::http::run(async move {
                        match super::#func_name(msg.try_into().expect("cannot convert from payload"))#await_postfix {
                            Ok(()) => Ok(()),
                            Err(e) => {
                                eprintln!("{}", e);
                                Err(self::preamble::wasmruntime::comp::redis_types::Error::Error)
                            },
                        }
                    })
                }
            }
        }
    )
        .into()
}


#[proc_macro_attribute]
pub fn http_component(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let func = syn::parse_macro_input!(item as syn::ItemFn);
    let func_name = &func.sig.ident;
    let preamble = preamble(Export::WasiHttp);
    let is_native_wasi_http_handler = func.sig.inputs.len() == 2;
    let await_postfix = func.sig.asyncness.map(|_| quote!(.await));
    let handler = if is_native_wasi_http_handler {
        quote! { super::#func_name(req, response_out)#await_postfix }
    } else {
        quote! { handle_response(response_out, super::#func_name(req)#await_postfix).await }
    };

    quote!(
        #func
        mod __wasmruntime_comp_wasi_http {
            mod preamble {
              #preamble
            }
            impl self::preamble::exports::wasi::http::incoming_handler::Guest for self::preamble::Comp {
                fn handle(request: self::preamble::wasi::http::types::IncomingRequest, response_out: self::preamble::wasi::http::types::ResponseOutparam) {
                    let request: ::wasmruntime_comp_sdk::http::IncomingRequest = ::std::convert::Into::into(request);
                    let response_out: ::wasmruntime_comp_sdk::http::ResponseOutparam = ::std::convert::Into::into(response_out);
                    ::wasmruntime_comp_sdk::http::run(async move {
                        match ::wasmruntime_comp_sdk::http::conversions::TryFromIncomingRequest::try_from_incoming_request(request).await {
                            ::std::result::Result::Ok(req) => #handler,
                            ::std::result::Result::Err(e) => handle_response(response_out, e).await,
                        }
                    });
                }
            }

            async fn handle_response<R: ::wasmruntime_comp_sdk::http::IntoResponse>(response_out: ::wasmruntime_comp_sdk::http::ResponseOutparam, resp: R) {
                let mut response = ::wasmruntime_comp_sdk::http::IntoResponse::into_response(resp);
                let body = ::std::mem::take(response.body_mut());
                match ::std::convert::TryInto::try_into(response) {
                    ::std::result::Result::Ok(response) => {
                        if let Err(e) = ::wasmruntime_comp_sdk::http::ResponseOutparam::set_with_body(response_out, response, body).await {
                            ::std::eprintln!("Could not set `ResponseOutparam`: {e}");
                        }
                    }
                    ::std::result::Result::Err(e) => {
                        ::std::eprintln!("Could not convert response: {e}");
                    }
                }
            }

            impl From<self::preamble::wasi::http::types::IncomingRequest> for ::wasmruntime_comp_sdk::http::IncomingRequest {
                fn from(req: self::preamble::wasi::http::types::IncomingRequest) -> Self {
                    unsafe { Self::from_handle(req.into_handle()) }
                }
            }

            impl From<::wasmruntime_comp_sdk::http::OutgoingResponse> for self::preamble::wasi::http::types::OutgoingResponse {
                fn from(resp: ::wasmruntime_comp_sdk::http::OutgoingResponse) -> Self {
                    unsafe { Self::from_handle(resp.into_handle()) }
                }
            }

            impl From<self::preamble::wasi::http::types::ResponseOutparam> for ::wasmruntime_comp_sdk::http::ResponseOutparam {
                fn from(resp: self::preamble::wasi::http::types::ResponseOutparam) -> Self {
                    unsafe { Self::from_handle(resp.into_handle()) }
                }
            }
        }

    )
        .into()
}

#[derive(Copy, Clone)]
enum Export {
    WasiHttp,
    Redis,
}

fn preamble(export: Export) -> proc_macro2::TokenStream {
    let export_decl = match export {
        Export::WasiHttp => quote!("wasi:http/incoming-handler": Comp),
        Export::Redis => quote!("wasmruntime:comp/inbound-redis": Comp),
    };
    let world = match export {
        Export::WasiHttp => quote!("wasi-http-trigger"),
        Export::Redis => quote!("redis-trigger"),
    };
    quote! {
        #![allow(missing_docs)]
        ::wasmruntime_comp_sdk::wit_bindgen::generate!({
            world: #world,
            path: #WIT_PATH,
            runtime_path: "::wasmruntime_comp_sdk::wit_bindgen::rt",
            exports: {
                #export_decl
            }
        });
        pub struct Comp;
    }
}
