use std::path::PathBuf;

use super::modifier::{sdxl_vae_fp16_fix, t5xxl_fp8_flux_1};
use crate::WeightType;

use super::get_huggingface_model_path;
use crate::config::ContextConfigBuilder;

pub fn stable_diffusion_1_4() -> anyhow::Result<ContextConfigBuilder> {
    let model_path =
        get_huggingface_model_path("CompVis/stable-diffusion-v-1-4-original/sd-v1-4.ckpt");

    let mut config = ContextConfigBuilder::default();

    config.model(model_path);

    Ok(config)
}

pub fn stable_diffusion_1_5() -> anyhow::Result<ContextConfigBuilder> {
    let model_path = get_huggingface_model_path(
        "runwayml/stable-diffusion-v1-5/v1-5-pruned-emaonly.safetensors",
    );

    let mut config = ContextConfigBuilder::default();

    config.model(model_path);

    Ok(config)
}

pub fn stable_diffusion_2_1() -> anyhow::Result<ContextConfigBuilder> {
    let model_path = get_huggingface_model_path(
        "stabilityai/stable-diffusion-2-1/v2-1_768-nonema-pruned.safetensors",
    );

    let mut config = ContextConfigBuilder::default();

    config.model(model_path).vae_tiling(true);

    // Tex2ImageConfig
    //      steps: 25
    //      width: 768,
    //      height: 768

    Ok(config)
}

pub fn stable_diffusion_3_medium_fp16() -> anyhow::Result<ContextConfigBuilder> {
    let model_path = get_huggingface_model_path(
        "stabilityai/stable-diffusion-3-medium/sd3_medium_incl_clips_t5xxlfp16.safetensors",
    );

    let mut config = ContextConfigBuilder::default();

    config.model(model_path).vae_tiling(true);

    // Tex2ImageConfig
    //      sampling_method: SampleMethod::EULER
    //      cfg_scale: 4.5
    //      steps: 30
    //      width: 1024,
    //      height: 1024

    Ok(config)
}

pub fn sdxl_base_1_0() -> anyhow::Result<ContextConfigBuilder> {
    let model_path = get_huggingface_model_path(
        "stabilityai/stable-diffusion-xl-base-1.0/sd_xl_base_1.0.safetensors",
    );

    let mut config = ContextConfigBuilder::default();

    config.model(model_path).vae_tiling(true);

    // Tex2ImageConfig
    //      width: 1024,
    //      height: 1024

    sdxl_vae_fp16_fix(config)
}

pub fn flux_1_dev(sd_type: WeightType) -> anyhow::Result<ContextConfigBuilder> {
    let model_path = flux_1_model_weight("dev", sd_type)?;
    let mut builder = flux_1("dev")?;

    builder.diffusion_model(model_path);

    t5xxl_fp8_flux_1(builder)
}

pub fn flux_1_schnell(sd_type: WeightType) -> anyhow::Result<ContextConfigBuilder> {
    let model_path = flux_1_model_weight("schnell", sd_type)?;
    let mut builder = flux_1("schnell")?;

    builder.diffusion_model(model_path);

    t5xxl_fp8_flux_1(builder)
}

fn flux_1_model_weight(model: &str, sd_type: WeightType) -> anyhow::Result<PathBuf> {
    check_flux_type(sd_type);
    let weight_type = flux_type_to_model(sd_type);

    Ok(get_huggingface_model_path(
        format!("city96/FLUX.1-{model}-gguf/flux1-{model}-{weight_type}.gguf").as_str(),
    ))
}

fn flux_1(vae_model: &str) -> anyhow::Result<ContextConfigBuilder> {
    let mut config = ContextConfigBuilder::default();
    let vae_path = get_huggingface_model_path(
        format!("black-forest-labs/FLUX.1-{vae_model}/ae.safetensors").as_str(),
    );

    let clip_l_path =
        get_huggingface_model_path("comfyanonymous/flux_text_encoders/clip_l.safetensors");

    config.vae(vae_path).clip_l(clip_l_path).vae_tiling(true);

    // Tex2ImageConfig
    //      cfg_scale: 1.
    //      sampling_method: SampleMethod::EULER
    //      steps: flux_dev: 28  flux_1_schnell: 4  flux_1_mini: 28
    //      width: 1024,
    //      height: 1024
    //

    Ok(config)
}

fn check_flux_type(sd_type: WeightType) {
    assert!(
        sd_type == WeightType::SD_TYPE_Q2_K
            || sd_type == WeightType::SD_TYPE_Q3_K
            || sd_type == WeightType::SD_TYPE_Q4_0
            || sd_type == WeightType::SD_TYPE_Q4_K
            || sd_type == WeightType::SD_TYPE_Q8_0
    );
}

fn flux_type_to_model(sd_type: WeightType) -> &'static str {
    match sd_type {
        WeightType::SD_TYPE_Q3_K => "q3_k",
        WeightType::SD_TYPE_Q2_K => "q2_k",
        WeightType::SD_TYPE_Q4_0 => "q4_0",
        WeightType::SD_TYPE_Q4_K => "q4_k",
        WeightType::SD_TYPE_Q8_0 => "q8_0",
        _ => "not_supported",
    }
}

pub fn sd_turbo() -> anyhow::Result<ContextConfigBuilder> {
    let model_path = get_huggingface_model_path("stabilityai/sd-turbo/sd_turbo.safetensors");

    let mut config = ContextConfigBuilder::default();

    config.model(model_path);

    // Tex2ImageConfig
    //      guidance: 0.
    //      cfg_scale: 1.
    //      steps: 4

    Ok(config)
}

pub fn sdxl_turbo_1_0_fp16() -> anyhow::Result<ContextConfigBuilder> {
    let model_path =
        get_huggingface_model_path("stabilityai/sdxl-turbo/sd_xl_turbo_1.0_fp16.safetensors");

    let mut config = ContextConfigBuilder::default();

    config.model(model_path);

    // Tex2ImageConfig
    //      guidance: 0.
    //      cfg_scale: 1.
    //      steps: 4

    sdxl_vae_fp16_fix(config)
}

pub fn stable_diffusion_3_5_large_fp16() -> anyhow::Result<ContextConfigBuilder> {
    stable_diffusion_3_5("large", "large")
}

pub fn stable_diffusion_3_5_large_turbo_fp16() -> anyhow::Result<ContextConfigBuilder> {
    stable_diffusion_3_5("large-turbo", "large_turbo")
}

pub fn stable_diffusion_3_5_medium_fp16() -> anyhow::Result<ContextConfigBuilder> {
    stable_diffusion_3_5("medium", "medium")
}

pub fn stable_diffusion_3_5(model: &str, file_model: &str) -> anyhow::Result<ContextConfigBuilder> {
    let model_path = get_huggingface_model_path(
        format!("stabilityai/stable-diffusion-3.5-{model}/sd3.5_{file_model}.safetensors").as_str(),
    );

    let clip_g_path = get_huggingface_model_path(
        "Comfy-Org/stable-diffusion-3.5-fp8/text_encoders/clip_g.safetensors",
    );
    let clip_l_path = get_huggingface_model_path(
        "Comfy-Org/stable-diffusion-3.5-fp8/text_encoders/clip_l.safetensors",
    );
    let t5xxl_path = get_huggingface_model_path(
        "Comfy-Org/stable-diffusion-3.5-fp8/text_encoders/t5xxl_fp16.safetensors",
    );

    let mut config = ContextConfigBuilder::default();

    config
        .diffusion_model(model_path)
        .clip_l(clip_l_path)
        .clip_g(clip_g_path)
        .t5xxl(t5xxl_path)
        .vae_tiling(true);

    // Tex2ImageConfig
    //      cfg_scale: large: 4.4 large-turbo: 0. medium: 4.5
    //      sampling_method: SampleMethod::EULER
    //      steps: large: 28 large-turbo: 4. medium: 40
    //      width: 1024
    //      height: 1024

    Ok(config)
}

pub fn juggernaut_xl_11() -> anyhow::Result<ContextConfigBuilder> {
    let model_path = get_huggingface_model_path(
        "RunDiffusion/Juggernaut-XI-v11/Juggernaut-XI-byRunDiffusion.safetensors",
    );

    let mut config = ContextConfigBuilder::default();

    config.model(model_path).vae_tiling(true);

    // Tex2ImageConfig
    //      sampling_method: SampleMethod::DPM2
    //      steps: 20
    //      steps: guidance
    //      width: 1024
    //      height: 1024

    Ok(config)
}

pub fn flux_1_mini() -> anyhow::Result<ContextConfigBuilder> {
    let model_path = get_huggingface_model_path("TencentARC/flux-mini/flux-mini.safetensors");

    let mut builder = flux_1("dev")?;

    builder.diffusion_model(model_path);

    // Tex2ImageConfig
    //      width: 512
    //      height: 512

    t5xxl_fp8_flux_1(builder)
}
