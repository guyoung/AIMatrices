mod preset_builder;
mod modifier;

use std::path::{Path, PathBuf};
use derive_builder::Builder;

use crate::WeightType;

use crate::config::{ ContextConfig, ContextConfigBuilder, ContextConfigBuilderError };

use preset_builder::{
    flux_1_dev, flux_1_mini, flux_1_schnell, juggernaut_xl_11, sd_turbo, sdxl_base_1_0,
    sdxl_turbo_1_0_fp16, stable_diffusion_1_4, stable_diffusion_1_5, stable_diffusion_2_1,
    stable_diffusion_3_5_large_fp16, stable_diffusion_3_5_large_turbo_fp16,
    stable_diffusion_3_5_medium_fp16, stable_diffusion_3_medium_fp16,
};

#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
/// Models ready to use
pub enum Preset {
    StableDiffusion1_4,
    StableDiffusion1_5,
    /// <https://huggingface.co/stabilityai/stable-diffusion-2-1> model.
    ///  Vae-tiling enabled. 768x768.
    StableDiffusion2_1,
    /// Requires access rights to <https://huggingface.co/stabilityai/stable-diffusion-3-medium> providing a token via [crate::util::set_hf_token]
    /// Vae-tiling enabled. 1024x1024. Enabled [api::SampleMethod::EULER]. 30 steps.
    StableDiffusion3MediumFp16,
    /// Requires access rights to <https://huggingface.co/stabilityai/stable-diffusion-3.5-medium> providing a token via [crate::util::set_hf_token]
    /// Vae-tiling enabled. 1024x1024. Enabled [api::SampleMethod::EULER]. cfg_scale 4.5. 40 steps.
    StableDiffusion3_5MediumFp16,
    /// Requires access rights to <https://huggingface.co/stabilityai/stable-diffusion-3.5-large> providing a token via [crate::util::set_hf_token]
    /// Vae-tiling enabled. 1024x1024. Enabled [api::SampleMethod::EULER]. cfg_scale 4.5. 28 steps.
    StableDiffusion3_5LargeFp16,
    /// Requires access rights to <https://huggingface.co/stabilityai/stable-diffusion-3.5-large-turbo> providing a token via [crate::util::set_hf_token]
    /// Vae-tiling enabled. 1024x1024. Enabled [api::SampleMethod::EULER]. cfg_scale 0. 4 steps.
    StableDiffusion3_5LargeTurboFp16,
    SDXLBase1_0,
    /// cfg_scale 1. guidance 0. 4 steps
    SDTurbo,
    /// cfg_scale 1. guidance 0. 4 steps
    SDXLTurbo1_0Fp16,
    /// Requires access rights to <https://huggingface.co/black-forest-labs/FLUX.1-dev> providing a token via [crate::util::set_hf_token]
    /// Vae-tiling enabled. 1024x1024. Enabled [api::SampleMethod::EULER]. 28 steps.
    Flux1Dev(WeightType),
    /// Requires access rights to <https://huggingface.co/black-forest-labs/FLUX.1-schnell> providing a token via [crate::util::set_hf_token]
    /// Vae-tiling enabled. 1024x1024. Enabled [api::SampleMethod::EULER]. 4 steps.
    Flux1Schnell(WeightType),
    /// A 3.2B param rectified flow transformer distilled from FLUX.1 [dev] https://huggingface.co/TencentARC/flux-mini
    /// Vae-tiling enabled. 512x512. Enabled [api::SampleMethod::EULER]. 28 steps.
    Flux1Mini,
    /// Requires access rights to <https://huggingface.co/RunDiffusion/Juggernaut-XI-v11> providing a token via [crate::util::set_hf_token]
    /// Vae-tiling enabled. 1024x1024. Enabled [api::SampleMethod::DPM2]. guidance 6. 20 steps
    JuggernautXL11,
}

impl Preset {
    fn try_config_builder(self) -> anyhow::Result<ContextConfigBuilder> {
        match self {
            Preset::StableDiffusion1_4 => stable_diffusion_1_4(),
            Preset::StableDiffusion1_5 => stable_diffusion_1_5(),
            Preset::StableDiffusion2_1 => stable_diffusion_2_1(),
            Preset::StableDiffusion3MediumFp16 => stable_diffusion_3_medium_fp16(),
            Preset::SDXLBase1_0 => sdxl_base_1_0(),
            Preset::Flux1Dev(sd_type_t) => flux_1_dev(sd_type_t),
            Preset::Flux1Schnell(sd_type_t) => flux_1_schnell(sd_type_t),
            Preset::SDTurbo => sd_turbo(),
            Preset::SDXLTurbo1_0Fp16 => sdxl_turbo_1_0_fp16(),
            Preset::StableDiffusion3_5LargeFp16 => stable_diffusion_3_5_large_fp16(),
            Preset::StableDiffusion3_5MediumFp16 => stable_diffusion_3_5_medium_fp16(),
            Preset::StableDiffusion3_5LargeTurboFp16 => stable_diffusion_3_5_large_turbo_fp16(),
            Preset::JuggernautXL11 => juggernaut_xl_11(),
            Preset::Flux1Mini => flux_1_mini(),
        }
    }
}

/// Helper functions that modifies the [ConfigBuilder] See [crate::modifier]
pub type Modifier = fn(ContextConfigBuilder) -> anyhow::Result<ContextConfigBuilder>;

#[derive(Debug, Clone, Builder)]
#[builder(
    name = "PresetBuilder",
    setter(into),
    build_fn(name = "internal_build", private, error = "ContextConfigBuilderError")
)]
/// Helper struct for [ConfigBuilder]
pub struct PresetConfig {
    preset: Preset,
    #[builder(private, default = "Vec::new()")]
    modifiers: Vec<fn(ContextConfigBuilder) -> anyhow::Result<ContextConfigBuilder>>,
}

impl PresetBuilder {
    /// Add modifier that will apply in sequence
    pub fn with_modifier(&mut self, f: Modifier) -> &mut Self {
        if self.modifiers.is_none() {
            self.modifiers = Some(Vec::new());
        }
        self.modifiers.as_mut().unwrap().push(f);
        self
    }

    pub fn build(&mut self) -> Result<ContextConfig, ContextConfigBuilderError> {
        let preset = self.internal_build()?;
        let config: ContextConfigBuilder = preset
            .try_into()
            .map_err(|err: anyhow::Error| ContextConfigBuilderError::ValidationError(err.to_string()))?;
        config.build()
    }
}

impl TryFrom<PresetConfig> for ContextConfigBuilder {
    type Error = anyhow::Error;

    fn try_from(value: PresetConfig) -> Result<Self, Self::Error> {
        let mut config_builder = value.preset.try_config_builder()?;
        for m in value.modifiers {
            config_builder = m(config_builder)?;
        }

        Ok(config_builder)
    }
}

pub fn get_huggingface_model_path(file_path: &str) -> PathBuf {
    if let Ok(dir) = std::env::var("AI_MATRICES_AI_MODELS") {
        let path = Path::new(dir.as_str());

        return path.join("huggingface").join(file_path);
    } else {
        let path = Path::new("./app_data/ai-models");

        return path.join("huggingface").join(file_path);
    }
}