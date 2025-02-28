use std::ffi::CString;
use std::path::PathBuf;
use std::pin::pin;

use anyhow::Context;

use crate::llama_cpp_2::llama_backend::LlamaBackend;
use crate::llama_cpp_2::model::params::kv_overrides::ParamOverrideValue;
use crate::llama_cpp_2::model::params::LlamaModelParams;
use crate::llama_cpp_2::model::LlamaModel;

#[derive(Debug, Clone)]
pub struct ModelInstanceConfig {
    /// Disable offloading layers to the gpu
    pub disable_gpu: bool,

    pub logging: bool,

    pub n_gpu_layers: u32,

    pub main_gpu: i32,

    /// override some parameters of the model
    pub kv_overrides: Vec<(String, ParamOverrideValue)>,
}

impl Default for ModelInstanceConfig {
    fn default() -> Self {
        Self {
            disable_gpu: false,
            logging: false,
            n_gpu_layers: 256,
            main_gpu: 0,
            kv_overrides: Vec::new(),
        }
    }
}

pub struct ModelInstance {
    pub model: LlamaModel,
    pub backend: LlamaBackend,
}

impl ModelInstance {
    pub fn create_instnce(
        model_path: PathBuf,
        config: ModelInstanceConfig,
    ) -> anyhow::Result<Self> {
        let mut backend = LlamaBackend::init()?;

        if !config.logging {
            backend.void_logs();
        }

        let model_params = {
            if !config.disable_gpu {
                LlamaModelParams::default()
                    .with_n_gpu_layers(config.n_gpu_layers)
                    .with_main_gpu(config.main_gpu)
            } else {
                LlamaModelParams::default()
            }
        };

        let mut model_params = pin!(model_params);

        for (k, v) in &config.kv_overrides {
            let k = CString::new(k.as_bytes()).with_context(|| format!("Invalid key: {k}"))?;
            model_params.as_mut().append_kv_override(k.as_c_str(), *v);
        }

        let model: LlamaModel = LlamaModel::load_from_file(&backend, model_path, &model_params)
            .with_context(|| "Failed to load model")?;

        Ok(Self { model, backend })
    }
}
