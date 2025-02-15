use rquickjs::{
    function::Func,
    module::{Declarations, Exports, ModuleDef},
    Ctx, Exception, Result,
};


use llrt_utils::{
    module::{export_default, ModuleInfo},
};

use wasmruntime_comp_sdk::variables;



fn get<'js>(ctx: Ctx<'js>, name: String) -> Result<String> {
    let val = variables::get(name.as_str())
        .map_err(|_| Exception::throw_type(&ctx, "Failed to acquire variable"))?;

    Ok(val)
}

pub struct VariablesModule;

impl ModuleDef for VariablesModule {
    fn declare(declare: &Declarations) -> Result<()> {
        declare.declare("get")?;

        declare.declare("default")?;
        Ok(())
    }

    fn evaluate<'js>(ctx: &Ctx<'js>, exports: &Exports<'js>) -> Result<()> {
        export_default(ctx, exports, |default| {
            default.set("get", Func::from(get))?;

            Ok(())
        })
    }
}

impl From<VariablesModule> for ModuleInfo<VariablesModule> {
    fn from(val: VariablesModule) -> Self {
        ModuleInfo {
            name: "variables",
            module: val,
        }
    }
}
