use spin_factor_key_value::runtime_config::RuntimeConfig;
use spin_factor_key_value::runtime_config::RuntimeConfigResolver;
use std::path::PathBuf;

use wasmruntime_key_value::WasmtimeKeyValueStore;

const DEFAULT_KEY_VALUE_STORE_LABEL: &str = "default";

pub fn config(_working_dir: &PathBuf, _app: &spin_app::App) -> anyhow::Result<RuntimeConfig> {
    let mut key_value_resolver = RuntimeConfigResolver::new();

    // Register the supported store types.
    // Unwraps are safe because the store types are known to not overlap.
    key_value_resolver.register_store_type(WasmtimeKeyValueStore::new())?;

    key_value_resolver.add_default_store::<WasmtimeKeyValueStore>(DEFAULT_KEY_VALUE_STORE_LABEL)?;

    let key_value = key_value_resolver.resolve().unwrap_or_default();

    Ok(key_value)
}
