use thiserror::Error;

#[non_exhaustive]
#[derive(Error, Debug)]
/// Error that can occurs while forwarding models
pub enum DiffusionError {
    #[error("The underling stablediffusion.cpp function returned NULL")]
    Forward,
    #[error("The underling stbi_write_image function returned 0 while saving image {0}/{1})")]
    StoreImages(usize, i32),
    #[error("The underling upsclaer model returned a NULL image")]
    Upscaler,
}