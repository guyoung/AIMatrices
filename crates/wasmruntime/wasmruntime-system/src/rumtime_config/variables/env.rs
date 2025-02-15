use std::{
    collections::HashMap,
    env::VarError,
    path::{Path, PathBuf},
    sync::OnceLock,
};

use anyhow::{self, Context as _};
use serde::Deserialize;
use async_trait::async_trait;

use spin_expressions::{Key, Provider};

/// Configuration for the environment variables provider.
#[derive(Debug, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EnvVariablesConfig {
    /// A prefix to add to variable names when resolving from the environment.
    ///
    /// Unless empty, joined to the variable name with an underscore.
    #[serde(default)]
    pub prefix: Option<String>,
    /// Optional path to a 'dotenv' file which will be merged into the environment.
    #[serde(default)]
    pub dotenv_path: Option<PathBuf>,
}

const DEFAULT_ENV_PREFIX: &str = "SPIN_VARIABLE";

type EnvFetcherFn = Box<dyn Fn(&str) -> Result<String, VarError> + Send + Sync>;

/// A [`Provider`] that uses environment variables.
pub struct EnvVariablesProvider {
    prefix: Option<String>,
    env_fetcher: EnvFetcherFn,
    dotenv_path: Option<PathBuf>,
    dotenv_cache: OnceLock<HashMap<String, String>>,
}

impl Default for EnvVariablesProvider {
    fn default() -> Self {
        Self {
            prefix: None,
            env_fetcher: Box::new(|s| std::env::var(s)),
            dotenv_path: Some(".env".into()),
            dotenv_cache: Default::default(),
        }
    }
}

impl EnvVariablesProvider {
    /// Creates a new EnvProvider.
    ///
    /// * `prefix` - The string prefix to use to distinguish an environment variable that should be used.
    ///    If not set, the default prefix is used.
    /// * `env_fetcher` - The function to use to fetch an environment variable.
    /// * `dotenv_path` - The path to the .env file to load environment variables from. If not set,
    ///    no .env file is loaded.
    pub fn new(
        prefix: Option<impl Into<String>>,
        env_fetcher: impl Fn(&str) -> Result<String, VarError> + Send + Sync + 'static,
        dotenv_path: Option<PathBuf>,
    ) -> Self {
        Self {
            prefix: prefix.map(Into::into),
            dotenv_path,
            env_fetcher: Box::new(env_fetcher),
            dotenv_cache: Default::default(),
        }
    }

    /// Gets the value of a variable from the environment.
    pub fn get_sync(&self, key: &Key) -> anyhow::Result<Option<String>> {
        let prefix = self
            .prefix
            .clone()
            .unwrap_or_else(|| DEFAULT_ENV_PREFIX.to_string());

        let upper_key = key.as_ref().to_ascii_uppercase();
        let env_key = format!("{prefix}_{upper_key}");

        self.query_env(&env_key)
    }

    /// Queries the environment for a variable defaulting to dotenv.
    fn query_env(&self, env_key: &str) -> anyhow::Result<Option<String>> {
        match (self.env_fetcher)(env_key) {
            Err(std::env::VarError::NotPresent) => self.get_dotenv(env_key),
            other => other
                .map(Some)
                .with_context(|| format!("failed to resolve env var {env_key}")),
        }
    }

    fn get_dotenv(&self, key: &str) -> anyhow::Result<Option<String>> {
        let Some(dotenv_path) = self.dotenv_path.as_deref() else {
            return Ok(None);
        };
        let cache = match self.dotenv_cache.get() {
            Some(cache) => cache,
            None => {
                let cache = load_dotenv(dotenv_path)?;
                let _ = self.dotenv_cache.set(cache);
                // Safe to unwrap because we just set the cache.
                // Ensures we always get the first value set.
                self.dotenv_cache.get().unwrap()
            }
        };
        Ok(cache.get(key).cloned())
    }
}

impl std::fmt::Debug for EnvVariablesProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EnvProvider")
            .field("prefix", &self.prefix)
            .field("dotenv_path", &self.dotenv_path)
            .finish()
    }
}

fn load_dotenv(dotenv_path: &Path) -> anyhow::Result<HashMap<String, String>> {
    Ok(dotenvy::from_path_iter(dotenv_path)
        .into_iter()
        .flatten()
        .collect::<Result<HashMap<String, String>, _>>()?)
}

#[async_trait]
impl Provider for EnvVariablesProvider {
    async fn get(&self, key: &Key) -> anyhow::Result<Option<String>> {
        tokio::task::block_in_place(|| self.get_sync(key))
    }
}
