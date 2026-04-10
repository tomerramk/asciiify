use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConvertError {
    #[error("failed to load image '{path}': {source}")]
    ImageLoad {
        path: String,
        source: image::ImageError,
    },

    #[error("failed to decode image from bytes: {source}")]
    ImageDecode { source: image::ImageError },

    #[cfg(feature = "video")]
    #[error("video error: {0}")]
    Video(String),
}
