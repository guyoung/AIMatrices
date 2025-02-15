use spin_world::{async_trait, comp::variables, wasi::config as wasi_config};

use crate::InstanceState;

#[async_trait]
impl variables::Host for InstanceState {
    async fn get(&mut self, key: String) -> Result<String, variables::Error> {
        let key = spin_expressions::Key::new(&key).map_err(expressions_to_variables_err)?;
        self.expression_resolver
            .resolve(&self.component_id, key)
            .await
            .map_err(expressions_to_variables_err)
    }

    fn convert_error(&mut self, error: variables::Error) -> anyhow::Result<variables::Error> {
        Ok(error)
    }
}

#[async_trait]
impl wasi_config::store::Host for InstanceState {
    async fn get(&mut self, key: String) -> Result<Option<String>, wasi_config::store::Error> {
        match <Self as variables::Host>::get(self, key).await {
            Ok(value) => Ok(Some(value)),
            Err(variables::Error::Undefined(_)) => Ok(None),
            Err(variables::Error::InvalidName(_)) => Ok(None), // this is the guidance from https://github.com/WebAssembly/wasi-runtime-config/pull/19)
            Err(variables::Error::Provider(msg)) => Err(wasi_config::store::Error::Upstream(msg)),
            Err(variables::Error::Other(msg)) => Err(wasi_config::store::Error::Io(msg)),
        }
    }

    async fn get_all(&mut self) -> Result<Vec<(String, String)>, wasi_config::store::Error> {
        let all = self
            .expression_resolver
            .resolve_all(&self.component_id)
            .await;
        all.map_err(|e| {
            match expressions_to_variables_err(e) {
                variables::Error::Undefined(msg) => wasi_config::store::Error::Io(msg), // this shouldn't happen but just in case
                variables::Error::InvalidName(msg) => wasi_config::store::Error::Io(msg), // this shouldn't happen but just in case
                variables::Error::Provider(msg) => wasi_config::store::Error::Upstream(msg),
                variables::Error::Other(msg) => wasi_config::store::Error::Io(msg),
            }
        })
    }

    fn convert_error(
        &mut self,
        err: wasi_config::store::Error,
    ) -> anyhow::Result<wasi_config::store::Error> {
        Ok(err)
    }
}

fn expressions_to_variables_err(err: spin_expressions::Error) -> variables::Error {
    use spin_expressions::Error;
    match err {
        Error::InvalidName(msg) => variables::Error::InvalidName(msg),
        Error::Undefined(msg) => variables::Error::Undefined(msg),
        Error::Provider(err) => variables::Error::Provider(err.to_string()),
        other => variables::Error::Other(format!("{other}")),
    }
}
