use bytes::Bytes;

use rquickjs::{
    class::Trace, function::Opt, ArrayBuffer, Class, Ctx, Exception, FromJs, IntoJs, JsLifetime,
    Null, Object, Result, TypedArray, Value,
};



use llrt_json::parse::json_parse;
use llrt_module_url::url_class::URL;

use llrt_utils::{bytes::ObjectBytes, class::get_class, object::ObjectExt};

use crate::{blob::Blob, headers::Headers};



#[rquickjs::class]
#[derive(rquickjs::JsLifetime)]
pub struct Request<'js> {
    url: String,
    method: String,
    headers: Option<Class<'js, Headers>>,
    body: Option<Value<'js>>,
}

impl<'js> Trace<'js> for Request<'js> {
    fn trace<'a>(&self, tracer: rquickjs::class::Tracer<'a, 'js>) {
        if let Some(headers) = &self.headers {
            headers.trace(tracer);
        }
        if let Some(body) = &self.body {
            body.trace(tracer);
        }
    }
}


impl<'js> Request<'js> {
    fn take_bytes(&mut self, ctx: &Ctx<'js>) -> Result<Option<ObjectBytes<'js>>> {
        match &self.body {
            Some(provided) => {
                let bytes = if let Some(blob) = get_class::<Blob>(provided)? {
                    ObjectBytes::Vec(blob.borrow().get_bytes())
                } else {
                    ObjectBytes::from(ctx, provided)?
                };
                Ok(Some(bytes))
            },
            None => Ok(None),
        }
    }
}


#[rquickjs::methods(rename_all = "camelCase")]
impl<'js> Request<'js> {
    #[qjs(constructor)]
    pub fn new(ctx: Ctx<'js>, input: Value<'js>, options: Opt<Value<'js>>) -> Result<Self> {
        let mut request = Self {
            url: String::from(""),
            method: "GET".to_string(),
            headers: None,
            body: None,
        };

        if input.is_string() {
            request.url = input.get()?;
        } else if let Ok(url) = URL::from_js(&ctx, input.clone()) {
            request.url = url.to_string();
        } else if input.is_object() {
            assign_request(&mut request, ctx.clone(), unsafe {
                input.as_object().unwrap_unchecked()
            })?;
        }
        if let Some(options) = options.0 {
            if let Some(obj) = options.as_object() {
                assign_request(&mut request, ctx.clone(), obj)?;
            }
        }
        if request.headers.is_none() {
            let headers = Class::instance(ctx, Headers::default())?;
            request.headers = Some(headers);
        }

        Ok(request)
    }

    #[qjs(get)]
    fn url(&self) -> String {
        self.url.clone()
    }

    #[qjs(get)]
    fn method(&self) -> String {
        self.method.clone()
    }

    #[qjs(get)]
    fn headers(&self) -> Option<Class<'js, Headers>> {
        self.headers.clone()
    }

    //TODO should implement readable stream
    #[qjs(get)]
    fn body(&self, ctx: Ctx<'js>) -> Result<Value<'js>> {
        if let Some(body) = &self.body {
            return Ok(body.clone());
        }
        Null.into_js(&ctx)
    }

    #[qjs(get)]
    fn keepalive(&self) -> bool {
        true
    }


    #[qjs(get)]
    fn body_used(&self) -> bool {
        self.body.is_some()
    }

    #[qjs(get)]
    fn mode(&self) -> String {
        "navigate".to_string()
    }

    #[qjs(get)]
    fn cache(&self) -> String {
        "no-store".to_string()
    }

    pub fn text(&mut self, ctx: Ctx<'js>) -> Result<String> {
        if let Some(bytes) = self.take_bytes(&ctx)? {
            let bytes = bytes.as_bytes();
            return Ok(String::from_utf8_lossy(bytes).to_string());
        }
        Ok("".into())
    }

    pub fn json(&mut self, ctx: Ctx<'js>) -> Result<Value<'js>> {
        if let Some(bytes) = self.take_bytes(&ctx)? {
            return json_parse(&ctx, bytes.as_bytes());
        }
        Err(Exception::throw_syntax(&ctx, "JSON input is empty"))
    }

    fn array_buffer(&mut self, ctx: Ctx<'js>) -> Result<ArrayBuffer<'js>> {
        if let Some(bytes) = self.take_bytes(&ctx)? {
            return ArrayBuffer::new(ctx, bytes.as_bytes());
        }
        ArrayBuffer::new(ctx, Vec::<u8>::new())
    }

    fn bytes(&mut self, ctx: Ctx<'js>) -> Result<Value<'js>> {
        if let Some(bytes) = self.take_bytes(&ctx)? {
            return TypedArray::new(ctx, bytes.as_bytes()).map(|m| m.into_value());
        }
        TypedArray::new(ctx, Vec::<u8>::new()).map(|m| m.into_value())
    }

    fn blob(&mut self, ctx: Ctx<'js>) -> Result<Blob> {
        if let Some(bytes) = self.take_bytes(&ctx)? {
            let headers = Headers::from_value(&ctx, self.headers().unwrap().as_value().clone())?;
            let mime_type = headers
                .iter()
                .find_map(|(k, v)| (k == "content-type").then(|| v.to_string()));
            return Ok(Blob::from_bytes(bytes.into(), mime_type));
        }
        Ok(Blob::from_bytes(Vec::<u8>::new(), None))
    }

    fn clone(&mut self, ctx: Ctx<'js>) -> Result<Self> {
        let headers = if let Some(headers) = &self.headers {
            Some(Class::<Headers>::instance(
                ctx.clone(),
                headers.borrow().clone(),
            )?)
        } else {
            None
        };

        Ok(Self {
            url: self.url.clone(),
            method: self.url.clone(),
            headers,
            body: self.body.clone(),
        })
    }

}

pub fn from_http_request<'js>(ctx: &Ctx<'js>, input: http::Request<Bytes>) -> Result<Request<'js>> {
    let url = input.uri().to_string();
    let method = input.method().to_string();

    let headers = input.headers().clone();
    //println!("headers: {:?}", headers);

    let body = input.into_body();

    let req_obj = Object::new(ctx.clone())?;

    if &method.to_uppercase() != "GET" && &method.to_uppercase() != "HEAD" {
        req_obj.set("body", body.into_js(&ctx))?;
    }

    req_obj.set("url", url.into_js(&ctx))?;
    req_obj.set("method", method.into_js(&ctx))?;

    let req_headers = Object::new(ctx.clone())?;

    for (k, v) in headers.iter() {
        req_headers.set(k.as_str(), v.to_str().unwrap_or_default())?   }

    req_obj.set("headers", req_headers)?;

    Request::new(ctx.clone(), req_obj.into_value(), Opt::from(None))
}


fn assign_request<'js>(request: &mut Request<'js>, ctx: Ctx<'js>, obj: &Object<'js>) -> Result<()> {
    if let Some(url) = obj.get_optional("url")? {
        request.url = url;
    }
    if let Some(method) = obj.get_optional("method")? {
        request.method = method;
    }

    if let Some(body) = obj.get_optional::<_, Value>("body")? {
        if !body.is_undefined() && !body.is_null() {
            if let "GET" | "HEAD" = request.method.as_str() {
                return Err(Exception::throw_type(
                    &ctx,
                    "Failed to construct 'Request': Request with GET/HEAD method cannot have body.",
                ));
            }

            request.body = if let Some(blob) = body.as_object().and_then(Class::<Blob>::from_object)
            {
                let blob = blob.borrow();
                Some(TypedArray::<u8>::new(ctx.clone(), blob.get_bytes())?.into_value())
            } else {
                Some(body)
            }
        }
    }

    if let Some(headers) = obj.get_optional("headers")? {
        let headers = Headers::from_value(&ctx, headers)?;
        let headers = Class::instance(ctx, headers)?;
        request.headers = Some(headers);
    }

    Ok(())
}
