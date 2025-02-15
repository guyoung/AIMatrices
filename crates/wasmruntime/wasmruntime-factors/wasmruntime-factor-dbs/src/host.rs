use async_trait::async_trait;
use serde_json::Value;

use wasmtime::component::Resource;

use crate::InstanceState;
use spin_world::comp::dbs;

impl InstanceState {
    async fn establish_connection(
        &mut self,
        database: String,
        shared: bool
    ) -> Result<Resource<dbs::Connection>, dbs::Error> {
        let namespace = {
            if shared {
                "__shared__".to_string()
            } else {
                self.namespace.clone()
            }
        };

        self.connections
            .push((namespace, database))
            .map(Resource::new_own)
            .map_err(|_| dbs::Error::ConnectionFailed)
    }

    async fn get_conn(
        &self,
        connection: Resource<dbs::Connection>,
    ) -> Result<(String, String), dbs::Error> {
        self.connections
            .get(connection.rep())
            .ok_or(dbs::Error::ConnectionFailed)
            .map(|c| c.clone())
    }
}

impl dbs::Host for InstanceState {}

#[async_trait]
impl dbs::HostConnection for InstanceState {
    async fn open(
        &mut self,
        database: String,
        shared: bool
    ) -> anyhow::Result<Result<Resource<dbs::Connection>, dbs::Error>> {
        let res = self.establish_connection(database, shared).await;

        Ok(res)
    }

    async fn create(
        &mut self,
        connection: Resource<dbs::Connection>,
        table: String,
        uid: Option<String>,
        data: Vec<u8>,
    ) -> anyhow::Result<Result<Vec<u8>, dbs::Error>> {
        let (namespace, database) = self.get_conn(connection).await?;

        let data: Value = serde_json::from_slice(&data).map_err(|_| dbs::Error::BadParameter)?;

        if let Value::Object(data) = data {
            let uid = {
                if let Some(uid) = uid {
                    uid.to_string()
                } else {
                    let uid = self.engine.generate_id();
                    //data.insert("uid".to_string(), uid.clone().into());
                    uid
                }
            };

            let res = self
                .engine
                .create(namespace, database, table, uid, data.into())
                .await
                .map_err(|e| {
                    println!("{:?}", e);

                    dbs::Error::OperationFailed
                });

            Ok(convert_result(res))
        } else {
            Ok(Err(dbs::Error::BadParameter))
        }
    }

    async fn update(
        &mut self,
        connection: Resource<dbs::Connection>,
        table: String,
        uid: String,
        data: Vec<u8>,
    ) -> anyhow::Result<Result<Vec<u8>, dbs::Error>> {

        let (namespace, database) = self.get_conn(connection).await?;

        let data: Value = serde_json::from_slice(&data).map_err(|_| dbs::Error::BadParameter)?;

        if let Value::Object(data) = data {
            let res = self
                .engine
                .update(namespace, database, table, uid, data.into())
                .await
                .map_err(|_| dbs::Error::OperationFailed);

            Ok(convert_result(res))
        } else {
            Ok(Err(dbs::Error::BadParameter))
        }

    }

    async fn delete(
        &mut self,
        connection: Resource<dbs::Connection>,
        table: String,
        uid: String,
    ) -> anyhow::Result<Result<Vec<u8>, dbs::Error>> {


        let (namespace, database) = self.get_conn(connection).await?;

        let res = self
            .engine
            .delete(namespace, database, table, uid)
            .await
            .map_err(|_| dbs::Error::OperationFailed);

        Ok(convert_result(res))

    }

    async fn select_all(
        &mut self,
        connection: Resource<dbs::Connection>,
        table: String,
    ) -> anyhow::Result<Result<Vec<u8>, dbs::Error>> {
        let (namespace, database) = self.get_conn(connection).await?;

        let res = self
            .engine
            .select_all(namespace, database, table)
            .await
            .map_err(|_| dbs::Error::OperationFailed);

        Ok(convert_result(res))
    }

    async fn select(
        &mut self,
        connection: Resource<dbs::Connection>,
        table: String,
        uid: String,
    ) -> anyhow::Result<Result<Vec<u8>, dbs::Error>> {

        let (namespace, database) = self.get_conn(connection).await?;

        let res = self
            .engine
            .select(namespace, database, table, uid)
            .await
            .map_err(|_| dbs::Error::OperationFailed);

        Ok(convert_result(res))

    }

    async fn query(
        &mut self,
        connection: Resource<dbs::Connection>,
        sql: String,
    ) -> anyhow::Result<Result<Vec<u8>, dbs::Error>> {
        let (namespace, database) = self.get_conn(connection).await?;

        let res = self
            .engine
            .query(namespace, database, sql)
            .await
            .map_err(|_| dbs::Error::OperationFailed);

        Ok(convert_result(res))

    }

    async fn drop(&mut self, connection: Resource<dbs::Connection>) -> anyhow::Result<()> {
        self.connections.remove(connection.rep());
        Ok(())
    }
}

fn convert_result(input: Result<Value, dbs::Error>) -> Result<Vec<u8>, dbs::Error> {
    match input {
        Ok(val) => {
            let val = serde_json::to_vec(&val).map_err(|_| dbs::Error::OtherError)?;

            Ok(val)
        }
        Err(e) => Err(e),
    }
}
