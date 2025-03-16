use std::ffi::c_void;
use std::path::PathBuf;
use std::ptr::null;
use std::slice;

use libc::free;

use sd_infer_executor_sys::sd_ctx_t;
use sd_infer_executor_sys::sd_image_t;
use sd_infer_executor_sys::upscaler_ctx_t;
use sd_infer_executor_sys::{free_sd_ctx, free_upscaler_ctx, new_sd_ctx, new_upscaler_ctx};

use crate::clib::{CLibPath, CLibString};
use crate::config::{ContextConfig, Txt2imgConfig};
use crate::error::DiffusionError;

#[derive(Debug, Clone)]
pub struct SdContext(*mut sd_ctx_t);

impl SdContext {
    pub fn new(config: ContextConfig, vae_decode_only: bool) -> SdContext {
        SdContext(unsafe { build_sd_ctx(config, vae_decode_only) })
    }

    pub fn free(&self) {
        unsafe {
            //Clean-up CTX section
            free_sd_ctx(self.0);
        }
    }

    pub fn txt2img(
        &mut self,
        prompt: String,
        negative_prompt: Option<String>,
        mut config: Txt2imgConfig,
        context_config: &ContextConfig,
    ) -> Result<Vec<String>, DiffusionError> {
        unsafe {
            let prompt: CLibString = match &context_config.prompt_suffix {
                Some(suffix) => format!("{} {suffix}", &prompt),
                None => prompt.clone(),
            }
            .into();

            let negative_prompt: CLibString =
                negative_prompt.unwrap_or_else(|| "".to_string()).into();

            let images = {
                let slice = sd_infer_executor_sys::txt2img(
                    self.0,
                    prompt.as_ptr(),
                    negative_prompt.as_ptr(),
                    config.clip_skip as i32,
                    config.cfg_scale,
                    config.guidance,
                    config.eta,
                    config.width,
                    config.height,
                    config.sampling_method,
                    config.steps,
                    config.seed,
                    config.batch_count,
                    null(),
                    config.control_strength,
                    config.style_ratio,
                    config.normalize_input,
                    config.input_id_images.as_ptr(),
                    config.skip_layer.as_mut_ptr(),
                    config.skip_layer.len(),
                    config.slg_scale,
                    config.skip_layer_start,
                    config.skip_layer_end,
                );

                if slice.is_null() {
                    return Err(DiffusionError::Forward);
                }

                let images = get_images(slice, config, context_config);

                //Clean-up slice section
                free(slice as *mut c_void);

                images
            };

            images
        }
    }

    pub fn img2img(
        &mut self,
        // mut init_image: crate::image::ImageWrapper,
        // mask_image: Option<crate::image::ImageWrapper>,
        prompt: String,
        negative_prompt: Option<String>,
        mut config: Txt2imgConfig,
        context_config: &ContextConfig,
    ) -> Result<Vec<String>, DiffusionError> {
        unsafe {
            let prompt: CLibString = match &context_config.prompt_suffix {
                Some(suffix) => format!("{} {suffix}", &prompt),
                None => prompt.clone(),
            }
            .into();

            let negative_prompt: CLibString =
                negative_prompt.unwrap_or_else(|| "".to_string()).into();

            let images = {
                // let  init_image = init_image.get_sd_image_t();
                // println!("init_image: {:?}", init_image);

                let slice = sd_infer_executor_sys::img2img(
                    self.0,
                    crate::image::get_init_image(config.width as u32, config.height as u32),
                    crate::image::get_default_mask(config.width as u32, config.height as u32),
                    prompt.as_ptr(),
                    negative_prompt.as_ptr(),
                    config.clip_skip as i32,
                    config.cfg_scale,
                    config.guidance,
                    config.eta,
                    config.width,
                    config.height,
                    config.sampling_method,
                    config.steps,
                    config.strength,
                    config.seed,
                    config.batch_count,
                    null(),
                    config.control_strength,
                    config.style_ratio,
                    config.normalize_input,
                    config.input_id_images.as_ptr(),
                    config.skip_layer.as_mut_ptr(),
                    config.skip_layer.len(),
                    config.slg_scale,
                    config.skip_layer_start,
                    config.skip_layer_end,
                );

                if slice.is_null() {
                    return Err(DiffusionError::Forward);
                }

                let images = get_images(slice, config, context_config);

                //Clean-up slice section
                free(slice as *mut c_void);

                images
            };

            images
        }
    }
}

#[derive(Debug, Clone)]
pub struct SDUpscalerContext(*mut upscaler_ctx_t);

impl SDUpscalerContext {
    pub fn new(esrgan_path: PathBuf, n_threads: i32) -> SDUpscalerContext {
        SDUpscalerContext(unsafe { build_sd_upscaler_ctx(esrgan_path, n_threads) })
    }

    pub fn free(&self) {
        unsafe {
            //Clean-up CTX section
            free_upscaler_ctx(self.0);
        }
    }

    unsafe fn upscale(
        &self,
        data: sd_image_t,
        upscale_repeats: i32,
    ) -> Result<sd_image_t, DiffusionError> {
        let upscale_factor = 4; // unused for RealESRGAN_x4plus_anime_6B.pth
        let mut current_image = data;
        for _ in 0..upscale_repeats {
            let upscaled_image =
                sd_infer_executor_sys::upscale(self.0, current_image, upscale_factor);

            if upscaled_image.data.is_null() {
                return Err(DiffusionError::Upscaler);
            }

            free(current_image.data as *mut c_void);
            current_image = upscaled_image;
        }
        Ok(current_image)
    }
}

unsafe fn build_sd_ctx(config: ContextConfig, vae_decode_only: bool) -> *mut sd_ctx_t {
    new_sd_ctx(
        config.model.as_ptr(),
        config.clip_l.as_ptr(),
        config.clip_g.as_ptr(),
        config.t5xxl.as_ptr(),
        config.diffusion_model.as_ptr(),
        config.vae.as_ptr(),
        config.taesd.as_ptr(),
        config.control_net.as_ptr(),
        config.lora_model.as_ptr(),
        config.embeddings.as_ptr(),
        config.stacked_id_embd.as_ptr(),
        vae_decode_only,
        config.vae_tiling,
        true, //free_params_immediately 是否立即释放参数
        config.n_threads,
        config.weight_type,
        config.rng,
        config.schedule,
        config.clip_on_cpu,
        config.control_net_cpu,
        config.vae_on_cpu,
        config.flash_attenuation,
    )
}

unsafe fn build_sd_upscaler_ctx(esrgan_path: PathBuf, n_threads: i32) -> *mut upscaler_ctx_t {
    let esrgan_path: CLibPath = esrgan_path.into();

    new_upscaler_ctx(esrgan_path.as_ptr(), n_threads)
}

fn output_files(path: PathBuf, batch_size: i32) -> Vec<CLibPath> {
    if batch_size == 1 {
        vec![path.into()]
    } else {
        (1..=batch_size)
            .map(|id| path.join(format!("output_{id}.png")).into())
            .collect()
    }
}

fn get_images(
    slice: *mut sd_image_t,
    config: Txt2imgConfig,
    context_config: &ContextConfig,
) -> Result<Vec<String>, DiffusionError> {
    let files = output_files(config.output, config.batch_count);

    let mut images: Vec<String> = Vec::new();

    unsafe {
        for (id, (img, _path)) in slice::from_raw_parts(slice, config.batch_count as usize)
            .iter()
            .zip(files)
            .enumerate()
        {
            let img = {
                if context_config.upscale_model.is_some() && config.upscale_repeats == 0 {
                    let upscaler_context = SDUpscalerContext::new(
                        context_config.upscale_model.clone().unwrap().into(),
                        context_config.n_threads,
                    );

                    let img = upscaler_context.upscale(*img, config.upscale_repeats);

                    upscaler_context.free();

                    img
                } else {
                    Ok(*img)
                }
            };

            match img {
                Ok(img) => {
                    let img_data = slice::from_raw_parts_mut(
                        img.data,
                        (img.width * img.height * img.channel) as usize,
                    )
                    .to_vec();

                    let img = crate::image::save_png_base64(
                        img.width as u32,
                        img.height as u32,
                        img.channel as u32,
                        img_data,
                    );

                    if img.is_err() {
                        return Err(DiffusionError::StoreImages(id, config.batch_count));
                    }

                    images.push(img.unwrap());
                }
                Err(err) => {
                    return Err(err);
                }
            }
        }
    }

    Ok(images)
}
