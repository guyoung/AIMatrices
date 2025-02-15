use std::collections::BTreeMap;
use std::sync::Arc;

use serde::{Deserialize, Serialize};

use nanoid::nanoid;
use surrealdb::dbs::Session;
//use surrealdb::engine::any::Any;
use surrealdb::kvs::Datastore;

use surrealdb::sql::Value;
use surrealdb::sql;

/// The characters which are supported in server record IDs.
pub const ID_CHARS: [char; 36] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i',
    'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
];

#[derive(Debug, Serialize, Deserialize)]
struct User {
    name: String,
    age: u8,
    email: String,
}

pub struct SurrealDbEngine {
    datastore: Arc<Datastore>,
}
impl SurrealDbEngine {
    pub fn new(datastore: Datastore) -> Self {
        Self {
            datastore: Arc::new(datastore),
        }
    }

    pub async fn create(
        &self,
        namespace: String,
        database: String,
        table: String,
        uid: String,
        data: serde_json::Value,
    ) -> anyhow::Result<serde_json::Value> {
        let seesion = Session::owner()
            .with_ns(namespace.as_str())
            .with_db(database.as_str());

        let data = surrealdb::sql::value(data.to_string().as_str())?;

        let sql = "CREATE type::thing($table, $uid) CONTENT $data";

        let mut vars: BTreeMap<String, Value> = BTreeMap::new();
        vars.insert("table".to_string(), Value::from(table));
        vars.insert("uid".to_string(), Value::from(uid));
        vars.insert("data".to_string(), data);

        let res = self.datastore.execute(sql, &seesion, Some(vars)).await?;

        let res = sql::to_value(res)?;

        Ok(res.into())


    }


    pub async fn update(
        &self,
        namespace: String,
        database: String,
        table: String,
        uid: String,
        data: serde_json::Value,
    ) -> anyhow::Result<serde_json::Value>
    {
        let seesion = Session::owner()
            .with_ns(namespace.as_str())
            .with_db(database.as_str());

        let data = surrealdb::sql::value(data.to_string().as_str())?;

        let sql = "UPDATE type::thing($table, $uid) CONTENT $data";

        let mut vars: BTreeMap<String, Value> = BTreeMap::new();
        vars.insert("table".to_string(), Value::from(table));
        vars.insert("uid".to_string(), Value::from(uid));
        vars.insert("data".to_string(), data);

        let res = self.datastore.execute(sql, &seesion, Some(vars)).await?;

        let res = sql::to_value(res)?;

        Ok(res.into())
    }

    pub async fn delete(
        &self,
        namespace: String,
        database: String,
        table: String,
        uid: String,
    ) -> anyhow::Result<serde_json::Value>

    {
        let seesion = Session::owner()
            .with_ns(namespace.as_str())
            .with_db(database.as_str());

        let sql = "DELETE FROM type::thing($table, $uid) RETURN BEFORE";

        let mut vars: BTreeMap<String, Value> = BTreeMap::new();
        vars.insert("table".to_string(), Value::from(table));
        vars.insert("uid".to_string(), Value::from(uid));

        let res = self.datastore.execute(sql, &seesion, Some(vars)).await?;

        let res = sql::to_value(res)?;

        Ok(res.into())
    }


    pub async fn select_all(
        &self,
        namespace: String,
        database: String,
        table: String,
    ) -> anyhow::Result<serde_json::Value>
    {
        let seesion = Session::owner()
            .with_ns(namespace.as_str())
            .with_db(database.as_str());


        let sql = "SELECT * FROM type::table($table)";

        let mut vars: BTreeMap<String, Value> = BTreeMap::new();
        vars.insert("table".to_string(), Value::from(table));

        let res = self.datastore.execute(sql, &seesion, Some(vars)).await?;

        let res = sql::to_value(res)?;

        Ok(res.into())
    }


    pub async fn select(
        &self,
        namespace: String,
        database: String,
        table: String,
        uid: String,
    ) -> anyhow::Result<serde_json::Value>
    {
        let seesion = Session::owner()
            .with_ns(namespace.as_str())
            .with_db(database.as_str());


        let sql = "SELECT * FROM type::thing($table, $uid)";

        let mut vars: BTreeMap<String, Value> = BTreeMap::new();
        vars.insert("table".to_string(), Value::from(table));
        vars.insert("uid".to_string(), Value::from(uid));

        let res = self.datastore.execute(sql, &seesion, Some(vars)).await?;

        let res = sql::to_value(res)?;

        Ok(res.into())
    }

    pub async fn query(
        &self,
        namespace: String,
        database: String,
        sql: String,
    ) -> anyhow::Result<serde_json::Value>
    {
        let seesion = Session::owner()
            .with_ns(namespace.as_str())
            .with_db(database.as_str());

        let res = self.datastore.execute(sql.as_str(), &seesion, None).await?;

        let res = sql::to_value(res)?;

        Ok(res.into())
    }


    /***
    pub async fn query_with<T>(
        &self,
        namespace: String,
        database: String,
        sql: String,
        params: String,
    ) -> anyhow::Result<Vec<T>>

    {

        let params =
            surrealdb::sql::json(&params).or_else(|_| Err(anyhow::anyhow!("Invalid params")))?;

        let params = match params {
            Value::Object(v) => v,
            _ => return Err(anyhow::anyhow!("Invalid params")),
        };

        let seesion = Session::owner()
            .with_ns(namespace.as_str())
            .with_db(database.as_str());


        let res = self.datastore.execute(sql.as_str(), &seesion, None).await?;

        let res = sql::to_value(res)?;

        Ok(res.into())
    }
***/

    pub async fn execute<>(
        &self,
        namespace: String,
        database: String,
        sql: String,
    ) -> anyhow::Result<()>
    {
        let seesion = Session::owner()
            .with_ns(namespace.as_str())
            .with_db(database.as_str());

        let _ = self.datastore.execute(sql.as_str(), &seesion, None).await?;

        Ok(())
    }

    pub fn generate_id(&self) ->String {
        nanoid!(20, &ID_CHARS)
    }


}
