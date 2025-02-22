use std::io::Read;

use llrt_buffer::Buffer;
use llrt_utils::{
    bytes::ObjectBytes,
    module::{export_default, ModuleInfo},
    object::ObjectExt,
};
use rquickjs::function::Func;
use rquickjs::{
    module::{Declarations, Exports, ModuleDef},
    prelude::Opt,
    Ctx, IntoJs, Result, Value,
};

macro_rules! define_sync_function {
    ($fn_name:ident, $converter:expr, $command:expr) => {
        pub fn $fn_name<'js>(
            ctx: Ctx<'js>,
            value: ObjectBytes<'js>,
            options: Opt<Value<'js>>,
        ) -> Result<Value<'js>> {
            $converter(ctx.clone(), value, options, $command)
        }
    };
}

enum ZlibCommand {
    Deflate,
    DeflateRaw,
    Gzip,
    Inflate,
    InflateRaw,
    Gunzip,
}

fn zlib_converter<'js>(
    ctx: Ctx<'js>,
    bytes: ObjectBytes<'js>,
    options: Opt<Value<'js>>,
    command: ZlibCommand,
) -> Result<Value<'js>> {
    let src = bytes.as_bytes();

    let mut level = qjsruntime_compression::zlib::Compression::default();
    if let Some(options) = options.0 {
        if let Some(opt) = options.get_optional("level")? {
            level = qjsruntime_compression::zlib::Compression::new(opt);
        }
    }

    let mut dst: Vec<u8> = Vec::with_capacity(src.len());

    let _ = match command {
        ZlibCommand::Deflate => {
            qjsruntime_compression::zlib::encoder(src, level).read_to_end(&mut dst)?
        }
        ZlibCommand::DeflateRaw => {
            qjsruntime_compression::deflate::encoder(src, level).read_to_end(&mut dst)?
        }
        ZlibCommand::Gzip => {
            qjsruntime_compression::gz::encoder(src, level).read_to_end(&mut dst)?
        }
        ZlibCommand::Inflate => qjsruntime_compression::zlib::decoder(src).read_to_end(&mut dst)?,
        ZlibCommand::InflateRaw => {
            qjsruntime_compression::deflate::decoder(src).read_to_end(&mut dst)?
        }
        ZlibCommand::Gunzip => qjsruntime_compression::gz::decoder(src).read_to_end(&mut dst)?,
    };

    Buffer(dst).into_js(&ctx)
}

define_sync_function!(deflate_sync, zlib_converter, ZlibCommand::Deflate);

define_sync_function!(deflate_raw_sync, zlib_converter, ZlibCommand::DeflateRaw);

define_sync_function!(gzip_sync, zlib_converter, ZlibCommand::Gzip);

define_sync_function!(inflate_sync, zlib_converter, ZlibCommand::Inflate);

define_sync_function!(inflate_raw_sync, zlib_converter, ZlibCommand::InflateRaw);

define_sync_function!(gunzip_sync, zlib_converter, ZlibCommand::Gunzip);

enum BrotliCommand {
    Compress,
    Decompress,
}

fn brotli_converter<'js>(
    ctx: Ctx<'js>,
    bytes: ObjectBytes<'js>,
    _options: Opt<Value<'js>>,
    command: BrotliCommand,
) -> Result<Value<'js>> {
    let src = bytes.as_bytes();

    let mut dst: Vec<u8> = Vec::with_capacity(src.len());

    let _ = match command {
        BrotliCommand::Compress => {
            qjsruntime_compression::brotli::encoder(src).read_to_end(&mut dst)?
        }
        BrotliCommand::Decompress => {
            qjsruntime_compression::brotli::decoder(src).read_to_end(&mut dst)?
        }
    };

    Buffer(dst).into_js(&ctx)
}

define_sync_function!(br_comp_sync, brotli_converter, BrotliCommand::Compress);

define_sync_function!(br_decomp_sync, brotli_converter, BrotliCommand::Decompress);

pub struct ZlibModule;

impl ModuleDef for ZlibModule {
    fn declare(declare: &Declarations) -> Result<()> {
        declare.declare("deflateSync")?;

        declare.declare("deflateRawSync")?;

        declare.declare("gzipSync")?;

        declare.declare("inflateSync")?;

        declare.declare("inflateRawSync")?;

        declare.declare("gunzipSync")?;

        declare.declare("brotliCompressSync")?;

        declare.declare("brotliDecompressSync")?;

        declare.declare("default")?;
        Ok(())
    }

    fn evaluate<'js>(ctx: &Ctx<'js>, exports: &Exports<'js>) -> Result<()> {
        export_default(ctx, exports, |default| {
            default.set("deflateSync", Func::from(deflate_sync))?;

            default.set("deflateRawSync", Func::from(deflate_raw_sync))?;

            default.set("gzipSync", Func::from(gzip_sync))?;

            default.set("inflateSync", Func::from(inflate_sync))?;

            default.set("inflateRawSync", Func::from(inflate_raw_sync))?;

            default.set("gunzipSync", Func::from(gunzip_sync))?;

            default.set("brotliCompressSync", Func::from(br_comp_sync))?;

            default.set("brotliDecompressSync", Func::from(br_decomp_sync))?;

            Ok(())
        })
    }
}

impl From<ZlibModule> for ModuleInfo<ZlibModule> {
    fn from(val: ZlibModule) -> Self {
        ModuleInfo {
            name: "zlib",
            module: val,
        }
    }
}
