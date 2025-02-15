use std::path::PathBuf;
use derive_builder::Builder;

use crate::SampleMethod;

use crate::clib::{CLibPath, ClipSkip};


#[derive(Builder, Debug, Clone)]
#[builder(setter(into, strip_option), build_fn(validate = "Self::validate"))]
pub struct Txt2imgConfig {
    /// Ignore last layers of CLIP network; 1 ignores none, 2 ignores one layer (default: -1)
    /// <= 0 represents unspecified, will be 1 for SD1.x, 2 for SD2.x
    #[builder(default = "ClipSkip::Unspecified")]
    pub clip_skip: ClipSkip,

    /// Unconditional guidance scale (default: 7.0)
    #[builder(default = "7.0")]
    pub cfg_scale: f32,

    /// Guidance (default: 3.5)
    #[builder(default = "3.5")]
    pub guidance: f32,

    /// Image height, in pixel space (default: 512)
    #[builder(default = "512")]
    pub height: i32,

    /// Image width, in pixel space (default: 512)
    #[builder(default = "512")]
    pub width: i32,

    /// Sampling-method (default: EULER_A)
    #[builder(default = "SampleMethod::EULER_A")]
    pub sampling_method: SampleMethod,

    /// Number of sample steps (default: 20)
    #[builder(default = "20")]
    pub steps: i32,

    /// strength for noising/unnoising
    #[builder(default = "0.75")]
    pub strength: f32,

    /// RNG seed (default: 42, use random seed for < 0)
    #[builder(default = "42")]
    pub seed: i64,

    /// Number of images to generate (default: 1)
    #[builder(default = "1")]
    pub batch_count: i32,

    /// Strength to apply Control Net (default: 0.9)
    /// 1.0 corresponds to full destruction of information in init
    #[builder(default = "0.9")]
    pub control_strength: f32,

    /// Strength for keeping input identity (default: 20%)
    #[builder(default = "20.0")]
    pub style_ratio: f32,

    /// Normalize PHOTOMAKER input id images
    #[builder(default = "false")]
    pub normalize_input: bool,

    /// Path to PHOTOMAKER input id images dir
    #[builder(default = "Default::default()")]
    pub input_id_images: CLibPath,

    /// Layers to skip for SLG steps: (default: [7,8,9])
    #[builder(default = "vec![7, 8, 9]")]
    pub skip_layer: Vec<i32>,

    /// skip layer guidance (SLG) scale, only for DiT models: (default: 0)
    /// 0 means disabled, a value of 2.5 is nice for sd3.5 medium
    #[builder(default = "0.")]
    pub slg_scale: f32,

    /// SLG enabling point: (default: 0.01)
    #[builder(default = "0.01")]
    pub skip_layer_start: f32,

    /// SLG disabling point: (default: 0.2)
    #[builder(default = "0.2")]
    pub skip_layer_end: f32,


    /// -------------------------------------------------------------------------------------------
    /// Path to write result image to (default: ./output.png)
    #[builder(default = "PathBuf::from(\"./output.png\")")]
    pub output: PathBuf,

    /// Run the ESRGAN upscaler this many times (default 1)
    #[builder(default = "0")]
    pub upscale_repeats: i32,

}

impl Txt2imgConfigBuilder {
    fn validate(&self) -> Result<(), Txt2imgConfigBuilderError> {
        self.validate_output_dir()?;

        Ok(())

    }

    fn validate_output_dir(&self) -> Result<(), Txt2imgConfigBuilderError> {
        let is_dir = self.output.as_ref().is_some_and(|val| val.is_dir());
        let multiple_items = self.batch_count.as_ref().is_some_and(|val| *val > 1);
        if is_dir == multiple_items {
            Ok(())
        } else {
            Err(Txt2imgConfigBuilderError::ValidationError(
                "When batch_count > 0, ouput should point to folder and viceversa".to_owned(),
            ))
        }
    }

}