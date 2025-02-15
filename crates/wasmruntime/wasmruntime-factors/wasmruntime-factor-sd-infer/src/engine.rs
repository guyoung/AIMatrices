use async_trait::async_trait;
use rand::Rng;
use std::collections::HashMap;
use std::path::PathBuf;

use spin_world::comp::sd_infer;

use sd_infer_executor::config::ContextConfig;
use sd_infer_executor::config::ContextConfigBuilder;
use sd_infer_executor::config::Txt2imgConfigBuilder;
use sd_infer_executor::context::SdContext;
use sd_infer_executor::SampleMethod;

/// The interface for a language model engine.
#[async_trait]
pub trait SdInferEngine: Send + Sync {
    async fn txt2img(
        &self,
        preset: String,
        prompt: String,
        params: sd_infer::InferencingParams,
    ) -> Result<sd_infer::InferencingResult, sd_infer::Error>;

    /// A human-readable summary of the given engine's configuration
    ///
    /// Example: "local model"
    fn summary(&self) -> Option<String> {
        None
    }
}

pub struct LocalSdInferEngine {
    model_paths: HashMap<String, PathBuf>,
}

impl LocalSdInferEngine {
    pub fn init(models: HashMap<String, String>) -> Result<Self, sd_infer::Error> {
        println!("Initializing local sd infer engine");

        println!("models: {:?}", models);

        let mut model_paths = HashMap::new();

        for (k, v) in models {
            let model_path = PathBuf::from(&v);

            if let Ok(exists) = std::fs::exists(&model_path) {
                if exists {
                    model_paths.insert(k, model_path);
                }
            }
        }

        Ok(LocalSdInferEngine { model_paths })
    }

    pub async fn txt2img(
        &self,
        preset: String,
        prompt: String,
        _params: sd_infer::InferencingParams,
    ) -> Result<sd_infer::InferencingResult, sd_infer::Error> {
        /***
        let context_config = PresetBuilder::default()
            .preset(Preset::StableDiffusion1_5)
            .build()
            .map_err(|e| wasi_sd_infer::Error::RuntimeError(e.to_string()))?;
        ***/

        /***
         let context_config = PresetBuilder::default()
           .preset(Preset::Flux1Dev(WeightType::SD_TYPE_Q4_0))
           .build()
           .map_err(|e| sd_infer::Error::RuntimeError(e.to_string()))?;

        ***/

        let context_config = self.crate_context_config(preset.as_str())?;

        let mut sd_context = SdContext::new(context_config.clone(), true);

        let mut rng = rand::thread_rng();

        let seed: i64 = rng.gen_range(i64::MIN..i64::MAX);

        let config = Txt2imgConfigBuilder::default()
            .cfg_scale(1.)
            .sampling_method(SampleMethod::EULER)
            .steps(28)
            .seed(seed)
            .output("output.png")
            .build()
            .map_err(|e| sd_infer::Error::RuntimeError(e.to_string()))?;

        let res = sd_context
            .txt2img(prompt, None, config, &context_config)
            .map_err(|e| sd_infer::Error::RuntimeError(e.to_string()))?;

        let result = sd_infer::InferencingResult { images: res };

        Ok(result)
    }

    pub fn crate_context_config(&self, preset: &str) -> Result<ContextConfig, sd_infer::Error> {
        let mut context_config_builder = ContextConfigBuilder::default();

        match preset {
            "StableDiffusion1_5" => {
                if let Some(model_path) = self.model_paths.get("stable-diffusion-v1-5") {
                    context_config_builder.model(model_path.clone());
                } else {
                    return Err(sd_infer::Error::RuntimeError(
                        "Model stable-diffusion-v1-5 not config".to_string(),
                    ));
                }
            }

            "Flux1Dev" => {
                if let Some(model_path) = self.model_paths.get("flux1-dev") {
                    context_config_builder.diffusion_model(model_path.clone());
                } else {
                    return Err(sd_infer::Error::RuntimeError(
                        "Model flux1-dev not config".to_string(),
                    ));
                }

                if let Some(model_path) = self.model_paths.get("t5xxl") {
                    context_config_builder.t5xxl(model_path.clone());
                } else {
                    return Err(sd_infer::Error::RuntimeError(
                        "Model t5xxl not config".to_string(),
                    ));
                }

                if let Some(model_path) = self.model_paths.get("vae") {
                    context_config_builder.vae(model_path.clone());
                } else {
                    return Err(sd_infer::Error::RuntimeError(
                        "Model vae not config".to_string(),
                    ));
                }

                if let Some(model_path) = self.model_paths.get("clip_l") {
                    context_config_builder.clip_l(model_path.clone());
                } else {
                    return Err(sd_infer::Error::RuntimeError(
                        "Model clip_l not config".to_string(),
                    ));
                }

                context_config_builder.vae_tiling(true);
            }

            _ => {
                return Err(sd_infer::Error::RuntimeError(format!(
                    "Preset: {} not support",
                    preset
                )))
            }
        }

        Ok(context_config_builder.build().unwrap())
    }
}

#[async_trait]
impl SdInferEngine for LocalSdInferEngine {
    async fn txt2img(
        &self,
        preset: String,
        prompt: String,
        params: sd_infer::InferencingParams,
    ) -> Result<sd_infer::InferencingResult, sd_infer::Error> {
        // println!("model: {:?}", model);
        // println!("prompt: {:?}", prompt);
        //println!("params: {:?}", params);

        let result = self.txt2img(preset, prompt, params).await?;

        Ok(result)
    }

    fn summary(&self) -> Option<String> {
        Some("local model".to_string())
    }
}
