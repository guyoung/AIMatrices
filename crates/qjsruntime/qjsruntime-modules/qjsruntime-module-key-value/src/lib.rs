use rquickjs::{
    module::{Declarations, Exports, ModuleDef},
    Class, Ctx, Exception, IntoJs, Result, Value,
};

use llrt_utils::{
    bytes::ObjectBytes,
    module::{export_default, ModuleInfo},
};

use wasmruntime_comp_sdk::key_value;

#[rquickjs::class]
#[derive(rquickjs::class::Trace, rquickjs::JsLifetime)]
pub struct Store {
    #[qjs(skip_trace)]
    store_inner: key_value::Store,
}

#[rquickjs::methods]
impl<'js> Store {
    #[qjs(constructor)]
    fn new(ctx: Ctx<'js>) -> Result<Self> {
        let store = key_value::Store::open_default()
            .map_err(|_| Exception::throw_type(&ctx, "Failed to init store"))?;

        Ok(Self { store_inner: store })
    }

    fn get(&self, ctx: Ctx<'js>, key: String) -> Result<Value<'js>> {
        let val = self
            .store_inner
            .get(key.as_str())
            .map_err(|_| Exception::throw_type(&ctx, "Failed to get key"))?;

        val.into_js(&ctx)
    }

    fn set(&self, ctx: Ctx<'js>, key: String, val: ObjectBytes<'js>) -> Result<()> {
        self.store_inner
            .set(key.as_str(), val.as_bytes())
            .map_err(|_| Exception::throw_type(&ctx, "Failed to set key"))?;

        Ok(())
    }

    fn delete(&self, ctx: Ctx<'js>, key: String) -> Result<()> {
        self.store_inner
            .delete(key.as_str())
            .map_err(|_| Exception::throw_type(&ctx, "Failed to delete key"))?;

        Ok(())
    }

    fn exists(&self, ctx: Ctx<'js>, key: String) -> Result<bool> {
        let exists = self
            .store_inner
            .exists(key.as_str())
            .map_err(|_| Exception::throw_type(&ctx, "Failed to get key"))?;

        Ok(exists)
    }

    fn get_keys(&self, ctx: Ctx<'js>) -> Result<Value<'js>> {
        let keys = self
            .store_inner
            .get_keys()
            .map_err(|_| Exception::throw_type(&ctx, "Failed to get keys"))?;

        keys.into_js(&ctx)
    }
}

pub struct KeyValueModule;

impl ModuleDef for KeyValueModule {
    fn declare(declare: &Declarations) -> Result<()> {
        declare.declare("Store")?;

        declare.declare("default")?;
        Ok(())
    }

    fn evaluate<'js>(ctx: &Ctx<'js>, exports: &Exports<'js>) -> Result<()> {
        export_default(ctx, exports, |default| {
            Class::<Store>::define(default)?;

            Ok(())
        })
    }
}

impl From<KeyValueModule> for ModuleInfo<KeyValueModule> {
    fn from(val: KeyValueModule) -> Self {
        ModuleInfo {
            name: "key_value",
            module: val,
        }
    }
}
