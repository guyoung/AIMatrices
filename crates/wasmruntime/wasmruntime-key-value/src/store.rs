use std::rc::Rc;
use std::{
    path::PathBuf,
    sync::OnceLock,
    sync::{Arc, Mutex},
};

use anyhow::Result;
use async_trait::async_trait;
use rusqlite::{named_params, Connection};
use tokio::task;

use spin_factor_key_value::{log_cas_error, log_error, Cas, Error, Store, StoreManager, SwapError};

#[derive(Clone, Debug)]
pub enum DatabaseLocation {
    InMemory,
    Path(PathBuf),
}

pub struct KeyValueSqlite {
    location: DatabaseLocation,
    connection: OnceLock<Arc<Mutex<Connection>>>,
}

impl KeyValueSqlite {
    /// Create a new `KeyValueSqlite` store manager.
    ///
    /// If location is `DatabaseLocation::InMemory`, the database will be created in memory.
    /// If it's `DatabaseLocation::Path`, the database will be created at the specified path.
    /// Relative paths will be resolved against the current working directory.
    pub fn new(location: DatabaseLocation) -> Self {
        Self {
            location,
            connection: OnceLock::new(),
        }
    }

    fn create_connection(&self) -> Result<Arc<Mutex<Connection>>, Error> {
        let connection = match &self.location {
            DatabaseLocation::InMemory => Connection::open_in_memory(),
            DatabaseLocation::Path(path) => Connection::open(path),
        }
        .map_err(log_error)?;

        connection
            .execute(
                "CREATE TABLE IF NOT EXISTS spin_key_value (
                           store TEXT NOT NULL,
                           key   TEXT NOT NULL,
                           value BLOB NOT NULL,

                           PRIMARY KEY (store, key)
                        )",
                [],
            )
            .map_err(log_error)?;

        // the array module is needed for `rarray` usage in queries.
        rusqlite::vtab::array::load_module(&connection).map_err(log_error)?;

        Ok(Arc::new(Mutex::new(connection)))
    }
}

#[async_trait]
impl StoreManager for KeyValueSqlite {
    async fn get(&self, name: &str) -> Result<Arc<dyn Store>, Error> {
        let connection = task::block_in_place(|| {
            if let Some(c) = self.connection.get() {
                return Ok(c);
            }
            // Only create the connection if we failed to get it.
            // We might do duplicate work here if there's a race, but that's fine.
            let new = self.create_connection()?;
            Ok(self.connection.get_or_init(|| new))
        })?;

        Ok(Arc::new(SqliteStore {
            name: name.to_owned(),
            connection: connection.clone(),
        }))
    }

    fn is_defined(&self, _store_name: &str) -> bool {
        true
    }

    fn summary(&self, _store_name: &str) -> Option<String> {
        Some(match &self.location {
            DatabaseLocation::InMemory => "a temporary in-memory store".into(),
            DatabaseLocation::Path(path) => format!("\"{}\"", path.display()),
        })
    }
}

struct SqliteStore {
    name: String,
    connection: Arc<Mutex<Connection>>,
}

#[async_trait]
impl Store for SqliteStore {
    async fn get(&self, key: &str) -> Result<Option<Vec<u8>>, Error> {
        task::block_in_place(|| {
            self.connection
                .lock()
                .unwrap()
                .prepare_cached("SELECT value FROM spin_key_value WHERE store=$1 AND key=$2")
                .map_err(log_error)?
                .query_map([&self.name, key], |row| row.get(0))
                .map_err(log_error)?
                .next()
                .transpose()
                .map_err(log_error)
        })
    }

    async fn set(&self, key: &str, value: &[u8]) -> Result<(), Error> {
        task::block_in_place(|| {
            self.connection
                .lock()
                .unwrap()
                .prepare_cached(
                    "INSERT INTO spin_key_value (store, key, value) VALUES ($1, $2, $3)
                     ON CONFLICT(store, key) DO UPDATE SET value=$3",
                )
                .map_err(log_error)?
                .execute(rusqlite::params![&self.name, key, value])
                .map_err(log_error)
                .map(drop)
        })
    }

    async fn delete(&self, key: &str) -> Result<(), Error> {
        task::block_in_place(|| {
            self.connection
                .lock()
                .unwrap()
                .prepare_cached("DELETE FROM spin_key_value WHERE store=$1 AND key=$2")
                .map_err(log_error)?
                .execute([&self.name, key])
                .map_err(log_error)
                .map(drop)
        })
    }

    async fn exists(&self, key: &str) -> Result<bool, Error> {
        Ok(self.get(key).await?.is_some())
    }

    async fn get_keys(&self) -> Result<Vec<String>, Error> {
        task::block_in_place(|| {
            self.connection
                .lock()
                .unwrap()
                .prepare_cached("SELECT key FROM spin_key_value WHERE store=$1")
                .map_err(log_error)?
                .query_map([&self.name], |row| row.get(0))
                .map_err(log_error)?
                .map(|r| r.map_err(log_error))
                .collect()
        })
    }

    async fn get_many(&self, keys: Vec<String>) -> Result<Vec<(String, Option<Vec<u8>>)>, Error> {
        task::block_in_place(|| {
            let sql_value_keys: Vec<rusqlite::types::Value> =
                keys.into_iter().map(rusqlite::types::Value::from).collect();
            let ptr = Rc::new(sql_value_keys);
            let row_iter: Vec<Result<(String, Option<Vec<u8>>), Error>> = self.connection
                .lock()
                .unwrap()
                .prepare_cached("SELECT key, value FROM spin_key_value WHERE store=:name AND key IN rarray(:keys)")
                .map_err(log_error)?
                .query_map(named_params! {":name": &self.name, ":keys": ptr}, |row| {
                    <(String, Option<Vec<u8>>)>::try_from(row)
                })
                .map_err(log_error)?
                .map(|r: Result<(String, Option<Vec<u8>>), rusqlite::Error>| r.map_err(log_error))
                .collect();

            let mut keys_and_values: Vec<(String, Option<Vec<u8>>)> = Vec::new();
            for row in row_iter {
                let res = row.map_err(log_error)?;
                keys_and_values.push(res);
            }
            Ok(keys_and_values)
        })
    }

    async fn set_many(&self, key_values: Vec<(String, Vec<u8>)>) -> Result<(), Error> {
        task::block_in_place(|| {
            let mut binding = self.connection.lock().unwrap();
            let tx = binding.transaction().map_err(log_error)?;
            for kv in key_values {
                tx.prepare_cached(
                    "INSERT INTO spin_key_value (store, key, value) VALUES ($1, $2, $3)
                     ON CONFLICT(store, key) DO UPDATE SET value=$3",
                )
                .map_err(log_error)?
                .execute(rusqlite::params![&self.name, kv.0, kv.1])
                .map_err(log_error)
                .map(drop)?;
            }
            tx.commit().map_err(log_error)
        })
    }

    async fn delete_many(&self, keys: Vec<String>) -> Result<(), Error> {
        task::block_in_place(|| {
            let sql_value_keys: Vec<rusqlite::types::Value> =
                keys.into_iter().map(rusqlite::types::Value::from).collect();
            let ptr = Rc::new(sql_value_keys);
            self.connection
                .lock()
                .unwrap()
                .prepare_cached(
                    "DELETE FROM spin_key_value WHERE store=:name AND key IN rarray(:keys)",
                )
                .map_err(log_error)?
                .execute(named_params! {":name": &self.name, ":keys": ptr})
                .map_err(log_error)
                .map(drop)
        })
    }

    // The assumption with increment is that if the value for the key does not exist, it will be
    // assumed to be zero. In the case that we are unable to unmarshal the value into an i64 an error will be returned.
    async fn increment(&self, key: String, delta: i64) -> Result<i64, Error> {
        task::block_in_place(|| {
            let mut binding = self.connection.lock().unwrap();

            let tx = binding.transaction().map_err(log_error)?;

            let value: Option<Vec<u8>> = tx
                .prepare_cached("SELECT value FROM spin_key_value WHERE store=$1 AND key=$2")
                .map_err(log_error)?
                .query_map([&self.name, &key], |row| row.get(0))
                .map_err(log_error)?
                .next()
                .transpose()
                .map_err(log_error)?;

            let numeric: i64 = match value {
                Some(v) => i64::from_le_bytes(v.try_into().expect("incorrect length")),
                None => 0,
            };

            let new_value = numeric + delta;
            tx.prepare_cached(
                "INSERT INTO spin_key_value (store, key, value) VALUES ($1, $2, $3)
                     ON CONFLICT(store, key) DO UPDATE SET value=$3",
            )
            .map_err(log_error)?
            .execute(rusqlite::params![&self.name, key, new_value.to_le_bytes()])
            .map_err(log_error)
            .map(drop)?;

            tx.commit().map_err(log_error)?;
            Ok(new_value)
        })
    }

    async fn new_compare_and_swap(
        &self,
        bucket_rep: u32,
        key: &str,
    ) -> Result<Arc<dyn Cas>, Error> {
        Ok(Arc::new(CompareAndSwap {
            name: self.name.clone(),
            key: key.to_string(),
            connection: self.connection.clone(),
            value: Mutex::new(None),
            bucket_rep,
        }))
    }
}

struct CompareAndSwap {
    name: String,
    key: String,
    value: Mutex<Option<Vec<u8>>>,
    connection: Arc<Mutex<Connection>>,
    bucket_rep: u32,
}

#[async_trait]
impl Cas for CompareAndSwap {
    async fn current(&self) -> Result<Option<Vec<u8>>, Error> {
        task::block_in_place(|| {
            let value: Option<Vec<u8>> = self
                .connection
                .lock()
                .unwrap()
                .prepare_cached("SELECT value FROM spin_key_value WHERE store=$1 AND key=$2")
                .map_err(log_error)?
                .query_map([&self.name, &self.key], |row| row.get(0))
                .map_err(log_error)?
                .next()
                .transpose()
                .map_err(log_error)?;

            self.value.lock().unwrap().clone_from(&value);
            Ok(value.clone())
        })
    }

    async fn swap(&self, value: Vec<u8>) -> Result<(), SwapError> {
        task::block_in_place(|| {
            let old_value = self.value.lock().unwrap();
            let mut conn = self.connection.lock().unwrap();
            let rows_changed = match old_value.clone() {
                Some(old_val) => {
                    conn
                        .prepare_cached(
                             "UPDATE spin_key_value SET value=:new_value WHERE store=:name and key=:key and value=:old_value")
                        .map_err(log_cas_error)?
                        .execute(named_params! {
                            ":name": &self.name,
                            ":key": self.key,
                            ":old_value": old_val,
                            ":new_value": value,
                        })
                        .map_err(log_cas_error)?
                }
                None => {
                    let tx = conn.transaction().map_err(log_cas_error)?;
                    let rows = tx
                        .prepare_cached(
                            "INSERT INTO spin_key_value (store, key, value) VALUES ($1, $2, $3)
                     ON CONFLICT(store, key) DO UPDATE SET value=$3",
                        )
                        .map_err(log_cas_error)?
                        .execute(rusqlite::params![&self.name, self.key, value])
                        .map_err(log_cas_error)?;
                    tx.commit().map_err(log_cas_error)?;
                    rows
                }
            };

            // We expect only 1 row to be updated. If 0, we know that the underlying value has changed.
            if rows_changed == 1 {
                Ok(())
            } else {
                Err(SwapError::CasFailed("failed to update 1 row".to_owned()))
            }
        })
    }

    async fn bucket_rep(&self) -> u32 {
        self.bucket_rep
    }

    async fn key(&self) -> String {
        self.key.clone()
    }
}
