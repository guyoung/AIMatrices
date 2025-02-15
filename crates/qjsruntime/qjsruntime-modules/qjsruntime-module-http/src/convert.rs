use bytes::Bytes;

use rquickjs::{Ctx, Exception, Object, Value};

use crate::response::Response;

pub fn convert_js_value_to_http_response<'js>(
    ctx: &Ctx<'js>,
    input: Value<'js>,
) -> rquickjs::Result<http::Response<Bytes>> {
    match input.type_of() {
        rquickjs::Type::String => {
            let str = input.as_string().unwrap().to_string()?;

            let res = http::Response::builder()
                .status(http::status::StatusCode::OK)
                .body(str.into())
                .map_err(|e| Exception::throw_type(&ctx, &format!("{:?}", e)))?;

            Ok(res)
        }

        rquickjs::Type::Object | rquickjs::Type::Exception => {
            let obj = input.into_object().unwrap();

            if obj.is_error() {
                let e: String = obj.get(rquickjs::atom::PredefinedAtom::Message)?;

                let res = http::Response::builder()
                    .status(http::status::StatusCode::INTERNAL_SERVER_ERROR)
                    .body(e.into())
                    .map_err(|e| Exception::throw_type(&ctx, &format!("{:?}", e)))?;

                return Ok(res);
            }

            if let Some(obj) = obj.as_class::<Response>() {
                let status: Value = obj.get("status")?;
                let status = status.as_int().unwrap_or(200) as u16;
                let headers: Object = obj.get("headers")?;
                let header_keys: Vec<_> = headers.keys::<Value>().collect();

                let obj_borrow = obj.borrow();
                let obj_borrow: &Response = &obj_borrow;

                /***
                let headers = headers::Headers::from_value(ctx, headers.into_value())?;

                 let headers = &obj_borrow.headers;
                 let headers_borrow = headers.borrow();
                 let headers: &Headers = &headers_borrow;
                 ***/

                let body = &obj_borrow.body;

                let body = if let Some(body) = body {
                    Bytes::from(convert_js_value_to_bytes(&ctx, body)?)
                } else {
                    Bytes::new()
                };

                let mut builder = http::Response::builder().status(status);

                for k in header_keys {
                    if let Ok(k) = k {
                        if let Some(k) = k.as_string() {
                            let k = k.to_string()?;
                            let v: Value = headers.get(&k)?;

                            if let Some(v) = v.as_string() {
                                let v = v.to_string()?;

                                builder = builder.header(k, v);
                            }
                        }
                    }
                }

                builder = builder.header("content-length", body.len().to_string().as_str());

                let res = builder
                    .body(body)
                    .map_err(|e| Exception::throw_type(&ctx, &format!("{:?}", e)))?;

                return Ok(res);
            }

            let res = http::Response::builder()
                .status(http::status::StatusCode::OK)
                .body(Bytes::new())
                .map_err(|e| Exception::throw_type(&ctx, &format!("{:?}", e)))?;

            Ok(res)
        }

        _ => {
            let res = http::Response::builder()
                .status(http::status::StatusCode::OK)
                .body(Bytes::new())
                .map_err(|e| Exception::throw_type(&ctx, &format!("{:?}", e)))?;

            Ok(res)
        }
    }
}

pub fn convert_js_value_to_bytes<'js>(
    ctx: &Ctx<'js>,
    input: &Value<'js>,
) -> rquickjs::Result<Vec<u8>> {
    match input.type_of() {
        rquickjs::Type::Object => {
            let obj = input.as_object();

            if let Some(obj) = obj {
                if let Some(arr) = obj.as_typed_array::<u8>() {
                    if let Some(data) = arr.as_bytes() {
                        return Ok(data.to_vec());
                    }
                }

                let data = llrt_json::stringify::json_stringify(&ctx, obj.clone().into_value())?;

                if let Some(data) = data {
                    return Ok(data.as_bytes().to_vec());
                }
            }

            Ok(Vec::new())
        }

        rquickjs::Type::String => {
            if let Some(str) = input.as_string() {
                let str = str.to_string()?;

                Ok(str.into())
            } else {
                Ok(Vec::new())
            }
        }
        _ => Ok(Vec::new()),
    }
}
