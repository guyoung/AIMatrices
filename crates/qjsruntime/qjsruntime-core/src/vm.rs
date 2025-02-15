use std::cmp::min;
use std::env;
use std::fmt::Write;
use std::process::exit;
use std::result::Result as StdResult;

use bytes::Bytes;
use ring::rand::SecureRandom;

use rquickjs::{
    atom::PredefinedAtom,
    context::EvalOptions,
    function::Opt,
    loader::FileResolver,
    prelude::{Func, Rest},
    CatchResultExt, CaughtError, Context, Ctx, Error, Function, IntoJs, Object, Result,
    Runtime, Value,
};

use llrt_json::{parse::json_parse_string, stringify::json_stringify_replacer_space};
use llrt_numbers::number_to_string;
use llrt_utils::{
    error::ErrorExtensions,
    object::ObjectExt,
    primordials::{BasePrimordials, Primordial},
};



use crate::module_loader::loader::CustomLoader;
use crate::module_loader::require;
use crate::module_loader::resolver::CustomResolver;
use crate::modules::console;
use crate::modules::crypto::SYSTEM_RANDOM;
use crate::utils::clone::structured_clone;

//vm
pub const ENV_QJSRUNTIME_GC_THRESHOLD_MB: &str = "LLRT_QJSRUNTIME_THRESHOLD_MB";

fn print(value: String, stdout: Opt<bool>) {
    if stdout.0.unwrap_or_default() {
        println!("{value}");
    } else {
        eprintln!("{value}")
    }
}

pub struct Vm {
    pub runtime: Runtime,
    pub ctx: Context,
}

#[allow(dead_code)]
struct ExportArgs<'js>(Ctx<'js>, Object<'js>, Value<'js>, Value<'js>);

pub struct VmOptions {
    pub module_builder: crate::module_builder::ModuleBuilder,
    pub max_stack_size: usize,
    pub gc_threshold_mb: usize,
}

impl Default for VmOptions {
    fn default() -> Self {
        Self {
            module_builder: crate::module_builder::ModuleBuilder::default(),
            max_stack_size: 512 * 1024,
            gc_threshold_mb: {
                const DEFAULT_GC_THRESHOLD_MB: usize = 20;

                let gc_threshold_mb: usize = env::var(ENV_QJSRUNTIME_GC_THRESHOLD_MB)
                    .map(|threshold| threshold.parse().unwrap_or(DEFAULT_GC_THRESHOLD_MB))
                    .unwrap_or(DEFAULT_GC_THRESHOLD_MB);

                gc_threshold_mb * 1024 * 1024
            },
        }
    }
}

impl Vm {
    pub fn from_options(
        vm_options: VmOptions,
    ) -> StdResult<Self, Box<dyn std::error::Error + Send + Sync>> {
        qjsruntime_module_locator::time::init();
        //http::init()?;
        //security::init()?;

        SYSTEM_RANDOM
            .fill(&mut [0; 8])
            .expect("Failed to initialize SystemRandom");

        let mut file_resolver = FileResolver::default();
        let mut paths: Vec<&str> = Vec::with_capacity(10);

        paths.push(".");

        for path in paths.iter() {
            file_resolver.add_path(*path);
        }

        let (builtin_resolver, module_loader, module_names, init_globals) =
            vm_options.module_builder.build();

        let resolver = (builtin_resolver, CustomResolver, file_resolver);

        let loader = (module_loader, CustomLoader);

        let runtime = Runtime::new()?;
        runtime.set_max_stack_size(vm_options.max_stack_size);
        runtime.set_gc_threshold(vm_options.gc_threshold_mb);
        runtime.set_loader(resolver, loader);

        let ctx = Context::full(&runtime)?;
        ctx.with(|ctx| {
            (|| {
                for init_global in init_globals {
                    init_global(&ctx)?;
                }
                init(&ctx)?;
                require::init(&ctx, module_names)?;
                Ok(())
            })()
            .catch(&ctx)
            .unwrap_or_else(|err| Self::print_error_and_exit(&ctx, err));
            Ok::<_, Error>(())
        })?;

        Ok(Vm { runtime, ctx })
    }

    pub fn new() -> StdResult<Self, Box<dyn std::error::Error + Send + Sync>> {
        let vm = Self::from_options(VmOptions::default())?;
        Ok(vm)
    }

    pub fn run_with<F>(&self, f: F)
    where
        F: for<'js> FnOnce(&Ctx<'js>) -> Result<()> + std::marker::Send,
    {
        self.ctx.with(|ctx| {
            if let Err(err) = f(&ctx).catch(&ctx) {
                Self::print_error_and_exit(&ctx, err);
            }
        });
    }

    pub fn run<S: Into<Vec<u8>> + Send>(&self, source: S, strict: bool, global: bool) {
        self.run_with(|ctx| {
            let mut options = EvalOptions::default();
            options.strict = strict;
            options.promise = true;
            options.global = global;
            let _ = ctx.eval_with_options::<Value, _>(source, options)?;
            Ok::<_, Error>(())
        })
    }

    pub fn http_request_run_with<F>(
        &self,
        f: F,
    ) -> StdResult<http::Response<Bytes>, Box<dyn std::error::Error + Sync + Send>>
    where
        F: for<'js> FnOnce(&Ctx<'js>) -> Result<http::Response<Bytes>> + std::marker::Send,
    {
        self.ctx.with(|ctx| {
            let res = f(&ctx).catch(&ctx);

            match res {
                Ok(res) => Ok(res),
                Err(err) => Self::print_error_and_exit(&ctx, err),
            }
        })
    }

    pub fn http_request_run<S: Into<Vec<u8>> + Send>(
        &self,
        request: http::Request<Bytes>,
        source: S,
        strict: bool,
    ) -> StdResult<http::Response<Bytes>, Box<dyn std::error::Error + Sync + Send>> {
        self.http_request_run_with(|ctx| {
            let mut options = EvalOptions::default();
            options.strict = strict;
            options.promise = true;
            options.global = true;

            let _ = ctx.eval_with_options::<Value, _>(source, options)?;
            let globals = ctx.globals();
            let func: Function = globals.get("main")?;

            let req = qjsruntime_module_http::request::from_http_request(&ctx, request)?;

            let res: Value = func.call((req,))?;

            let res = qjsruntime_module_http::convert::convert_js_value_to_http_response(&ctx, res)?;

            Ok(res)
        })
    }

    pub fn print_error_and_exit<'js>(ctx: &Ctx<'js>, err: CaughtError<'js>) -> ! {
        let mut error_str = String::new();
        write!(error_str, "Error: {:?}", err).unwrap();
        if let Ok(error) = err.into_value(ctx) {
            if console::log_std_err(ctx, Rest(vec![error.clone()]), console::LogLevel::Fatal)
                .is_err()
            {
                eprintln!("{}", error_str);
            };
            if cfg!(test) {
                panic!("{:?}", error);
            } else {
                exit(1)
            }
        } else if cfg!(test) {
            panic!("{}", error_str);
        } else {
            eprintln!("{}", error_str);
            exit(1)
        };
    }

    pub fn idle(self) -> StdResult<(), Box<dyn std::error::Error + Sync + Send>> {
        drop(self.ctx);
        drop(self.runtime);
        Ok(())
    }
}

fn init(ctx: &Ctx<'_>) -> Result<()> {
    let globals = ctx.globals();

    globals.set("__gc", Func::from(|ctx: Ctx| ctx.run_gc()))?;

    let number: Function = globals.get(PredefinedAtom::Number)?;
    let number_proto: Object = number.get(PredefinedAtom::Prototype)?;
    number_proto.set(PredefinedAtom::ToString, Func::from(number_to_string))?;

    let readable_stream_stub = ctx.eval::<Value,_>(
        r#"class ReadableStream{constructor(){throw Error(`ReadableStream is not supported via global scope. Enable this by adding this to your code:\nimport { ReadableStream } from "stream";\nglobalThis.ReadableStream = ReadableStream;`)}};"#
    )?;

    globals.set("ReadableStream", readable_stream_stub)?;
    globals.set("global", ctx.globals())?;
    globals.set("self", ctx.globals())?;
    globals.set("load", Func::from(load))?;
    globals.set("print", Func::from(print))?;
    globals.set(
        "structuredClone",
        Func::from(|ctx, value, options| structured_clone(&ctx, value, options)),
    )?;

    let json_module: Object = globals.get(PredefinedAtom::JSON)?;
    json_module.set("parse", Func::from(json_parse_string))?;
    json_module.set(
        "stringify",
        Func::from(|ctx, value, replacer, space| {
            struct StringifyArgs<'js>(Ctx<'js>, Value<'js>, Opt<Value<'js>>, Opt<Value<'js>>);
            let StringifyArgs(ctx, value, replacer, space) =
                StringifyArgs(ctx, value, replacer, space);

            let mut space_value = None;
            let mut replacer_value = None;

            if let Some(replacer) = replacer.0 {
                if let Some(space) = space.0 {
                    if let Some(space) = space.as_string() {
                        let mut space = space.clone().to_string()?;
                        space.truncate(20);
                        space_value = Some(space);
                    }
                    if let Some(number) = space.as_int() {
                        if number > 0 {
                            space_value = Some(" ".repeat(min(10, number as usize)));
                        }
                    }
                }
                replacer_value = Some(replacer);
            }

            json_stringify_replacer_space(&ctx, value, replacer_value, space_value)
                .map(|v| v.into_js(&ctx))?
        }),
    )?;

    //init base primordials
    let _ = BasePrimordials::get(ctx)?;

    Ok(())
}

fn load<'js>(ctx: Ctx<'js>, filename: String, options: Opt<Object<'js>>) -> Result<Value<'js>> {
    let mut eval_options = EvalOptions::default();
    eval_options.strict = false;
    eval_options.promise = true;

    if let Some(options) = options.0 {
        if let Some(global) = options.get_optional("global")? {
            eval_options.global = global;
        }

        if let Some(strict) = options.get_optional("strict")? {
            eval_options.strict = strict;
        }
    }

    ctx.eval_file_with_options(filename, eval_options)
}





