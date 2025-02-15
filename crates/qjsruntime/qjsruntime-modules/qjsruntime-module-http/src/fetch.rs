use std::collections::BTreeMap;

use rquickjs::function::Opt;
use rquickjs::Coerced;
use rquickjs::FromJs;
use rquickjs::Object;
use rquickjs::{Ctx, Exception, Result, Value};

use llrt_utils::bytes::ObjectBytes;

use wasmruntime_comp_sdk::http::{
    Method, Request as HttpRequest, RequestBuilder, Response as HttpResponse,
};

pub fn fetch_sync<'js>(
    ctx: Ctx<'js>,
    url: Value<'js>,
    options: Opt<Value<'js>>,
) -> Result<Value<'js>> {
    let requst = parse_request(&ctx, url, options)
        .map_err(|e| Exception::throw_type(&ctx, &format!("Failed to fetch {:?}", e)))?;

    let response: HttpResponse = wasmruntime_comp_sdk::http::send_sync(requst)
        .map_err(|e| Exception::throw_type(&ctx, &format!("Failed to fetch {:?}", e)))?;

    let status = response.status();

    let new_headers = Object::new(ctx.clone())?;
    let headers = response.headers();

    for (k, v) in headers {
        new_headers.set(k, v.as_str().unwrap_or_default())?;
    }

    let body = response.body();

    let result = Object::new(ctx)?;
    result.set("status", status)?;
    result.set("headers", new_headers)?;
    result.set("body", body)?;

    Ok(result.into())
}

fn parse_request<'js>(
    ctx: &Ctx<'js>,
    url: Value<'js>,
    options: Opt<Value<'js>>,
) -> Result<HttpRequest> {
    let url = url.get::<Coerced<String>>()?.to_string();
    let mut method = None;
    let mut headers: Option<BTreeMap<String, Coerced<String>>> = None;
    let mut body = None;

    let mut arg_opts = None;

    if let Some(options) = options.0 {
        arg_opts = options.into_object();
    }

    if arg_opts.is_some() {
        if let Some(method_opt) = get_option::<String>("method", arg_opts.as_ref())? {
            method = Some(match method_opt.to_uppercase().as_str() {
                "GET" => Ok(Method::Get),
                "POST" => Ok(Method::Post),
                "PUT" => Ok(Method::Put),
                "CONNECT" => Ok(Method::Connect),
                "HEAD" => Ok(Method::Head),
                "PATCH" => Ok(Method::Patch),
                "DELETE" => Ok(Method::Delete),
                _ => Err(Exception::throw_type(
                    ctx,
                    &["Invalid HTTP method: ", &method_opt].concat(),
                )),
            }?);
        }

        if let Some(headers_op) = get_option::<Value>("headers", arg_opts.as_ref())? {
            if headers_op.is_object() {
                headers = headers_op.get().unwrap_or_default();
            }
        }

        if let Some(body_opt) = get_option::<Value>("body", arg_opts.as_ref())? {
            let bytes = ObjectBytes::from(ctx, &body_opt)?;
            body = Some(bytes.into_bytes());
        }
    }

    let method = method.unwrap_or(Method::Get);

    let mut request_bulder = RequestBuilder::new(method, url);

    if let Some(headers) = headers {
        for (k, v) in headers {
            request_bulder.header(k, v.0);
        }
    }

    if let Some(body) = body {
        request_bulder.body(body);
    }

    let request = request_bulder.build();

    Ok(request)
}

fn get_option<'js, V: FromJs<'js> + Sized>(
    arg: &str,
    opt: Option<&Object<'js>>,
) -> Result<Option<V>> {
    if let Some(opt) = opt {
        if let Some(value) = opt.get::<_, Option<V>>(arg)? {
            return Ok(Some(value));
        }
    }

    Ok(None)
}
