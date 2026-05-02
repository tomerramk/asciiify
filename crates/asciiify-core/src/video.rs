#[cfg(feature = "video")]
use image::{DynamicImage, RgbImage};

#[cfg(feature = "video")]
use crate::error::ConvertError;

/// Iterator that yields video frames as `DynamicImage`.
#[cfg(feature = "video")]
pub struct FrameIterator {
    inner: ffmpeg_sidecar::iter::FfmpegIterator,
    fps: f64,
}

#[cfg(feature = "video")]
impl FrameIterator {
    /// Frames per second of the video stream.
    pub fn fps(&self) -> f64 {
        self.fps
    }
}

/// Open a video file and return a `FrameIterator` yielding decoded frames.
#[cfg(feature = "video")]
pub fn extract_frames(path: &str) -> Result<FrameIterator, ConvertError> {
    ffmpeg_sidecar::download::auto_download()
        .map_err(|e| ConvertError::Video(format!("ffmpeg download: {e}")))?;

    let mut child = ffmpeg_sidecar::command::FfmpegCommand::new()
        .input(path)
        .rawvideo()
        .spawn()
        .map_err(|e| ConvertError::Video(format!("spawn ffmpeg: {e}")))?;

    let mut iter = child
        .iter()
        .map_err(|e| ConvertError::Video(format!("ffmpeg iter: {e}")))?;

    // Collect metadata to retrieve the video stream's fps before yielding frames.
    let meta = iter
        .collect_metadata()
        .map_err(|e| ConvertError::Video(format!("ffmpeg metadata: {e}")))?;

    let fps = meta
        .input_streams
        .iter()
        .find_map(|s| s.video_data().filter(|v| v.fps > 0.0).map(|v| v.fps as f64))
        .unwrap_or(30.0);

    Ok(FrameIterator { inner: iter, fps })
}

#[cfg(feature = "video")]
impl Iterator for FrameIterator {
    type Item = Result<DynamicImage, ConvertError>;

    fn next(&mut self) -> Option<Self::Item> {
        use ffmpeg_sidecar::event::FfmpegEvent;
        loop {
            match self.inner.next() {
                Some(FfmpegEvent::OutputFrame(frame)) => {
                    let w = frame.width;
                    let h = frame.height;
                    match RgbImage::from_raw(w, h, frame.data) {
                        Some(img) => return Some(Ok(DynamicImage::ImageRgb8(img))),
                        None => {
                            return Some(Err(ConvertError::Video(
                                "frame buffer size mismatch".into(),
                            )))
                        }
                    }
                }
                Some(FfmpegEvent::Error(e)) => {
                    return Some(Err(ConvertError::Video(e)));
                }
                Some(FfmpegEvent::Done) | None => return None,
                Some(_) => continue,
            }
        }
    }
}
