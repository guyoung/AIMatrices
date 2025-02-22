pub use crate::wit::comp::sd_infer::{self, Error, InferencingParams, InferencingResult};

impl Default for InferencingParams {
    fn default() -> Self {
        Self { max_tokens: 100 }
    }
}

/// Perform inferencing using the provided model and prompt
pub fn txt2img(preset: &str, prompt: &str) -> Result<InferencingResult, Error> {
    sd_infer::txt2img(preset, prompt, None)
}

/// Perform inferencing using the provided model, prompt, and options
pub fn txt2img_with_options(
    preset: &str,
    prompt: &str,
    options: InferencingParams,
) -> Result<InferencingResult, Error> {
    sd_infer::txt2img(preset, prompt, Some(options))
}
