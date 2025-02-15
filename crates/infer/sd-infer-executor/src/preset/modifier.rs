#![allow(dead_code)]

use crate::config::ContextConfigBuilder;

use super::get_huggingface_model_path;

/// Add the <https://huggingface.co/ximso/RealESRGAN_x4plus_anime_6B> upscaler
pub fn real_esrgan_x4plus_anime_6_b(mut builder: ContextConfigBuilder ) -> anyhow::Result<ContextConfigBuilder> {
    let upscaler_path = get_huggingface_model_path(
        "ximso/RealESRGAN_x4plus_anime_6B/RealESRGAN_x4plus_anime_6B.pth"
    );
    builder.upscale_model(upscaler_path);

    Ok(builder)
}

/// Apply <https://huggingface.co/madebyollin/sdxl-vae-fp16-fix> to avoid black images with xl models
pub fn sdxl_vae_fp16_fix(mut builder: ContextConfigBuilder) -> anyhow::Result<ContextConfigBuilder> {
    let vae_path = get_huggingface_model_path("madebyollin/sdxl-vae-fp16-fix/sdxl.vae.safetensors");
    builder.vae(vae_path);
    Ok(builder)
}

/// Apply <https://huggingface.co/madebyollin/taesd> taesd autoencoder for faster decoding (SD v1/v2)
pub fn taesd(mut builder: ContextConfigBuilder) -> anyhow::Result<ContextConfigBuilder> {
    let taesd_path =
        get_huggingface_model_path("madebyollin/taesd/diffusion_pytorch_model.safetensors");
    builder.taesd(taesd_path);
    Ok(builder)
}

/// Apply <https://huggingface.co/madebyollin/taesdxl> taesd autoencoder for faster decoding (SDXL)
pub fn taesd_xl(mut builder: ContextConfigBuilder) -> anyhow::Result<ContextConfigBuilder> {
    let taesd_path =
        get_huggingface_model_path("madebyollin/taesdxl/diffusion_pytorch_model.safetensors");
    builder.taesd(taesd_path);
    Ok(builder)
}

/// Apply <https://huggingface.co/cqyan/hybrid-sd-tinyvae> taesd autoencoder for faster decoding (SD v1/v2)
pub fn hybrid_taesd(mut builder: ContextConfigBuilder) -> anyhow::Result<ContextConfigBuilder> {
    let taesd_path = get_huggingface_model_path(
        "cqyan/hybrid-sd-tinyvae/diffusion_pytorch_model.safetensors");
    builder.taesd(taesd_path);
    Ok(builder)
}

/// Apply <https://huggingface.co/cqyan/hybrid-sd-tinyvae-xl> taesd autoencoder for faster decoding (SDXL)
pub fn hybrid_taesd_xl(mut builder: ContextConfigBuilder) -> anyhow::Result<ContextConfigBuilder> {
    let taesd_path = get_huggingface_model_path(
        "cqyan/hybrid-sd-tinyvae-xl/diffusion_pytorch_model.safetensors");
    builder.taesd(taesd_path);
    Ok(builder)
}

/// Apply <https://huggingface.co/latent-consistency/lcm-lora-sdv1-5> to reduce inference steps for SD v1 between 2-8
/// cfg_scale 1. 4 steps.
pub fn lcm_lora_sd_1_5(mut builder: ContextConfigBuilder) -> anyhow::Result<ContextConfigBuilder> {
    let lora_path = get_huggingface_model_path(
        "latent-consistency/lcm-lora-sdv1-5/pytorch_lora_weights.safetensors");
    builder.lora_model(&lora_path);

    // Tex2ImageConfig
    //      cfg_scale: 1.
    //      steps: 4


    Ok(builder)
}

/// Apply <https://huggingface.co/latent-consistency/lcm-lora-sdxl> to reduce inference steps for SD v1 between 2-8 (default 8)
/// Enabled [api::SampleMethod::LCM]. cfg_scale 2. 8 steps.
pub fn lcm_lora_sdxl_base_1_0(mut builder: ContextConfigBuilder) -> anyhow::Result<ContextConfigBuilder> {
    let lora_path = get_huggingface_model_path(
        "latent-consistency/lcm-lora-sdxl/pytorch_lora_weights.safetensors");
    builder
        .lora_model(&lora_path);

    // Tex2ImageConfig
    //      cfg_scale: 2.
    //      steps: 8
    //      sampling_method: SampleMethod::LCM


    Ok(builder)
}

/// Apply <https://huggingface.co/comfyanonymous/flux_text_encoders/blob/main/t5xxl_fp8_e4m3fn.safetensors> Fp8 t5xxl text encoder to reduce memory usage
pub fn t5xxl_fp8_flux_1(mut builder: ContextConfigBuilder) -> anyhow::Result<ContextConfigBuilder> {
    let t5xxl_path = get_huggingface_model_path(
        "comfyanonymous/flux_text_encoders/t5xxl_fp8_e4m3fn.safetensors");

    builder.t5xxl(t5xxl_path);

    Ok(builder)
}

/// Apply <https://huggingface.co/comfyanonymous/flux_text_encoders/blob/main/t5xxl_fp16.safetensors>
/// Default for flux_1_dev/schnell
pub fn t5xxl_fp16_flux_1(mut builder: ContextConfigBuilder) -> anyhow::Result<ContextConfigBuilder> {
    let t5xxl_path = get_huggingface_model_path("comfyanonymous/flux_text_encoders/t5xxl_fp16.safetensors");

    builder.t5xxl(t5xxl_path);

    Ok(builder)
}

