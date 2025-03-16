use std::collections::HashMap;
use std::path::PathBuf;

use async_trait::async_trait;

use spin_world::comp::whisper_infer;

use whisper_infer_executor::{
    FullParams, SamplingStrategy, WhisperContext, WhisperContextParameters,
};

use whisper_infer_executor::audio::parse_wav;

/// The interface for a language model engine.
#[async_trait]
pub trait WhisperInferEngine: Send + Sync {
    async fn infer(
        &self,
        model_id: String,
        input: Vec<u8>,
        params: whisper_infer::InferencingParams,
    ) -> Result<whisper_infer::InferencingResult, whisper_infer::Error>;

    /// A human-readable summary of the given engine's configuration
    ///
    /// Example: "local model"
    fn summary(&self) -> Option<String> {
        None
    }
}

pub struct LocalWhisperInferEngine {
    model_paths: HashMap<String, PathBuf>,
}

impl LocalWhisperInferEngine {
    pub fn init(models: HashMap<String, String>) -> Result<Self, whisper_infer::Error> {
        println!("Initializing local whisper infer engine");

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

        Ok(LocalWhisperInferEngine { model_paths })
    }

    pub async fn infer(
        &self,
        model_id: String,
        input: Vec<u8>,
        params: whisper_infer::InferencingParams,
    ) -> Result<whisper_infer::InferencingResult, whisper_infer::Error> {
        let model_path = self.model_paths.get(&model_id);

        if model_path.is_none() {
            return Err(whisper_infer::Error::RuntimeError(format!(
                "Model: {} not initialized",
                model_id
            )));
        }

        let ctx = WhisperContext::new_with_params(
            &model_path.unwrap().to_string_lossy(),
            WhisperContextParameters::default(),
        )
        .map_err(|_| {
            whisper_infer::Error::RuntimeError("Failed to open whisper model".to_string())
        })?;

        let samples = parse_wav(input).map_err(|_| {
            whisper_infer::Error::RuntimeError("Error reading WAV file".to_string())
        })?;

        let mut state = ctx.create_state().map_err(|_| {
            whisper_infer::Error::RuntimeError("Failed to create whisper state".to_string())
        })?;

        let mut full_params = FullParams::new(SamplingStrategy::default());

        // Set the number of threads to use to 1.
        full_params.set_n_threads(params.threads as i32);
        // Enable translation.
        full_params.set_translate(true);
        // Set the language to translate to to English.
        full_params.set_language(Some(params.language.as_str()));

        // Enable token level timestamps
        full_params.set_token_timestamps(true);

        // Disable anything that prints to stdout.
        full_params.set_print_special(false);
        full_params.set_print_progress(false);
        full_params.set_print_realtime(false);
        full_params.set_print_timestamps(false);

        let _st = std::time::Instant::now();

        state.full(full_params, &samples).map_err(|_| {
            whisper_infer::Error::RuntimeError("Failed to convert samples".to_string())
        })?;

        let _et = std::time::Instant::now();

        let num_segments = state
            .full_n_segments()
            .expect("failed to get number of segments");

        let mut segments: Vec<whisper_infer::SegmentItem> = Vec::new();

        for i in 0..num_segments {
            let text = state.full_get_segment_text(i).map_err(|_| {
                whisper_infer::Error::RuntimeError("Failed to get segment text".to_string())
            })?;

            let t0 = state.full_get_segment_t0(i).map_err(|_| {
                whisper_infer::Error::RuntimeError("Failed to get start timestamp".to_string())
            })?;

            let t1 = state.full_get_segment_t1(i).map_err(|_| {
                whisper_infer::Error::RuntimeError("Failed to get end timestamp".to_string())
            })?;

            segments.push(whisper_infer::SegmentItem {
                text,
                t0: t0 as u64,
                t1: t1 as u64,
            });
        }

        Ok(whisper_infer::InferencingResult { segments })
    }
}

#[async_trait]
impl WhisperInferEngine for LocalWhisperInferEngine {
    async fn infer(
        &self,
        model_id: String,
        input: Vec<u8>,
        params: whisper_infer::InferencingParams,
    ) -> Result<whisper_infer::InferencingResult, whisper_infer::Error> {
        let result = self.infer(model_id, input, params).await?;

        Ok(result)
    }

    fn summary(&self) -> Option<String> {
        Some("local model".to_string())
    }
}
