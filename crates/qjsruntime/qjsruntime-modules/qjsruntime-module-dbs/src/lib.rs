use rquickjs::{
    module::{Declarations, Exports, ModuleDef},
    prelude::Opt,
    Class, Ctx, Exception, IntoJs, Result, Value,
};

use llrt_utils::module::{export_default, ModuleInfo};

use wasmruntime_comp_sdk::dbs;

#[rquickjs::class]
#[derive(rquickjs::class::Trace, rquickjs::JsLifetime)]
pub struct Connection {
    #[qjs(skip_trace)]
    connection_inner: dbs::Connection,
}

#[rquickjs::methods]
impl<'js> Connection {
    #[qjs(constructor)]
    fn new(ctx: Ctx<'js>, database: String, shared: Opt<bool>) -> Result<Self> {
        let shared = {
            if let Some(shared) = shared.0 {
                shared
            } else {
                false
            }
        };

        let connection = dbs::Connection::open(database.as_str(), shared)
            .map_err(|_| Exception::throw_type(&ctx, "Failed to init connection"))?;

        Ok(Self {
            connection_inner: connection,
        })
    }

    fn create_sync(
        &self,
        ctx: Ctx<'js>,
        table: String,
        value: Value<'js>,
        data: Value<'js>,
    ) -> Result<Value<'js>> {
        let uid = {
            if let Some(str) = value.as_string() {
                if let Ok(str) = str.to_string() {
                    Some(str)
                } else {
                    None
                }
            } else {
                None
            }
        };

        let data = llrt_json::stringify::json_stringify(&ctx, data)
            .map_err(|_| Exception::throw_type(&ctx, "Bad parameter"))?;

        if data.is_none() {
            return Err(Exception::throw_type(&ctx, "Bad parameter"));
        }

        let val = self
            .connection_inner
            .create(table.as_str(), uid.as_deref(), &data.unwrap().into_bytes())
            .map_err(|e| Exception::throw_type(&ctx, &format!("Failed to create: {:?}", e)))?;

        val.into_js(&ctx)
    }

    fn update_sync(
        &self,
        ctx: Ctx<'js>,
        table: String,
        uid: String,
        data: Value<'js>,
    ) -> Result<Value<'js>> {
        let data = llrt_json::stringify::json_stringify(&ctx, data)
            .map_err(|_| Exception::throw_type(&ctx, "Bad parameter"))?;

        if data.is_none() {
            return Err(Exception::throw_type(&ctx, "Bad parameter"));
        }

        let val = self
            .connection_inner
            .update(table.as_str(), uid.as_str(), &data.unwrap().into_bytes())
            .map_err(|e| Exception::throw_type(&ctx, &format!("Failed to update: {:?}", e)))?;

        val.into_js(&ctx)
    }

    fn delete_sync(&self, ctx: Ctx<'js>, table: String, uid: String) -> Result<Value<'js>> {
        let val = self
            .connection_inner
            .delete(table.as_str(), uid.as_str())
            .map_err(|e| Exception::throw_type(&ctx, &format!("Failed to delete: {:?}", e)))?;

        val.into_js(&ctx)
    }

    fn select_all_sync(&self, ctx: Ctx<'js>, table: String) -> Result<Value<'js>> {
        let val = self
            .connection_inner
            .select_all(table.as_str())
            .map_err(|e| Exception::throw_type(&ctx, &format!("Failed to select all: {:?}", e)))?;

        val.into_js(&ctx)
    }

    fn select_sync(&self, ctx: Ctx<'js>, table: String, uid: String) -> Result<Value<'js>> {
        let val = self
            .connection_inner
            .select(table.as_str(), uid.as_str())
            .map_err(|e| Exception::throw_type(&ctx, &format!("Failed to select: {:?}", e)))?;

        val.into_js(&ctx)
    }

    fn query_sync(
        &self,
        ctx: Ctx<'js>,
        sql: String,
        params: Opt<Value<'js>>,
    ) -> Result<Value<'js>> {
        let params = if let Some(parms) = params.0 {
            let params = convert_params(&ctx, parms)?;

            params
        } else {
            None
        };

        let val = self
            .connection_inner
            .query(sql.as_str(), params.as_ref())
            .map_err(|e| Exception::throw_type(&ctx, &format!("Failed to query: {:?}", e)))?;

        val.into_js(&ctx)
    }
}

pub struct DbsModule;

impl ModuleDef for DbsModule {
    fn declare(declare: &Declarations) -> Result<()> {
        declare.declare("Connection")?;

        declare.declare("default")?;
        Ok(())
    }

    fn evaluate<'js>(ctx: &Ctx<'js>, exports: &Exports<'js>) -> Result<()> {
        export_default(ctx, exports, |default| {
            Class::<Connection>::define(default)?;

            Ok(())
        })
    }
}

impl From<DbsModule> for ModuleInfo<DbsModule> {
    fn from(val: DbsModule) -> Self {
        ModuleInfo {
            name: "dbs",
            module: val,
        }
    }
}

fn convert_params<'js>(ctx: &Ctx<'js>, value: Value<'js>) -> Result<Option<Vec<u8>>> {
    if value.is_null() || value.is_undefined() {
        return Ok(None);
    }

    if !value.is_object() {
        return Err(Exception::throw_type(&ctx, "Bad parameter"));
    }

    let data = llrt_json::stringify::json_stringify(&ctx, value)
        .map_err(|_| Exception::throw_type(&ctx, "Bad parameter"))?;

    if let Some(data) = data {
        Ok(Some(data.into_bytes()))
    } else {
        Ok(None)
    }
}
