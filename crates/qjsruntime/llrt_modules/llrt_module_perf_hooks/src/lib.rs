// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
use llrt_utils::{
    module::{export_default, ModuleInfo},
    time,
};
use rquickjs::{
    atom::PredefinedAtom,
    module::{Declarations, Exports, ModuleDef},
    prelude::Func,
    Ctx, JsLifetime, Object, Result,
};

fn get_time_origin() -> f64 {
    let time_origin = time::origin_nanos() as f64;

    time_origin / 1e6
}

fn now() -> f64 {
    let now = time::now_nanos();
    let started = time::origin_nanos();
    let elapsed = now.checked_sub(started).unwrap_or_default();

    (elapsed as f64) / 1e6
}

fn to_json(ctx: Ctx<'_>) -> Result<Object<'_>> {
    let obj = Object::new(ctx.clone())?;

    obj.set("timeOrigin", get_time_origin())?;

    Ok(obj)
}

struct PerfInitedUserData;

unsafe impl JsLifetime<'_> for PerfInitedUserData {
    type Changed<'to> = PerfInitedUserData;
}

fn new_performance(ctx: Ctx<'_>) -> Result<Object<'_>> {
    let global = ctx.globals();

    let inited = ctx.userdata::<PerfInitedUserData>().is_some();
    if !inited {
        ctx.store_userdata(PerfInitedUserData)?;
        let performance = Object::new(ctx)?;
        performance.set("timeOrigin", get_time_origin())?;
        performance.set("now", Func::from(now))?;
        performance.set(PredefinedAtom::ToJSON, Func::from(to_json))?;
        global.set("performance", performance.clone())?;
        return Ok(performance);
    }
    global.get("performance")
}

pub struct PerfHooksModule;

impl ModuleDef for PerfHooksModule {
    fn declare(declare: &Declarations<'_>) -> Result<()> {
        declare.declare("performance")?;
        declare.declare("default")?;
        Ok(())
    }

    fn evaluate<'js>(ctx: &Ctx<'js>, exports: &Exports<'js>) -> Result<()> {
        export_default(ctx, exports, |default| {
            let performance = new_performance(ctx.clone())?;
            default.set("performance", performance)?;
            Ok(())
        })
    }
}

impl From<PerfHooksModule> for ModuleInfo<PerfHooksModule> {
    fn from(val: PerfHooksModule) -> Self {
        ModuleInfo {
            name: "perf_hooks",
            module: val,
        }
    }
}

pub fn init(ctx: &Ctx<'_>) -> Result<()> {
    new_performance(ctx.clone())?;
    Ok(())
}
