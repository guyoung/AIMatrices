mod blob;
mod file;

pub mod headers;
pub mod request;

pub mod response;

pub mod convert;

mod fetch;

use rquickjs::{
    function::Func,
    module::{Declarations, Exports, ModuleDef},
    Class, Ctx, Result,
};

use llrt_utils::module::{export_default, ModuleInfo};

pub struct HttpModule;

impl ModuleDef for HttpModule {
    fn declare(declare: &Declarations) -> Result<()> {
        declare.declare("Request")?;
        declare.declare("Response")?;

        declare.declare("fetch_sync")?;

        declare.declare("default")?;
        Ok(())
    }

    fn evaluate<'js>(ctx: &Ctx<'js>, exports: &Exports<'js>) -> Result<()> {
        export_default(ctx, exports, |default| {
            Class::<request::Request>::define(default)?;
            Class::<response::Response>::define(default)?;

            default.set("fetch_sync", Func::from(fetch::fetch_sync))?;

            Ok(())
        })
    }
}

impl From<HttpModule> for ModuleInfo<HttpModule> {
    fn from(val: HttpModule) -> Self {
        ModuleInfo {
            name: "http",
            module: val,
        }
    }
}
