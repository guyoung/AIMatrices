pub use crate::wit::comp::whisper_infer::{
    self, Error, InferencingParams, InferencingResult
};

impl Default for InferencingParams {
    fn default() -> Self {
        Self {
            audio_format: "wav".to_string(),
            language: "en".to_string(),
            threads: 1,
        }
    }
}


pub fn infer(model_id: &str, input: Vec<u8>) -> Result<InferencingResult, Error> {
    whisper_infer::infer(model_id, &input, None)
}

pub fn infer_with_options(
    model_id: &str,
    input: Vec<u8>,
    options: InferencingParams,
) -> Result<InferencingResult, Error> {
    whisper_infer::infer(model_id, &input, Some(&options))
}

