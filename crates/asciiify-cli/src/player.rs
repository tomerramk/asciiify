#[cfg(feature = "video")]
use std::io::{self, Write};
#[cfg(feature = "video")]
use std::time::{Duration, Instant};

#[cfg(feature = "video")]
use crossterm::{cursor, execute, queue, terminal};

#[cfg(feature = "video")]
use asciiify_core::ConvertOptions;

#[cfg(feature = "video")]
use rodio::{buffer::SamplesBuffer, OutputStream, Sink};

/// Decode all audio from `path` and play it back in a background thread via rodio.
/// Returns immediately; the thread keeps the sink alive until playback finishes.
#[cfg(feature = "video")]
fn start_audio(path: &str) -> Option<std::thread::JoinHandle<()>> {
    let path = path.to_string();
    Some(std::thread::spawn(move || {
        let audio = match asciiify_core::decode_audio(&path) {
            Ok(Some(a)) => a,
            _ => return,
        };

        let (_stream, handle) = match OutputStream::try_default() {
            Ok(s) => s,
            Err(_) => return,
        };
        let sink = match Sink::try_new(&handle) {
            Ok(s) => s,
            Err(_) => return,
        };

        sink.append(SamplesBuffer::new(
            audio.channels,
            audio.sample_rate,
            audio.samples,
        ));
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
    mute: bool,
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
    let _audio_thread = if mute { None } else { start_audio(path) };

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
