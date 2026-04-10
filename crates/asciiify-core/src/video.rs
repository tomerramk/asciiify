#[cfg(feature = "video")]
use image::{DynamicImage, RgbImage};

#[cfg(feature = "video")]
use crate::error::ConvertError;

/// Iterator that yields video frames as `DynamicImage`.
#[cfg(feature = "video")]
pub struct FrameIterator {
    input_ctx: ffmpeg_next::format::context::Input,
    decoder: ffmpeg_next::codec::decoder::Video,
    stream_index: usize,
    scaler: ffmpeg_next::software::scaling::Context,
    fps: f64,
    width: u32,
    height: u32,
    // Buffer for decoded packets
    packets_finished: bool,
}

#[cfg(feature = "video")]
impl FrameIterator {
    /// Frames per second of the video stream.
    pub fn fps(&self) -> f64 {
        self.fps
    }

    /// Video frame width in pixels.
    pub fn width(&self) -> u32 {
        self.width
    }

    /// Video frame height in pixels.
    pub fn height(&self) -> u32 {
        self.height
    }
}

/// Open a video file and return a `FrameIterator` yielding decoded frames.
#[cfg(feature = "video")]
pub fn extract_frames(path: &str) -> Result<FrameIterator, ConvertError> {
    ffmpeg_next::init().map_err(|e| ConvertError::Video(format!("ffmpeg init: {e}")))?;

    let input_ctx = ffmpeg_next::format::input(&path)
        .map_err(|e| ConvertError::Video(format!("open video '{path}': {e}")))?;

    let stream = input_ctx
        .streams()
        .best(ffmpeg_next::media::Type::Video)
        .ok_or_else(|| ConvertError::Video("no video stream found".into()))?;

    let stream_index = stream.index();

    let rate = stream.avg_frame_rate();
    let fps = if rate.denominator() != 0 {
        rate.numerator() as f64 / rate.denominator() as f64
    } else {
        30.0
    };

    let codec_params = stream.parameters();
    let decoder_ctx = ffmpeg_next::codec::Context::from_parameters(codec_params)
        .map_err(|e| ConvertError::Video(format!("codec context: {e}")))?;
    let decoder = decoder_ctx
        .decoder()
        .video()
        .map_err(|e| ConvertError::Video(format!("video decoder: {e}")))?;

    let width = decoder.width();
    let height = decoder.height();

    let scaler = ffmpeg_next::software::scaling::Context::get(
        decoder.format(),
        width,
        height,
        ffmpeg_next::format::Pixel::RGB24,
        width,
        height,
        ffmpeg_next::software::scaling::Flags::BILINEAR,
    )
    .map_err(|e| ConvertError::Video(format!("scaler init: {e}")))?;

    Ok(FrameIterator {
        input_ctx,
        decoder,
        stream_index,
        scaler,
        fps,
        width,
        height,
        packets_finished: false,
    })
}

#[cfg(feature = "video")]
impl Iterator for FrameIterator {
    type Item = Result<DynamicImage, ConvertError>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            // Try to receive a decoded frame first
            let mut decoded = ffmpeg_next::util::frame::Video::empty();
            if self.decoder.receive_frame(&mut decoded).is_ok() {
                let mut rgb_frame = ffmpeg_next::util::frame::Video::empty();
                if let Err(e) = self.scaler.run(&decoded, &mut rgb_frame) {
                    return Some(Err(ConvertError::Video(format!("scale frame: {e}"))));
                }
                let w = self.width;
                let h = self.height;
                let data = rgb_frame.data(0);
                let stride = rgb_frame.stride(0);

                // Copy row-by-row (stride may differ from width*3)
                let mut buf = Vec::with_capacity((w * h * 3) as usize);
                for row in 0..h as usize {
                    let start = row * stride;
                    let end = start + (w as usize * 3);
                    buf.extend_from_slice(&data[start..end]);
                }

                match RgbImage::from_raw(w, h, buf) {
                    Some(img) => return Some(Ok(DynamicImage::ImageRgb8(img))),
                    None => return Some(Err(ConvertError::Video("frame buffer mismatch".into()))),
                }
            }

            if self.packets_finished {
                return None;
            }

            // Feed the next packet to the decoder
            match self.input_ctx.packets().next() {
                Some((stream, packet)) => {
                    if stream.index() != self.stream_index {
                        continue;
                    }
                    if let Err(e) = self.decoder.send_packet(&packet) {
                        return Some(Err(ConvertError::Video(format!("send packet: {e}"))));
                    }
                }
                None => {
                    // No more packets — flush the decoder
                    self.packets_finished = true;
                    let _ = self.decoder.send_eof();
                }
            }
        }
    }
}
