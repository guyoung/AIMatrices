// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
mod access;
mod mkdir;
mod read_dir;
mod read_file;
mod rm;
mod stats;
mod write_file;

use llrt_utils::module::{export_default, ModuleInfo};
use rquickjs::{
    module::{Declarations, Exports, ModuleDef},
    prelude::Func,
};
use rquickjs::{Class, Ctx, Object, Result};

use self::access::access_sync;

use self::mkdir::{mkdir_sync, mkdtemp_sync};
use self::read_dir::{read_dir_sync, Dirent};
use self::read_file::read_file_sync;
use self::rm::{rmdir_sync, rmfile_sync};
use self::stats::{stat_fn_sync, Stats};
use self::write_file::write_file_sync;

pub const CONSTANT_F_OK: u32 = 0;
pub const CONSTANT_R_OK: u32 = 4;
pub const CONSTANT_W_OK: u32 = 2;
pub const CONSTANT_X_OK: u32 = 1;



pub struct FsModule;

impl ModuleDef for FsModule {
    fn declare(declare: &Declarations) -> Result<()> {
        declare.declare("accessSync")?;
        declare.declare("mkdirSync")?;
        declare.declare("mkdtempSync")?;
        declare.declare("readdirSync")?;
        declare.declare("readFileSync")?;
        declare.declare("rmdirSync")?;
        declare.declare("rmSync")?;
        declare.declare("statSync")?;
        declare.declare("writeFileSync")?;
        declare.declare("constants")?;

        declare.declare("default")?;

        Ok(())
    }

    fn evaluate<'js>(ctx: &Ctx<'js>, exports: &Exports<'js>) -> Result<()> {
        let globals = ctx.globals();

        Class::<Dirent>::define(&globals)?;
        Class::<Stats>::define(&globals)?;

        export_default(ctx, exports, |default| {
            export_constants(ctx, default)?;

            default.set("accessSync", Func::from(access_sync))?;
            default.set("mkdirSync", Func::from(mkdir_sync))?;
            default.set("mkdtempSync", Func::from(mkdtemp_sync))?;
            default.set("readdirSync", Func::from(read_dir_sync))?;
            default.set("readFileSync", Func::from(read_file_sync))?;
            default.set("rmdirSync", Func::from(rmdir_sync))?;
            default.set("rmSync", Func::from(rmfile_sync))?;
            default.set("statSync", Func::from(stat_fn_sync))?;
            default.set("writeFileSync", Func::from(write_file_sync))?;

            Ok(())
        })
    }
}


fn export_constants<'js>(ctx: &Ctx<'js>, exports: &Object<'js>) -> Result<()> {
    let constants = Object::new(ctx.clone())?;
    constants.set("F_OK", CONSTANT_F_OK)?;
    constants.set("R_OK", CONSTANT_R_OK)?;
    constants.set("W_OK", CONSTANT_W_OK)?;
    constants.set("X_OK", CONSTANT_X_OK)?;

    exports.set("constants", constants)?;

    Ok(())
}

impl From<FsModule> for ModuleInfo<FsModule> {
    fn from(val: FsModule) -> Self {
        ModuleInfo {
            name: "fs",
            module: val,
        }
    }
}
