use std::path::PathBuf;
use std::sync::Arc;

//use surrealdb::engine::any::connect;
use surrealdb::kvs::Datastore;


use wasmruntime_factor_dbs::engine::{ SurrealDbEngine };
use wasmruntime_factor_dbs::runtime_config::RuntimeConfig;

pub fn config(working_dir: &PathBuf, _app: &spin_app::App) -> anyhow::Result<RuntimeConfig> {

    let app_db_path = format!("file://{}", working_dir.join("dbs/app__db").display());

    let datastore = futures::executor::block_on(async {
        //connect(app_db_path).await
        Datastore::new(&app_db_path).await
    })?;

    let engine = SurrealDbEngine::new(datastore);

    let engine = Arc::new(engine);

    Ok(RuntimeConfig {
        engine,
    })
}

