use std::collections::HashSet;

use rquickjs::{
    loader::{ModuleLoader, Resolver},
    module::ModuleDef,
    Ctx, Error, Result,
};

pub use llrt_utils::module::ModuleInfo;

#[derive(Debug, Default)]
pub struct ModuleResolver {
    modules: HashSet<String>,
}

impl ModuleResolver {
    #[must_use]
    pub fn with_module<P: Into<String>>(mut self, path: P) -> Self {
        self.modules.insert(path.into());
        self
    }
}

impl Resolver for ModuleResolver {
    fn resolve(&mut self, _: &Ctx<'_>, base: &str, name: &str) -> Result<String> {
        let name = name.trim_start_matches("node:");
        if self.modules.contains(name) {
            Ok(name.into())
        } else {
            Err(Error::new_resolving(base, name))
        }
    }
}

pub type Modules = (
    ModuleResolver,
    ModuleLoader,
    HashSet<&'static str>,
    Vec<fn(&Ctx<'_>) -> Result<()>>,
);

pub struct ModuleBuilder {
    builtin_resolver: ModuleResolver,
    module_loader: ModuleLoader,
    module_names: HashSet<&'static str>,
    init_global: Vec<fn(&Ctx<'_>) -> Result<()>>,
}

impl Default for ModuleBuilder {
    fn default() -> Self {
        Self::new()
            .with_module(crate::modules::assert::AssertModule)
            .with_module(crate::modules::buffer::BufferModule)
            .with_global(crate::modules::buffer::init)
            .with_module(crate::modules::crypto::CryptoModule)
            .with_global(crate::modules::crypto::init)
            .with_module(crate::modules::events::EventsModule)
            .with_global(crate::modules::events::init)
            .with_global(crate::modules::exceptions::init)
            .with_module(crate::modules::fs::FsModule)
            .with_module(crate::modules::path::PathModule)
            .with_module(crate::modules::perf_hooks::PerfHooksModule)
            .with_global(crate::modules::perf_hooks::init)
            .with_module(crate::modules::timers::TimersModule)
            .with_global(crate::modules::timers::init)
            .with_module(crate::modules::url::UrlModule)
            .with_global(crate::modules::url::init)
            .with_module(crate::modules::module::ModuleModule)
            .with_module(crate::modules::console::ConsoleModule)
            .with_global(crate::modules::console::init)
            .with_module(crate::modules::util::UtilModule)
            .with_global(crate::modules::util::init)
            .with_module(crate::modules::hex::HexModule)
            .with_module(crate::modules::uuid::UuidModule)
            .with_module(crate::modules::xml::XmlModule)
            .with_global(crate::modules::navigator::init)
            .with_module(crate::modules::process::ProcessModule)
            .with_global(crate::modules::process::init)
            .with_module(crate::modules::zlib::ZlibModule)
            .with_module(crate::modules::variables::VariablesModule)
            .with_module(crate::modules::key_value::KeyValueModule)
            .with_module(crate::modules::http::HttpModule)

            .with_module(crate::modules::openai::OpenAiModule)


            .with_module(crate::modules::dbs::DbsModule)
    }
}

impl ModuleBuilder {
    pub fn new() -> Self {
        Self {
            builtin_resolver: ModuleResolver::default(),
            module_loader: ModuleLoader::default(),
            module_names: HashSet::new(),
            init_global: Vec::new(),
        }
    }

    pub fn with_module<M: ModuleDef, I: Into<ModuleInfo<M>>>(mut self, module: I) -> Self {
        let module_info: ModuleInfo<M> = module.into();

        self.builtin_resolver = self.builtin_resolver.with_module(module_info.name);
        self.module_loader = self
            .module_loader
            .with_module(module_info.name, module_info.module);
        self.module_names.insert(module_info.name);

        self
    }

    pub fn with_global(mut self, init_global: fn(&Ctx<'_>) -> Result<()>) -> Self {
        self.init_global.push(init_global);
        self
    }

    pub fn build(self) -> Modules {
        (
            self.builtin_resolver,
            self.module_loader,
            self.module_names,
            self.init_global,
        )
    }
}
