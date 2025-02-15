#[cfg(feature = "json")]
use serde::{de::DeserializeOwned, Serialize};

pub use super::wit::comp::key_value::{self, Error, Store};

impl Store {
    /// Open the default store.
    ///
    /// This is equivalent to `Store::open("default")`.
    pub fn open_default() -> Result<Self, Error> {
        Self::open("default")
    }
}

impl Store {
    #[cfg(feature = "json")]
    /// Serialize the given data to JSON, then set it as the value for the specified `key`.
    pub fn set_json<T: Serialize>(
        &self,
        key: impl AsRef<str>,
        value: &T,
    ) -> Result<(), anyhow::Error> {
        Ok(self.set(key.as_ref(), &serde_json::to_vec(value)?)?)
    }

    #[cfg(feature = "json")]
    /// Deserialize an instance of type `T` from the value of `key`.
    pub fn get_json<T: DeserializeOwned>(
        &self,
        key: impl AsRef<str>,
    ) -> Result<Option<T>, anyhow::Error> {
        let Some(value) = self.get(key.as_ref())? else {
            return Ok(None);
        };
        Ok(serde_json::from_slice(&value)?)
    }
}
