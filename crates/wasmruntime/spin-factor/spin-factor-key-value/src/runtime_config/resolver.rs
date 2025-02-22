use std::{collections::HashMap, sync::Arc};

use anyhow::Context as _;

use super::{MakeKeyValueStore, RuntimeConfig, StoreManager};

/// A function that creates a store manager from a TOML table.
type StoreFn = Arc<dyn Fn() -> anyhow::Result<Arc<dyn StoreManager>> + Send + Sync>;

fn store_fn<T: MakeKeyValueStore>(provider_type: T) -> StoreFn {
    Arc::new(move || {
        let provider = provider_type
            .make_store()
            .context("could not make key-value store from runtime config")?;
        Ok(Arc::new(provider))
    })
}

/// Converts from runtime configuration into a [`RuntimeConfig`].
///
/// The various store types are
/// registered with the resolver using `add_store_type`. The default store for a
/// label is registered using `add_default_store`.
#[derive(Default, Clone)]
pub struct RuntimeConfigResolver {
    /// A map of store types to a function that returns the appropriate store
    store_types: HashMap<&'static str, StoreFn>,
    /// A map of default store configurations for a label.
    defaults: HashMap<&'static str, String>,
}

impl RuntimeConfigResolver {
    /// Create a new RuntimeConfigResolver.
    pub fn new() -> Self {
        <Self as Default>::default()
    }

    /// Adds a default store configuration for a label.
    ///
    /// Users must ensure that the store type for `config` has been registered with
    /// the resolver using [`Self::register_store_type`].
    pub fn add_default_store<T>(&mut self, label: &'static str) -> anyhow::Result<()>
    where
        T: MakeKeyValueStore,
    {
        self.defaults
            .insert(label, T::RUNTIME_CONFIG_TYPE.to_owned());
        Ok(())
    }

    /// Registers a store type to the resolver.
    pub fn register_store_type<T: MakeKeyValueStore>(
        &mut self,
        store_type: T,
    ) -> anyhow::Result<()> {
        if self
            .store_types
            .insert(T::RUNTIME_CONFIG_TYPE, store_fn(store_type))
            .is_some()
        {
            anyhow::bail!(
                "duplicate key value store type {:?}",
                T::RUNTIME_CONFIG_TYPE
            );
        }
        Ok(())
    }

    /// The default stores are also added to the runtime config.
    pub fn resolve(&self) -> anyhow::Result<RuntimeConfig> {
        let mut runtime_config = RuntimeConfig::default();

        for (&label, config_type) in &self.defaults {
            if !runtime_config.store_managers.contains_key(label) {
                let store_manager = self
                    .store_manager_from_config(config_type.as_str())
                    .with_context(|| {
                        format!("could not configure key-value store with label '{label}'")
                    })?;
                runtime_config.add_store_manager(label.to_owned(), store_manager);
            }
        }
        Ok(runtime_config)
    }

    ///  returns a store manager.
    ///
    /// Errors if there is no [`MakeKeyValueStore`] registered for the store config's type
    /// or if the store manager cannot be created from the config.
    fn store_manager_from_config(
        &self,
        config_type: &str,
    ) -> anyhow::Result<Arc<dyn StoreManager>> {
        let maker = self.store_types.get(config_type).with_context(|| {
            format!("the store type '{config_type}' was not registered with the config resolver")
        })?;
        maker()
    }
}
