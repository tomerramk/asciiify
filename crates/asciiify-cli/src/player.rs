#[cfg(feature = "video")]
use std::io::{self, Write};
#[cfg(feature = "video")]
use std::time::{Duration, Instant};

#[cfg(feature = "video")]
use crossterm::{cursor, execute, queue, terminal};

#[cfg(feature = "video")]
use asciiify_core::ConvertOptions;

#[cfg(feature = "video")]
use ffmpeg_next::{codec, format::sample::Type as SampleType, media, software::resampling};
#[cfg(feature = "video")]
use rodio::{buffer::SamplesBuffer, OutputStream, Sink};

/// Start streaming audio from `path` in a background thread.
/// Decodes in small chunks and appends to the `rodio` sink as data becomes available.
#[cfg(feature = "video")]
fn start_audio(path: &str) -> Option<std::thread::JoinHandle<()>> {
    // Enough samples to fill ~100ms of audio per chunk
    const CHUNK_SAMPLES: usize = 4096;

    let path = path.to_string();
    Some(std::thread::spawn(move || {
        let _ = ffmpeg_next::init();

        let mut input_ctx = match ffmpeg_next::format::input(&path) {
            Ok(c) => c,
            Err(_) => return,
        };
        let stream = match input_ctx.streams().best(media::Type::Audio) {
            Some(s) => s,
            None => return,
        };
        let stream_index = stream.index();
        let codec_ctx = match codec::Context::from_parameters(stream.parameters()) {
            Ok(c) => c,
            Err(_) => return,
        };
        let mut decoder = match codec_ctx.decoder().audio() {
            Ok(d) => d,
            Err(_) => return,
        };

        let rate = decoder.rate();
        let channels = decoder.channels() as u16;
        let src_format = decoder.format();
        let src_layout = decoder.channel_layout();
        let dst_format = ffmpeg_next::format::Sample::F32(SampleType::Packed);

        let mut resampler = match resampling::Context::get(
            src_format, src_layout, rate, dst_format, src_layout, rate,
        ) {
            Ok(r) => r,
            Err(_) => return,
        };

        let (_stream, handle) = match OutputStream::try_default() {
            Ok(s) => s,
            Err(_) => return,
        };
        let sink = match Sink::try_new(&handle) {
            Ok(s) => s,
            Err(_) => return,
        };

        let mut chunk: Vec<f32> = Vec::with_capacity(CHUNK_SAMPLES * channels as usize);
        let mut packets_done = false;

        loop {
            let mut decoded = ffmpeg_next::util::frame::Audio::empty();
            if decoder.receive_frame(&mut decoded).is_ok() {
                let mut resampled = ffmpeg_next::util::frame::Audio::empty();
                if resampler.run(&decoded, &mut resampled).is_ok() {
                    let data = resampled.data(0);
                    let n_bytes = (resampled.samples() * channels as usize * 4).min(data.len());
                    for c in data[..n_bytes].chunks_exact(4) {
                        chunk.push(f32::from_le_bytes([c[0], c[1], c[2], c[3]]));
                    }
                }
                // Append chunk to sink without waiting for it to finish
                if chunk.len() >= CHUNK_SAMPLES * channels as usize {
                    sink.append(SamplesBuffer::new(
                        channels,
                        rate,
                        chunk.drain(..).collect::<Vec<_>>(),
                    ));
                }
                continue;
            }

            if packets_done {
                break;
            }

            match input_ctx.packets().next() {
                Some((s, packet)) => {
                    if s.index() == stream_index {
                        let _ = decoder.send_packet(&packet);
                    }
                }
                None => {
                    packets_done = true;
                    let _ = decoder.send_eof();
                }
            }
        }

        if !chunk.is_empty() {
            sink.append(SamplesBuffer::new(channels, rate, chunk));
        }
        sink.sleep_until_end();
    }))
}

/// Play video in the terminal. This function buffers a few frames in a background
/// thread, and renders frames at wall-clock time (skipping frames when behind).
#[cfg(feature = "video")]
pub fn play_video(
    path: &str,
    opts: &ConvertOptions,
    target_fps: f64,
    output_path: Option<&str>,
) -> Result<(), Box<dyn std::error::Error>> {
    // Peek at fps before spawning the decode thread
    let fps = {
        let probe = asciiify_core::extract_frames(path)?;
        if target_fps > 0.0 {
            target_fps
        } else {
            probe.fps()
        }
    };
    let frame_duration = Duration::from_secs_f64(1.0 / fps);

    if let Some(out_path) = output_path {
        let frames = asciiify_core::extract_frames(path)?;
        let mut file = std::fs::File::create(out_path)?;
        for frame_result in frames {
            let img = frame_result?;
            let art = asciiify_core::convert_image(&img, opts)?;
            write!(file, "{art}\x0C")?;
        }
        eprintln!("Written frames to {out_path}");
        return Ok(());
    }

    // Decode + ASCII-convert frames in a background thread, buffering a few ahead.
    // FrameIterator isn't Send so we open the file fresh inside the thread.
    let opts_bg = opts.clone();
    let path_bg = path.to_string();
    let (tx, rx) = std::sync::mpsc::sync_channel::<Result<String, asciiify_core::ConvertError>>(4);
    std::thread::spawn(move || {
        let frames = match asciiify_core::extract_frames(&path_bg) {
            Ok(f) => f,
            Err(e) => {
                let _ = tx.send(Err(e));
                return;
            }
        };
        for frame_result in frames {
            let item = frame_result.and_then(|img| asciiify_core::convert_image(&img, &opts_bg));
            if tx.send(item).is_err() {
                break; // receiver dropped (user quit)
            }
        }
    });

    // Start audio streaming in parallel — begins playing after first chunk is decoded
    let _audio_thread = start_audio(path);

    // Interactive terminal playback
    let mut stdout = io::BufWriter::new(io::stdout());
    terminal::enable_raw_mode()?;
    execute!(stdout, terminal::EnterAlternateScreen, cursor::Hide)?;

    let result = (|| -> Result<(), Box<dyn std::error::Error>> {
        let playback_start = Instant::now();
        let mut frame_index: u64 = 0;

        loop {
            let art = match rx.recv() {
                Ok(Ok(a)) => a,
                Ok(Err(e)) => return Err(e.into()),
                Err(_) => break, // decoder finished
            };

            // When this frame should appear on screen
            let target_time = Duration::from_secs_f64(frame_index as f64 / fps);
            let elapsed = playback_start.elapsed();

            frame_index += 1;

            // Skip rendering if we're already behind by more than one frame
            if elapsed > target_time + frame_duration {
                continue;
            }

            execute!(stdout, cursor::MoveTo(0, 0))?;
            // Write each line at its exact row position — no full-screen clear,
            // so there is no blank flash between frames.
            for (row, line) in art.split('\n').enumerate() {
                queue!(stdout, cursor::MoveTo(0, row as u16))?;
                write!(stdout, "{line}")?;
            }
            stdout.flush()?;

            // Sleep the remaining time until the *next* frame is due,
            // polling for keypress so we stay responsive
            let next_frame_time = Duration::from_secs_f64(frame_index as f64 / fps);
            let elapsed = playback_start.elapsed();
            let wait = next_frame_time.saturating_sub(elapsed);

            if crossterm::event::poll(wait)? {
                if let crossterm::event::Event::Key(key) = crossterm::event::read()? {
                    use crossterm::event::KeyCode;
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => break,
                        KeyCode::Char('c')
                            if key
                                .modifiers
                                .contains(crossterm::event::KeyModifiers::CONTROL) =>
                        {
                            break
                        }
                        _ => {}
                    }
                }
            }
        }
        Ok(())
    })();

    execute!(stdout, cursor::Show, terminal::LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;

    result
}
