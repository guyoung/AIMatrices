use rquickjs::{Ctx, Object, Result};

fn get_user_agent() -> &'static str {
    concat!("QJSRUNTIME ", env!("CARGO_PKG_VERSION"))
}

pub fn init(ctx: &Ctx<'_>) -> Result<()> {
    let globals = ctx.globals();

    let navigator = Object::new(ctx.clone())?;

    navigator.set("userAgent", get_user_agent())?;

    globals.set("navigator", navigator)?;

    Ok(())
}
