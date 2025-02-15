mod txt2img_config;
mod context_config;

pub use context_config::{ ContextConfig, ContextConfigBuilder, ContextConfigBuilderError};
pub use txt2img_config::{ Txt2imgConfig, Txt2imgConfigBuilder, Txt2imgConfigBuilderError};


/***
use derive_builder::Builder;



use crate::clib::CLibPath;

#[derive(Builder, Debug, Clone)]
#[builder(setter(into, strip_option), build_fn(validate = "Self::validate"))]
/// Config struct common to all diffusion methods
pub struct Config {


    /// Path to the input image, required by img2img
    #[builder(default = "Default::default()")]
    pub init_img: CLibPath,

    /// Path to image condition, control net
    #[builder(default = "Default::default()")]
    pub control_image: CLibPath,

    /// Strength for noising/unnoising (default: 0.75)
    #[builder(default = "0.75")]
    pub strength: f32,


    /// Apply canny preprocessor (edge detection) (default: false)
    #[builder(default = "false")]
    pub canny: bool,


}

impl ConfigBuilder {
    fn validate(&self) -> Result<(), ConfigBuilderError> {
        Ok(())
    }


}

***/