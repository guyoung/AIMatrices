
use std::env;

use once_cell::sync::Lazy;

pub mod loader;
pub mod require;
pub mod resolver;

pub const ENV_QJSRUNTIME_PLATFORM: &str = "QJSRUNTIME_PLATFORM";

// added when .cjs files are imported
pub const CJS_IMPORT_PREFIX: &str = "__cjs:";
// added to force CJS imports in loader
pub const CJS_LOADER_PREFIX: &str = "__cjsm:";

pub static LLRT_PLATFORM: Lazy<String> = Lazy::new(|| {
    env::var(ENV_QJSRUNTIME_PLATFORM)
        .ok()
        .filter(|platform| platform == "node")
        .unwrap_or_else(|| "browser".to_string())
});
