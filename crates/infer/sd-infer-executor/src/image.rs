use std::io::Cursor;
use std::path::PathBuf;

use anyhow::anyhow;
use base64::{engine::general_purpose, Engine as _};
use image::ImageReader;
use image::{ImageBuffer, Rgb, Rgba};

use sd_infer_executor_sys::sd_image_t;

pub struct ImageWrapper {
    pub width: u32,
    pub height: u32,
    pub channel: u32,
    pub data: Vec<u8>,
}

impl ImageWrapper {
    pub fn get_sd_image_t(&mut self) -> sd_image_t {
        sd_image_t {
            width: self.width,
            height: self.height,
            channel: self.channel,
            data: self.data.as_mut_ptr() as *mut u8,
        }
    }

    pub fn from_path(path: PathBuf) -> anyhow::Result<Self> {
        let image_data = std::fs::read(path)?;

        let image = ImageReader::new(Cursor::new(&image_data))
            .with_guessed_format()?
            .decode()?
            .to_rgb8();

        Ok(ImageWrapper {
            width: image.width(),
            height: image.height(),
            channel: 3,
            data: image.as_raw().to_vec(),
        })
    }
}

#[allow(dead_code)]
pub fn save_png_file(
    path: PathBuf,
    width: u32,
    height: u32,
    channel: u32,
    raw_data: Vec<u8>,
) -> anyhow::Result<()> {
    if channel == 4 {
        let img: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::from_raw(width, height, raw_data)
            .ok_or("Failed to write png")
            .map_err(|e| anyhow!("{:?}", e))?;

        img.save(path)
            .map_err(|_e| anyhow!("Failed to write png"))?;

        Ok(())
    } else if channel == 3 {
        let img: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::from_raw(width, height, raw_data)
            .ok_or("Failed to write png")
            .map_err(|e| anyhow!("{:?}", e))?;

        img.save(path)
            .map_err(|_e| anyhow!("Failed to write png"))?;

        Ok(())
    } else {
        return Err(anyhow!("Failed to save png file"));
    }
}

pub fn save_png_base64(
    width: u32,
    height: u32,
    channel: u32,
    raw_data: Vec<u8>,
) -> anyhow::Result<String> {
    if channel == 4 {
        let img: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::from_raw(width, height, raw_data)
            .ok_or("Failed to write png")
            .map_err(|e| anyhow!("{:?}", e))?;

        let mut buffer = Cursor::new(Vec::new());

        img.write_to(&mut buffer, image::ImageFormat::Png)?;

        let result = base64_image_encode(buffer.get_ref(), "image/png");

        Ok(result)
    } else if channel == 3 {
        let img: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::from_raw(width, height, raw_data)
            .ok_or("Failed to write png")
            .map_err(|e| anyhow!("{:?}", e))?;

        let mut buffer = Cursor::new(Vec::new());

        img.write_to(&mut buffer, image::ImageFormat::Png)?;

        let result = base64_image_encode(buffer.get_ref(), "image/png");

        Ok(result)
    } else {
        return Err(anyhow!("Failed to write png data"));
    }
}

pub fn get_init_image(width: u32, height: u32) -> sd_image_t {
    let mut data = vec![255u8; (width * height * 3) as usize];

    sd_image_t {
        width,
        height,
        channel: 3,
        data: data.as_mut_ptr() as *mut u8,
    }
}

pub fn get_default_mask(width: u32, height: u32) -> sd_image_t {
    let mut data = vec![255u8; (width * height) as usize];

    sd_image_t {
        width,
        height,
        channel: 1,
        data: data.as_mut_ptr() as *mut u8,
    }
}

fn base64_image_encode(input: &[u8], mime: &str) -> String {
    let base64 = general_purpose::STANDARD.encode(input);

    let base64 = format!("data:{};base64,{}", mime, base64);

    return base64;
}
