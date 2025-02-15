//! The runtime configuration for the variables factor used in the Spin CLI.

mod env;
mod statik;

use std::path::PathBuf;
use std::sync::Arc;

use serde::Deserialize;

use spin_expressions::Provider;

use spin_factor_variables::runtime_config::RuntimeConfig;

pub use env::*;
pub use statik::*;

pub fn config(_working_dir: &PathBuf, _app: &spin_app::App) -> anyhow::Result<RuntimeConfig> {
    // Always include the environment variable provider.
    let var_provider = vec![Arc::<EnvVariablesProvider>::default() as _];

    let value: Option<toml::Table> = None;

    let Some(array) = value else {
        return Ok(RuntimeConfig {
            providers: var_provider,
        });
    };

    let provider_configs: Vec<VariableProviderConfiguration> = array.clone().try_into()?;

    let mut providers = provider_configs
        .into_iter()
        .map(VariableProviderConfiguration::into_provider)
        .collect::<anyhow::Result<Vec<_>>>()?;
    providers.extend(var_provider);

    Ok(RuntimeConfig { providers })
}


/// A runtime configuration used in the Spin CLI for one type of variable provider.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum VariableProviderConfiguration {
    /// A static provider of variables.
    Static(StaticVariablesProvider),
    /// An environment variable provider.
    Env(EnvVariablesConfig),
}

impl VariableProviderConfiguration {
    /// Returns the provider for the configuration.
    pub fn into_provider(self) -> anyhow::Result<Arc<dyn Provider>> {
        let provider: Arc<dyn Provider> = match self {
            VariableProviderConfiguration::Static(provider) => Arc::new(provider),
            VariableProviderConfiguration::Env(config) => Arc::new(env::EnvVariablesProvider::new(
                config.prefix,
                |s| std::env::var(s),
                config.dotenv_path,
            )),
        };
        Ok(provider)
    }
}
