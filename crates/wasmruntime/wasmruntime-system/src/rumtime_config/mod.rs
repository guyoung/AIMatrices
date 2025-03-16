#[cfg(feature = "dbs")]
mod dbs;
mod key_value;
#[cfg(feature = "llm-infer")]
mod llm_infer;
#[cfg(feature = "sd-infer")]
mod sd_infer;
#[cfg(feature = "whisper-infer")]
mod whisper_infer;
mod variables;

use std::path::PathBuf;
use std::sync::Arc;
use std::sync::OnceLock;

pub struct Context {
    pub key_value_runtime_config: Option<Arc<spin_factor_key_value::runtime_config::RuntimeConfig>>,
    pub variablese_runtime_config:
        Option<Arc<spin_factor_variables::runtime_config::RuntimeConfig>>,
    #[cfg(feature = "dbs")]
    pub dbs_runtime_config: Option<Arc<wasmruntime_factor_dbs::runtime_config::RuntimeConfig>>,
    #[cfg(feature = "llm-infer")]
    pub llm_infer_runtime_config:
        Option<Arc<wasmruntime_factor_llm_infer::runtime_config::RuntimeConfig>>,
    #[cfg(feature = "sd-infer")]
    pub sd_infer_runtime_config:
        Option<Arc<wasmruntime_factor_sd_infer::runtime_config::RuntimeConfig>>,
    #[cfg(feature = "whisper-infer")]
    pub whisper_infer_runtime_config:
        Option<Arc<wasmruntime_factor_whisper_infer::runtime_config::RuntimeConfig>>,
}

static CONTEXT: OnceLock<Context> = OnceLock::new();

pub fn get_context(working_dir: &PathBuf, app: &spin_app::App) -> &'static Context {
    CONTEXT.get_or_init(|| {
        let key_value_runtime_config = {
            if let Ok(config) = key_value::config(working_dir, app) {
                Some(Arc::new(config))
            } else {
                None
            }
        };
        let variablese_runtime_config = {
            if let Ok(config) = variables::config(working_dir, app) {
                Some(Arc::new(config))
            } else {
                None
            }
        };
        #[cfg(feature = "dbs")]
        let dbs_runtime_config = {
            if let Ok(config) = dbs::config(working_dir, app) {
                Some(Arc::new(config))
            } else {
                None
            }
        };
        #[cfg(feature = "llm-infer")]
        let llm_infer_runtime_config = {
            if let Ok(config) = llm_infer::config(working_dir, app) {
                Some(Arc::new(config))
            } else {
                None
            }
        };
        #[cfg(feature = "sd-infer")]
        let sd_infer_runtime_config = {
            if let Ok(config) = sd_infer::config(working_dir, app) {
                Some(Arc::new(config))
            } else {
                None
            }
        };
        #[cfg(feature = "whisper-infer")]
        let whisper_infer_runtime_config = {
            if let Ok(config) = whisper_infer::config(working_dir, app) {
                Some(Arc::new(config))
            } else {
                None
            }
        };

        Context {
            key_value_runtime_config,
            variablese_runtime_config,
            #[cfg(feature = "dbs")]
            dbs_runtime_config,
            #[cfg(feature = "llm-infer")]
            llm_infer_runtime_config,
            #[cfg(feature = "sd-infer")]
            sd_infer_runtime_config,
            #[cfg(feature = "whisper-infer")]
            whisper_infer_runtime_config,
        }
    })
}
