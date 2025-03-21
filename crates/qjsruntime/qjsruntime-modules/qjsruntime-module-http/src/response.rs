use std::{
    collections::{BTreeMap, HashMap},
    time::Instant,
};

use either::Either;
use once_cell::sync::Lazy;

use rquickjs::{
    class::{Trace, Tracer},
    function::Opt,
    ArrayBuffer, Class, Coerced, Ctx, Exception, IntoJs, JsLifetime, Null, Object, Result,
    TypedArray, Value,
};

use llrt_json::parse::json_parse;
use llrt_module_url::url_class::URL;
use llrt_utils::bytes::ObjectBytes;

use crate::{blob::Blob, headers::Headers};

static STATUS_TEXTS: Lazy<HashMap<u16, &'static str>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert(100, "Continue");
    map.insert(101, "Switching Protocols");
    map.insert(102, "Processing");
    map.insert(103, "Early Hints");
    map.insert(200, "OK");
    map.insert(201, "Created");
    map.insert(202, "Accepted");
    map.insert(203, "Non-Authoritative Information");
    map.insert(204, "No Content");
    map.insert(205, "Reset Content");
    map.insert(206, "Partial Content");
    map.insert(207, "Multi-Status");
    map.insert(208, "Already Reported");
    map.insert(226, "IM Used");
    map.insert(300, "Multiple Choices");
    map.insert(301, "Moved Permanently");
    map.insert(302, "Found");
    map.insert(303, "See Other");
    map.insert(304, "Not Modified");
    map.insert(305, "Use Proxy");
    map.insert(307, "Temporary Redirect");
    map.insert(308, "Permanent Redirect");
    map.insert(400, "Bad Request");
    map.insert(401, "Unauthorized");
    map.insert(402, "Payment Required");
    map.insert(403, "Forbidden");
    map.insert(404, "Not Found");
    map.insert(405, "Method Not Allowed");
    map.insert(406, "Not Acceptable");
    map.insert(407, "Proxy Authentication Required");
    map.insert(408, "Request Timeout");
    map.insert(409, "Conflict");
    map.insert(410, "Gone");
    map.insert(411, "Length Required");
    map.insert(412, "Precondition Failed");
    map.insert(413, "Payload Too Large");
    map.insert(414, "URI Too Long");
    map.insert(415, "Unsupported Media Type");
    map.insert(416, "Range Not Satisfiable");
    map.insert(417, "Expectation Failed");
    map.insert(418, "I'm a teapot");
    map.insert(421, "Misdirected Request");
    map.insert(422, "Unprocessable Content");
    map.insert(423, "Locked");
    map.insert(424, "Failed Dependency");
    map.insert(425, "Too Early");
    map.insert(426, "Upgrade Required");
    map.insert(428, "Precondition Required");
    map.insert(429, "Too Many Requests");
    map.insert(431, "Request Header Fields Too Large");
    map.insert(451, "Unavailable For Legal Reasons");
    map.insert(500, "Internal Server Error");
    map.insert(501, "Not Implemented");
    map.insert(502, "Bad Gateway");
    map.insert(503, "Service Unavailable");
    map.insert(504, "Gateway Timeout");
    map.insert(505, "HTTP Version Not Supported");
    map.insert(506, "Variant Also Negotiates");
    map.insert(507, "Insufficient Storage");
    map.insert(508, "Loop Detected");
    map.insert(510, "Not Extended");
    map.insert(511, "Network Authentication Required");

    map
});

#[rquickjs::class]
pub struct Response<'js> {
    pub body: Option<Value<'js>>,
    method: String,
    url: String,
    start: Instant,
    pub status: u16,
    status_text: Option<String>,
    redirected: bool,
    pub headers: Class<'js, Headers>,
    content_encoding: Option<String>,
}

unsafe impl<'js> JsLifetime<'js> for Response<'js> {
    type Changed<'to> = Response<'to>;
}

impl<'js> Trace<'js> for Response<'js> {
    fn trace<'a>(&self, tracer: Tracer<'a, 'js>) {
        self.headers.trace(tracer);
        if let Some(body) = &self.body {
            body.trace(tracer);
        }
    }
}

impl<'js> Response<'js> {
    fn take_bytes(&mut self, ctx: &Ctx<'js>) -> Result<Option<Vec<u8>>> {
        let bytes = match &mut self.body {
            Some(val) => {
                if let Some(blob) = val.as_object().and_then(Class::<Blob>::from_object) {
                    let blob = blob.borrow();
                    blob.get_bytes()
                } else {
                    let bytes = ObjectBytes::from(ctx, val)?;
                    bytes.as_bytes().to_vec()
                }
            }
            None => return Ok(None),
        };

        Ok(Some(bytes))
    }
}

#[rquickjs::methods(rename_all = "camelCase")]
impl<'js> Response<'js> {
    #[qjs(constructor)]
    pub fn new(ctx: Ctx<'js>, body: Opt<Value<'js>>, options: Opt<Object<'js>>) -> Result<Self> {
        let mut url = String::from("");
        let mut status = 200;
        let mut headers = None;
        let mut status_text = None;

        if let Some(opt) = options.0 {
            if let Some(url_opt) = opt.get("url")? {
                url = url_opt;
            }
            if let Some(status_opt) = opt.get("status")? {
                status = status_opt;
            }
            if let Some(headers_opt) = opt.get("headers")? {
                headers = Some(Headers::from_value(&ctx, headers_opt)?);
            }
            if let Some(status_text_opt) = opt.get("statusText")? {
                status_text = Some(status_text_opt);
            }
        }

        let headers = Class::instance(ctx.clone(), headers.unwrap_or_default())?;
        let content_encoding = headers.get("content-encoding")?;

        let body = body.0.and_then(|body| {
            if body.is_null() || body.is_undefined() {
                None
            } else {
                Some(body)
            }
        });

        Ok(Self {
            body,
            method: "GET".into(),
            url,
            start: Instant::now(),
            status,
            status_text,
            redirected: false,
            headers,
            content_encoding,
        })
    }

    #[qjs(get)]
    pub fn status(&self) -> u64 {
        self.status.into()
    }

    #[qjs(get)]
    pub fn url(&self) -> String {
        self.url.clone()
    }

    #[qjs(get)]
    pub fn ok(&self) -> bool {
        self.status > 199 && self.status < 300
    }

    #[qjs(get)]
    pub fn redirected(&self) -> bool {
        self.redirected
    }

    //FIXME return readable stream when implemented
    #[qjs(get)]
    pub fn body(&self) -> Null {
        Null
    }

    #[qjs(get)]
    fn headers(&self) -> Class<'js, Headers> {
        self.headers.clone()
    }

    #[qjs(get, rename = "type")]
    fn response_type(&self) -> &'js str {
        match &self.status {
            0 => "error",
            _ => "basic",
        }
    }

    #[qjs(get)]
    fn status_text(&self) -> String {
        if let Some(text) = &self.status_text {
            return text.to_string();
        }
        STATUS_TEXTS.get(&self.status).unwrap_or(&"").to_string()
    }

    #[qjs(get)]
    fn body_used(&self) -> bool {
        false
    }

    pub(crate) fn text(&mut self, ctx: Ctx<'js>) -> Result<String> {
        if let Some(bytes) = self.take_bytes(&ctx)? {
            return Ok(String::from_utf8_lossy(&bytes).to_string());
        }
        Ok("".into())
    }

    pub(crate) fn json(&mut self, ctx: Ctx<'js>) -> Result<Value<'js>> {
        if let Some(bytes) = self.take_bytes(&ctx)? {
            return json_parse(&ctx, bytes);
        }
        Err(Exception::throw_syntax(&ctx, "JSON input is empty"))
    }

    fn array_buffer(&mut self, ctx: Ctx<'js>) -> Result<ArrayBuffer<'js>> {
        if let Some(bytes) = self.take_bytes(&ctx)? {
            return ArrayBuffer::new(ctx, bytes);
        }
        ArrayBuffer::new(ctx, Vec::<u8>::new())
    }

    fn bytes(&mut self, ctx: Ctx<'js>) -> Result<Value<'js>> {
        if let Some(bytes) = self.take_bytes(&ctx)? {
            return TypedArray::new(ctx, bytes).map(|m| m.into_value());
        }
        TypedArray::new(ctx, Vec::<u8>::new()).map(|m| m.into_value())
    }

    fn blob(&mut self, ctx: Ctx<'js>) -> Result<Blob> {
        if let Some(bytes) = self.take_bytes(&ctx)? {
            let headers = Headers::from_value(&ctx, self.headers().as_value().clone())?;
            let mime_type = headers
                .iter()
                .find_map(|(k, v)| (k == "content-type").then(|| v.to_string()));
            return Ok(Blob::from_bytes(bytes, mime_type));
        }
        Ok(Blob::from_bytes(Vec::<u8>::new(), None))
    }

    pub(crate) fn clone(&mut self, ctx: Ctx<'js>) -> Result<Self> {
        let body = match &mut self.body {
            Some(val) => Some(val.clone()),
            None => None,
        };

        Ok(Self {
            body,
            method: self.method.clone(),
            url: self.url.clone(),
            start: self.start,
            status: self.status,
            status_text: self.status_text.clone(),
            redirected: self.redirected,
            headers: Class::<Headers>::instance(ctx, self.headers.borrow().clone())?,
            content_encoding: self.content_encoding.clone(),
        })
    }

    #[qjs(static)]
    fn error(ctx: Ctx<'js>) -> Result<Self> {
        Ok(Self {
            body: None,
            method: "".into(),
            url: "".into(),
            start: Instant::now(),
            status: 0,
            status_text: None,
            redirected: false,
            headers: Class::instance(ctx.clone(), Headers::default())?,
            content_encoding: None,
        })
    }
    #[qjs(static, rename = "text")]
    fn text_static(ctx: Ctx<'js>, body: Value<'js>, options: Opt<Object<'js>>) -> Result<Self> {
        Self::out_static(ctx, body, options, "text/plain".to_string())
    }

    #[qjs(static, rename = "json")]
    fn json_static(ctx: Ctx<'js>, body: Value<'js>, options: Opt<Object<'js>>) -> Result<Self> {
        Self::out_static(ctx, body, options, "application/json".to_string())
    }

    #[qjs(static, rename = "bad_request")]
    fn bad_request_static(ctx: Ctx<'js>) -> Result<Self> {
        let headers = Headers::default();
        let headers = Class::instance(ctx.clone(), headers)?;

        Ok(Self {
            body: None,
            method: "".into(),
            url: "".into(),
            start: Instant::now(),
            status: 400,
            status_text: Some("Bad Request".to_string()),
            redirected: false,
            headers,
            content_encoding: None,
        })
    }

    #[qjs(static, rename = "forbidden")]
    fn forbidden_static(ctx: Ctx<'js>) -> Result<Self> {
        let headers = Headers::default();
        let headers = Class::instance(ctx.clone(), headers)?;

        Ok(Self {
            body: None,
            method: "".into(),
            url: "".into(),
            start: Instant::now(),
            status: 403,
            status_text: Some("Forbidden".to_string()),
            redirected: false,
            headers,
            content_encoding: None,
        })
    }

    #[qjs(static, rename = "not_found")]
    fn not_found_static(ctx: Ctx<'js>) -> Result<Self> {
        let headers = Headers::default();
        let headers = Class::instance(ctx.clone(), headers)?;

        Ok(Self {
            body: None,
            method: "".into(),
            url: "".into(),
            start: Instant::now(),
            status: 404,
            status_text: Some("Not Found".to_string()),
            redirected: false,
            headers,
            content_encoding: None,
        })
    }

    #[qjs(static, rename = "internal_server_error")]
    fn internal_server_error_static(ctx: Ctx<'js>, error: Opt<String>) -> Result<Self> {
        let headers = Headers::default();

        let body = if let Some(error) = error.0 {
            Some(error)
        } else {
            None
        };

        let headers = Class::instance(ctx.clone(), headers)?;

        let body = body.into_js(&ctx)?;

        Ok(Self {
            body: Some(body),
            method: "".into(),
            url: "".into(),
            start: Instant::now(),
            status: 500,
            status_text: Some("Internal Server Error".to_string()),
            redirected: false,
            headers,
            content_encoding: None,
        })
    }

    #[qjs(static)]
    fn redirect(
        ctx: Ctx<'js>,
        url: Either<URL<'js>, Coerced<String>>,
        status: Opt<u16>,
    ) -> Result<Self> {
        let status = status.0.unwrap_or(302_u16);
        let url = match url {
            Either::Left(url) => url.to_string(),
            Either::Right(url) => url.0,
        };

        let mut header = BTreeMap::new();
        header.insert("location".to_string(), Coerced(url));
        let headers = Headers::from_map(header);
        let headers = Class::instance(ctx.clone(), headers)?;

        Ok(Self {
            body: None,
            method: "".into(),
            url: "".into(),
            start: Instant::now(),
            status,
            status_text: None,
            redirected: false,
            headers,
            content_encoding: None,
        })
    }

    fn out_static(
        ctx: Ctx<'js>,
        body: Value<'js>,
        options: Opt<Object<'js>>,
        default_content_type: String,
    ) -> Result<Self> {
        let mut status = 200;
        let mut headers = None;
        let mut status_text = None;

        if let Some(opt) = options.0 {
            if let Some(status_opt) = opt.get("status")? {
                status = status_opt;
            }
            if let Some(status_text_opt) = opt.get("statusText")? {
                status_text = Some(status_text_opt);
            }
            if let Some(headers_opt) = opt.get("headers")? {
                headers = Some(Headers::from_value(&ctx, headers_opt)?);
            }
        }

        let headers = Class::instance(ctx.clone(), headers.unwrap_or_default())?;

        if !headers.contains_key("content-type").unwrap_or(false) {
            headers.set("content-type".to_string(), default_content_type)?;
        }

        let content_encoding = headers.get("content-encoding")?;

        let body = Some(body);

        Ok(Self {
            body,
            method: "".into(),
            url: "".into(),
            start: Instant::now(),
            status,
            status_text,
            redirected: false,
            headers,
            content_encoding,
        })
    }
}
