#[cfg(feature = "video")]
use crate::error::ConvertError;

/// Decoded audio ready for playback: interleaved f32 samples, sample rate, and channel count.
#[cfg(feature = "video")]
pub struct AudioData {
    pub samples: Vec<f32>,
    pub sample_rate: u32,
    pub channels: u16,
}

/// Decode all audio from a video/audio file into interleaved f32 PCM samples.
/// Returns `None` if the file has no audio stream.
///
/// FFmpeg is downloaded automatically on first use via `ffmpeg-sidecar` — no system
/// installation required.
#[cfg(feature = "video")]
pub fn decode_audio(path: &str) -> Result<Option<AudioData>, ConvertError> {
    ffmpeg_sidecar::download::auto_download()
        .map_err(|e| ConvertError::Video(format!("ffmpeg download: {e}")))?;

    const SAMPLE_RATE: u32 = 44100;
    const CHANNELS: u16 = 2;

    // Spawn ffmpeg to extract audio as raw interleaved f32le PCM at a fixed rate.
    // -vn disables video output; pipe:1 writes to stdout.
    let mut child = ffmpeg_sidecar::command::FfmpegCommand::new()
        .input(path)
        .args(["-vn", "-f", "f32le", "-ar", "44100", "-ac", "2", "pipe:1"])
        .spawn()
        .map_err(|e| ConvertError::Video(format!("spawn ffmpeg audio: {e}")))?;

    let all_bytes: Vec<u8> = child
        .iter()
        .map_err(|e| ConvertError::Video(format!("ffmpeg audio iter: {e}")))?
        .filter_chunks()
        .flatten()
        .collect();

    if all_bytes.is_empty() {
        return Ok(None);
    }

    let samples: Vec<f32> = all_bytes
        .chunks_exact(4)
        .map(|c| f32::from_le_bytes([c[0], c[1], c[2], c[3]]))
        .collect();

    Ok(Some(AudioData {
        samples,
        sample_rate: SAMPLE_RATE,
        channels: CHANNELS,
    }))
}
