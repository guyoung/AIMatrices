/***

use rquickjs::{
    module::{Declarations, Exports, ModuleDef},
    prelude::Opt,
    Class, Ctx, Exception, IntoJs, Result, Value,
};

use llrt_utils::module::{export_default, ModuleInfo};

 ***/

pub mod ollama_rs;
mod http_client;
mod error;
/***
#[rquickjs::class]
#[derive(rquickjs::class::Trace, rquickjs::JsLifetime)]
pub struct Client {
    #[qjs(skip_trace)]
    client_inner: v1::api::OpenAIClient,
}

#[rquickjs::methods]
impl<'js> Client {
    #[qjs(constructor)]
    fn new(ctx: Ctx<'js>, api_endpoint: String, api_key: Opt<String>) -> Result<Self> {
        let api_key = {
            if let Some(api_key) = api_key.0 {
                Some(api_key)
            } else {
                None
            }
        };

        let client = v1::api::OpenAIClient::builder(api_endpoint, api_key)
            .build()
            .map_err(|_| Exception::throw_type(&ctx, "Failed to create client"))?;

        Ok(Self {
            client_inner: client,
        })
    }

    fn chat_completion(&self, ctx: Ctx<'js>, req: Value<'js>) -> Result<Value<'js>> {
        let req = llrt_json::stringify::json_stringify(&ctx, req)
            .map_err(|_| Exception::throw_type(&ctx, "Bad request parameter"))?;

        if req.is_none() {
            return Err(Exception::throw_type(&ctx, "Bad request parameter"));
        }

        let req = serde_json::from_str::<v1::chat_completion::ChatCompletionRequest>(&req.unwrap())
            .map_err(|_| Exception::throw_type(&ctx, "Bad request parameter"))?;

        let res = self
            .client_inner
            .chat_completion(req)
            .map_err(|e| Exception::throw_type(&ctx, &format!("Failed to request: {:?}", e)))?;

        let res =
            serde_json::to_vec(&res).map_err(|_| Exception::throw_type(&ctx, "Bad response"))?;

        res.into_js(&ctx)
    }
}

pub struct OpenAiModule;

impl ModuleDef for OpenAiModule {
    fn declare(declare: &Declarations) -> Result<()> {
        declare.declare("Client")?;

        declare.declare("default")?;
        Ok(())
    }

    fn evaluate<'js>(ctx: &Ctx<'js>, exports: &Exports<'js>) -> Result<()> {
        export_default(ctx, exports, |default| {
            Class::<Client>::define(default)?;

            Ok(())
        })
    }
}

impl From<OpenAiModule> for ModuleInfo<OpenAiModule> {
    fn from(val: OpenAiModule) -> Self {
        ModuleInfo {
            name: "openai",
            module: val,
        }
    }
}
***/