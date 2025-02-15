use std::path::Path;
use derive_builder::Builder;


use crate::WeightType;
use crate::RngFunction;
use crate::Schedule;

use crate::clib::CLibPath;


#[derive(Builder, Debug, Clone)]
#[builder(setter(into, strip_option), build_fn(validate = "Self::validate"))]
pub struct ContextConfig {
    /// Path to full model
    #[builder(default = "Default::default()")]
    pub model: CLibPath,

    /// path to the clip-l text encoder
    #[builder(default = "Default::default()")]
    pub clip_l: CLibPath,

    /// path to the clip-g text encoder
    #[builder(default = "Default::default()")]
    pub clip_g: CLibPath,

    /// Path to the t5xxl text encoder
    #[builder(default = "Default::default()")]
    pub t5xxl: CLibPath,

    /// Path to the standalone diffusion model
    #[builder(default = "Default::default()")]
    pub diffusion_model: CLibPath,

    /// Path to vae
    #[builder(default = "Default::default()")]
    pub vae: CLibPath,


    /// Path to taesd. Using Tiny AutoEncoder for fast decoding (low quality)
    #[builder(default = "Default::default()")]
    pub taesd: CLibPath,

    /// Path to control net model
    #[builder(default = "Default::default()")]
    pub control_net: CLibPath,

    /// Lora model directory
    #[builder(default = "Default::default()", setter(custom))]
    pub lora_model: CLibPath,

    /// Path to embeddings
    #[builder(default = "Default::default()")]
    pub embeddings: CLibPath,


    /// Path to PHOTOMAKER stacked id embeddings
    #[builder(default = "Default::default()")]
    pub stacked_id_embd: CLibPath,

    /// Process vae in tiles to reduce memory usage (default: false)
    #[builder(default = "false")]
    pub vae_tiling: bool,

    /// Number of threads to use during computation (default: 0).
    /// If n_ threads <= 0, then threads will be set to the number of CPU physical cores.
    #[builder(default = "num_cpus::get_physical() as i32", setter(custom))]
    pub n_threads: i32,

    /// Weight type. If not specified, the default is the type of the weight file
    #[builder(default = "WeightType::SD_TYPE_COUNT")]
    pub weight_type: WeightType,


    /// RNG (default: CUDA)
    #[builder(default = "RngFunction::CUDA_RNG")]
    pub rng: RngFunction,

    /// Denoiser sigma schedule (default: DEFAULT)
    #[builder(default = "Schedule::DEFAULT")]
    pub schedule: Schedule,

    /// keep clip in cpu (for low vram) (default: false)
    #[builder(default = "false")]
    pub clip_on_cpu: bool,

    /// Keep controlnet in cpu (for low vram) (default: false)
    #[builder(default = "false")]
    pub control_net_cpu: bool,

    /// Keep vae in cpu (for low vram) (default: false)
    #[builder(default = "false")]
    pub vae_on_cpu: bool,

    /// Use flash attention in the diffusion model (for low vram).
    /// Might lower quality, since it implies converting k and v to f16.
    /// This might crash if it is not supported by the backend.
    #[builder(default = "false")]
    pub flash_attenuation: bool,

    /// --------------------------------------------------------------------------------------------
    /// Suffix that needs to be added to prompt (e.g. lora model)
    #[builder(default = "None", private)]
    pub prompt_suffix: Option<String>,

    /// Path to esrgan model. Upscale images after generate, just RealESRGAN_x4plus_anime_6B supported by now
    #[builder(default = "Default::default()")]
    pub upscale_model: Option<CLibPath>,


}

impl ContextConfigBuilder {
    fn validate(&self) -> Result<(), ContextConfigBuilderError>  {
        self.validate_model()?;

        Ok(())
    }

    pub fn lora_model(&mut self, lora_model: &Path) -> &mut Self {
        let folder = lora_model.parent().unwrap();
        let file_name = lora_model.file_stem().unwrap().to_str().unwrap().to_owned();
        self.prompt_suffix(format!("<lora:{file_name}:1>"));
        self.lora_model = Some(folder.into());
        self
    }

    pub fn n_threads(&mut self, value: i32) -> &mut Self {
        self.n_threads = if value > 0 {
            Some(value)
        } else {
            Some(num_cpus::get_physical() as i32)
        };
        self
    }

    fn validate_model(&self) -> Result<(), ContextConfigBuilderError> {
        self.model
            .as_ref()
            .or(self.diffusion_model.as_ref())
            .map(|_| ())
            .ok_or(ContextConfigBuilderError::UninitializedField(
                "Model OR DiffusionModel must be valorized",
            ))
    }
}