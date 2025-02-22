pub mod initial_kv_setter;
mod store;
pub mod summary;

use spin_factor_key_value::runtime_config::MakeKeyValueStore;

use store::{DatabaseLocation, KeyValueSqlite};

/// A key-value store that uses SQLite as the backend.
pub struct WasmtimeKeyValueStore {}

impl WasmtimeKeyValueStore {
    pub fn new() -> Self {
        Self {}
    }
}

impl MakeKeyValueStore for WasmtimeKeyValueStore {
    const RUNTIME_CONFIG_TYPE: &'static str = "spin";
    type StoreManager = KeyValueSqlite;

    fn make_store(&self) -> anyhow::Result<Self::StoreManager> {
        let location = DatabaseLocation::InMemory;

        Ok(KeyValueSqlite::new(location))
    }
}
