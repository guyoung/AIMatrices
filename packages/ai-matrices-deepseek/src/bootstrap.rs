use std::path::PathBuf;
use std::sync::Arc;

use spin_app::App;

use wasmruntime_core::stdio::FollowComponents;
use wasmruntime_core::FactorsConfig;

#[derive(Debug, Default)]
pub struct Bootstrap {
    /// Disable Wasmtime cache.
    pub disable_cache: bool,

    /// Wasmtime cache configuration file.
    pub cache: Option<PathBuf>,

    /// Disable Wasmtime's pooling instance allocator.
    pub disable_pooling: bool,

    /// Print output to stdout/stderr only for given component(s)
    pub follow_components: Vec<String>,

    /// Silence all component output to stdout/stderr
    pub silence_component_logs: bool,

    /// Configuration file for config providers and wasmtime config.
    pub runtime_config_file: Option<PathBuf>,
}

impl Bootstrap {
    pub async fn run(
        &mut self,
        app: &App,
        working_dir: String,
        ip: String,
        port: u16,
        user: Option<String>,
        pass: Option<String>,
    ) -> anyhow::Result<()> {
        let working_dir = PathBuf::from(working_dir);
        let state_dir = working_dir.join("states");
        let log_dir = working_dir.join("logs");

        let follow_components = self.follow_components();

        let common_options = FactorsConfig {
            working_dir,
            runtime_config_file: self.runtime_config_file.clone(),
            state_dir,
            follow_components,
            log_dir,
        };

        let http_app = wasmruntime_http_server::crate_http_trigger_app(
            app,
            &common_options,
            self.disable_cache,
            &self.cache,
            self.disable_pooling,
        )
        .await?;

        let task_app = wasmruntime_task_server::crate_task_trigger_app(
            app,
            &common_options,
            self.disable_cache,
            &self.cache,
            self.disable_pooling,
        )
        .await?;

        let task_handle = tokio::spawn(wasmruntime_task_server::start(Arc::new(task_app)));

        let run_http_server_fut = wasmruntime_http_server::start(
            format!("{ip}:{port}").parse()?,
            Arc::new(http_app),
            user,
            pass,
        );

        let (abortable, abort_handle) = futures::future::abortable(run_http_server_fut);

        ctrlc::set_handler(move || {
            abort_handle.abort();
            task_handle.abort();
        })?;

        match abortable.await {
            Ok(Ok(())) => {
                tracing::info!("Application shut down: exiting");
                Ok(())
            }
            Ok(Err(err)) => {
                tracing::error!("Application execute failed");
                Err(err)
            }
            Err(_aborted) => {
                tracing::info!("User requested shutdown: exiting");
                Ok(())
            }
        }
    }

    fn follow_components(&self) -> FollowComponents {
        if self.silence_component_logs {
            FollowComponents::None
        } else if self.follow_components.is_empty() {
            FollowComponents::All
        } else {
            let followed = self.follow_components.clone().into_iter().collect();
            FollowComponents::Named(followed)
        }
    }
}
