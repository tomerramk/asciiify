pub mod charset;
pub mod converter;
pub mod error;
pub mod image;
pub mod options;
#[cfg(feature = "video")]
pub mod audio;
#[cfg(feature = "video")]
pub mod video;

pub use converter::{convert_image, convert_image_bytes, convert_image_file};
pub use error::ConvertError;
pub use options::{ConvertOptions, OutputMode};

#[cfg(feature = "video")]
pub use audio::{decode_audio, AudioData};
#[cfg(feature = "video")]
pub use video::{extract_frames, FrameIterator};
