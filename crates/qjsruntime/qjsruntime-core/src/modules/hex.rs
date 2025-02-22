pub mod encoder {
    pub use llrt_encoding::*;
}

use rquickjs::{
    module::{Declarations, Exports, ModuleDef},
    prelude::Func,
    Ctx, Result, Value,
};

use llrt_utils::bytes::{bytes_to_typed_array, ObjectBytes};

use crate::{
    module_builder::ModuleInfo, modules::module::export_default, utils::result::ResultExt,
};

use self::encoder::{bytes_from_hex, bytes_to_hex_string};

pub struct HexModule;

impl HexModule {
    pub fn encode(bytes: ObjectBytes<'_>) -> Result<String> {
        Ok(bytes_to_hex_string(bytes.as_bytes()))
    }

    pub fn decode(ctx: Ctx, encoded: String) -> Result<Value> {
        let bytes = bytes_from_hex(encoded.as_bytes())
            .or_throw_msg(&ctx, "Cannot decode unrecognized sequence")?;

        bytes_to_typed_array(ctx, &bytes)
    }
}

impl ModuleDef for HexModule {
    fn declare(declare: &Declarations) -> Result<()> {
        declare.declare(stringify!(encode))?;
        declare.declare(stringify!(decode))?;
        declare.declare("default")?;
        Ok(())
    }

    fn evaluate<'js>(ctx: &Ctx<'js>, exports: &Exports<'js>) -> Result<()> {
        export_default(ctx, exports, |default| {
            default.set(stringify!(encode), Func::from(Self::encode))?;
            default.set(stringify!(decode), Func::from(Self::decode))?;
            Ok(())
        })?;

        Ok(())
    }
}

impl From<HexModule> for ModuleInfo<HexModule> {
    fn from(val: HexModule) -> Self {
        ModuleInfo {
            name: "hex",
            module: val,
        }
    }
}
