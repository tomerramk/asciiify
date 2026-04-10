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
#[cfg(feature = "video")]
pub fn decode_audio(path: &str) -> Result<Option<AudioData>, ConvertError> {
    use ffmpeg_next::{codec, format::sample::Type as SampleType, media, software::resampling};

    ffmpeg_next::init().map_err(|e| ConvertError::Video(format!("ffmpeg init: {e}")))?;

    let mut input_ctx = ffmpeg_next::format::input(&path)
        .map_err(|e| ConvertError::Video(format!("open '{path}': {e}")))?;

    let stream = match input_ctx.streams().best(media::Type::Audio) {
        Some(s) => s,
        None => return Ok(None),
    };
    let stream_index = stream.index();

    let codec_ctx = codec::Context::from_parameters(stream.parameters())
        .map_err(|e| ConvertError::Video(format!("audio codec context: {e}")))?;
    let mut decoder = codec_ctx
        .decoder()
        .audio()
        .map_err(|e| ConvertError::Video(format!("audio decoder: {e}")))?;

    let rate = decoder.rate();
    let channels = decoder.channels() as u16;
    let src_format = decoder.format();
    let src_layout = decoder.channel_layout();
    let dst_format = ffmpeg_next::format::Sample::F32(SampleType::Packed);

    let mut resampler =
        resampling::Context::get(src_format, src_layout, rate, dst_format, src_layout, rate)
            .map_err(|e| ConvertError::Video(format!("audio resampler: {e}")))?;

    let mut all_samples: Vec<f32> = Vec::new();
    let mut packets_done = false;

    loop {
        let mut decoded = ffmpeg_next::util::frame::Audio::empty();
        if decoder.receive_frame(&mut decoded).is_ok() {
            let mut resampled = ffmpeg_next::util::frame::Audio::empty();
            if resampler.run(&decoded, &mut resampled).is_ok() {
                let data = resampled.data(0);
                let n_bytes = (resampled.samples() * channels as usize * 4).min(data.len());
                for chunk in data[..n_bytes].chunks_exact(4) {
                    all_samples.push(f32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]));
                }
            }
            continue;
        }

        if packets_done {
            break;
        }

        match input_ctx.packets().next() {
            Some((stream, packet)) => {
                if stream.index() == stream_index {
                    let _ = decoder.send_packet(&packet);
                }
            }
            None => {
                packets_done = true;
                let _ = decoder.send_eof();
            }
        }
    }

    if all_samples.is_empty() {
        return Ok(None);
    }

    Ok(Some(AudioData {
        samples: all_samples,
        sample_rate: rate,
        channels,
    }))
}
