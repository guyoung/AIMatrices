use std::collections::HashMap;
use std::path::PathBuf;

use async_trait::async_trait;

use spin_world::comp::llm_infer;

use llama_infer_executor::{LlamaContext, LlamaContextConfig};
use llama_infer_executor::{ModelInstance, ModelInstanceConfig};

use llama_infer_executor::InferencingParams;

#[async_trait]
pub trait LlmInferEngine: Send + Sync {
    async fn infer(
        &self,
        model_id: String,
        prompt: String,
        params: llm_infer::InferencingParams,
    ) -> Result<llm_infer::InferencingResult, llm_infer::Error>;

    async fn infer_chat(
        &self,
        model_id: String,
        messages: Vec<(String, String)>,
        params: llm_infer::InferencingParams,
    ) -> Result<llm_infer::InferencingResult, llm_infer::Error>;

    async fn generate_embeddings(
        &self,
        model_id: String,
        data: Vec<String>,
    ) -> Result<llm_infer::EmbeddingsResult, llm_infer::Error>;

    /// A human-readable summary of the given engine's configuration
    ///
    /// Example: "local model"
    fn summary(&self) -> Option<String> {
        None
    }
}

pub struct LocalLlmInferEngine {
    model_cached: bool,
    model_paths: HashMap<String, PathBuf>,
    model_instances: HashMap<String, ModelInstance>,
}

impl LocalLlmInferEngine {
    pub fn init(
        models: HashMap<String, String>,
        model_cached: bool,
    ) -> Result<Self, llm_infer::Error> {
        println!("Initializing local llm infer engine");

        println!("models: {:?}", models);

        let mut model_paths = HashMap::new();
        let mut model_instances = HashMap::new();

        for (k, v) in models {
            let model_path = PathBuf::from(&v);

            if let Ok(exists) = std::fs::exists(&model_path) {
                if exists {
                    model_paths.insert(k.clone(), model_path.clone());

                    if model_cached {
                        let mut model_instance_config = ModelInstanceConfig::default();

                        if cfg!(debug_assertions) {
                            model_instance_config.logging = true;
                        } else {
                            model_instance_config.logging = false;
                        }

                        let model_instance =
                            ModelInstance::create_instnce(model_path, model_instance_config)
                                .map_err(|e| llm_infer::Error::RuntimeError(e.to_string()))?;

                        model_instances.insert(k, model_instance);
                    }
                }
            }
        }

        Ok(LocalLlmInferEngine {
            model_cached,
            model_paths,
            model_instances,
        })
    }

    pub async fn infer(
        &self,
        model_id: String,
        prompt: String,
        params: llm_infer::InferencingParams,
    ) -> Result<llm_infer::InferencingResult, llm_infer::Error> {
        let model_path = self.model_paths.get(&model_id);

        if model_path.is_none() {
            return Err(llm_infer::Error::RuntimeError(format!(
                "Model: {} not initialized",
                model_id
            )));
        }

        if self.model_cached && self.model_instances.contains_key(&model_id) {
            let model_instance = self.model_instances.get(&model_id).unwrap();

            infer_inner(model_instance, prompt, params).await
        } else {
            let mut model_instance_config = ModelInstanceConfig::default();

            if cfg!(debug_assertions) {
                model_instance_config.logging = true;
            } else {
                model_instance_config.logging = false;
            }

            let model_instance =
                ModelInstance::create_instnce(model_path.unwrap().clone(), model_instance_config)
                    .map_err(|e| llm_infer::Error::RuntimeError(e.to_string()))?;

            infer_inner(&model_instance, prompt, params).await
        }
    }

    pub async fn infer_chat(
        &self,
        model_id: String,
        messages: Vec<(String, String)>,
        params: llm_infer::InferencingParams,
    ) -> Result<llm_infer::InferencingResult, llm_infer::Error> {
        let model_path = self.model_paths.get(&model_id);

        if model_path.is_none() {
            return Err(llm_infer::Error::RuntimeError(format!(
                "Model: {} not initialized",
                model_id
            )));
        }

        if self.model_cached && self.model_instances.contains_key(&model_id) {
            let model_instance = self.model_instances.get(&model_id).unwrap();

            infer_chat_inner(model_instance, messages, params).await
        } else {
            let mut model_instance_config = ModelInstanceConfig::default();

            if cfg!(debug_assertions) {
                model_instance_config.logging = true;
            } else {
                model_instance_config.logging = false;
            }

            let model_instance =
                ModelInstance::create_instnce(model_path.unwrap().clone(), model_instance_config)
                    .map_err(|e| llm_infer::Error::RuntimeError(e.to_string()))?;

            infer_chat_inner(&model_instance, messages, params).await
        }
    }

    pub async fn generate_embeddings(
        &self,
        model_id: String,
        text: Vec<String>,
    ) -> Result<llm_infer::EmbeddingsResult, llm_infer::Error> {
        let model_path = self.model_paths.get(&model_id);

        if model_path.is_none() {
            return Err(llm_infer::Error::RuntimeError(format!(
                "Model: {} not initialized",
                model_id
            )));
        }

        if self.model_cached && self.model_instances.contains_key(&model_id) {
            let model_instance = self.model_instances.get(&model_id).unwrap();

            generate_embeddings(model_instance, text).await
        } else {
            let mut model_instance_config = ModelInstanceConfig::default();

            if cfg!(debug_assertions) {
                model_instance_config.logging = true;
            } else {
                model_instance_config.logging = false;
            }

            let model_instance =
                ModelInstance::create_instnce(model_path.unwrap().clone(), model_instance_config)
                    .map_err(|e| llm_infer::Error::RuntimeError(e.to_string()))?;

            generate_embeddings(&model_instance, text).await
        }
    }
}

async fn infer_inner(
    model_instance: &ModelInstance,
    prompt: String,
    params: llm_infer::InferencingParams,
) -> Result<llm_infer::InferencingResult, llm_infer::Error> {
    let context_config = LlamaContextConfig::default();

    let mut context = LlamaContext::new(model_instance, &context_config, false)
        .map_err(|e| llm_infer::Error::RuntimeError(e.to_string()))?;

    let mut infer_params = InferencingParams::default();
    infer_params.max_tokens = params.max_tokens as i32;
    infer_params.temperature = Some(params.temperature);
    infer_params.top_k = Some(params.top_k as i32);
    infer_params.top_p = Some(params.top_p);
    infer_params.penalty_repeat = Some(params.penalty_repeat);
    infer_params.penalty_last_n = Some(params.penalty_last_n as i32);
    infer_params.penalty_freq = Some(params.penalty_freq);
    infer_params.penalty_present = Some(params.penalty_present);

    let result = context
        .infer_simple(prompt.as_str(), &infer_params)
        .map_err(|e| llm_infer::Error::RuntimeError(e.to_string()))?;

    let result = llm_infer::InferencingResult {
        text: String::from_utf8_lossy(&result.text).to_string(),
        usage: llm_infer::InferencingUsage {
            prompt_token_count: result.prompt_token_count as u32,
            generated_token_count: result.generated_token_count as u32,
        },
    };

    Ok(result)
}

async fn infer_chat_inner(
    model_instance: &ModelInstance,
    messages: Vec<(String, String)>,
    params: llm_infer::InferencingParams,
) -> Result<llm_infer::InferencingResult, llm_infer::Error> {
    let context_config = LlamaContextConfig::default();

    let mut context = LlamaContext::new(model_instance, &context_config, false)
        .map_err(|e| llm_infer::Error::RuntimeError(e.to_string()))?;

    let mut infer_params = InferencingParams::default();
    infer_params.max_tokens = params.max_tokens as i32;
    infer_params.temperature = Some(params.temperature);
    infer_params.top_k = Some(params.top_k as i32);
    infer_params.top_p = Some(params.top_p);
    infer_params.penalty_repeat = Some(params.penalty_repeat);
    infer_params.penalty_last_n = Some(params.penalty_last_n as i32);
    infer_params.penalty_freq = Some(params.penalty_freq);
    infer_params.penalty_present = Some(params.penalty_present);

    let result = context
        .infer_chat(messages, &infer_params)
        .map_err(|e| llm_infer::Error::RuntimeError(e.to_string()))?;

    let result = llm_infer::InferencingResult {
        text: String::from_utf8_lossy(&result.text).to_string(),
        usage: llm_infer::InferencingUsage {
            prompt_token_count: result.prompt_token_count as u32,
            generated_token_count: result.generated_token_count as u32,
        },
    };

    Ok(result)
}

async fn generate_embeddings(
    model_instance: &ModelInstance,
    text: Vec<String>,
) -> Result<llm_infer::EmbeddingsResult, llm_infer::Error> {
    let context_config = LlamaContextConfig::default();

    let context = LlamaContext::new(model_instance, &context_config, false)
        .map_err(|e| llm_infer::Error::RuntimeError(e.to_string()))?;

    let _params = InferencingParams::default();

    let result = context
        .generate_embeddings(text)
        .map_err(|e| llm_infer::Error::RuntimeError(e.to_string()))?;

    let result = llm_infer::EmbeddingsResult {
        embeddings: result.embeddings,
        usage: llm_infer::EmbeddingsUsage {
            prompt_token_count: result.prompt_token_count as u32,
        },
    };

    Ok(result)
}

#[async_trait]
impl LlmInferEngine for LocalLlmInferEngine {
    async fn infer(
        &self,
        model_id: String,
        prompt: String,
        params: llm_infer::InferencingParams,
    ) -> Result<llm_infer::InferencingResult, llm_infer::Error> {
        //println!("model: {:?}", model);
        //println!("prompt: {:?}", prompt);
        //println!("params: {:?}", params);

        let result = self.infer(model_id, prompt, params).await?;

        Ok(result)
    }

    async fn infer_chat(
        &self,
        model_id: String,
        messages: Vec<(String, String)>,
        params: llm_infer::InferencingParams,
    ) -> Result<llm_infer::InferencingResult, llm_infer::Error> {
        //println!("model: {:?}", model);
        //println!("prompt: {:?}", prompt);
        //println!("params: {:?}", params);

        let result = self.infer_chat(model_id, messages, params).await?;

        Ok(result)
    }

    async fn generate_embeddings(
        &self,
        model_id: String,
        data: Vec<String>,
    ) -> Result<llm_infer::EmbeddingsResult, llm_infer::Error> {
        let result = self.generate_embeddings(model_id, data).await?;

        Ok(result)
    }

    fn summary(&self) -> Option<String> {
        Some("local model".to_string())
    }
}
