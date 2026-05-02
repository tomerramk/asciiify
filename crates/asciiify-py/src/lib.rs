use pyo3::prelude::*;

use asciiify_core::{ConvertOptions, OutputMode};

#[cfg(feature = "video")]
use asciiify_core::{convert_image, extract_frames, FrameIterator};

#[cfg(feature = "video")]
use rodio::{buffer::SamplesBuffer, OutputStream, Sink};

fn parse_mode(mode: &str) -> PyResult<OutputMode> {
    match mode.to_lowercase().as_str() {
        "ascii" => Ok(OutputMode::Ascii),
        "half-block" | "halfblock" | "block" => Ok(OutputMode::HalfBlock),
        "braille" => Ok(OutputMode::Braille),
        _ => Err(pyo3::exceptions::PyValueError::new_err(format!(
            "unknown mode '{mode}', expected: ascii, half-block, braille"
        ))),
    }
}

fn build_options(
    mode: &str,
    width: Option<u32>,
    height: Option<u32>,
    invert: bool,
    charset: Option<String>,
) -> PyResult<ConvertOptions> {
    Ok(ConvertOptions {
        width,
        height,
        mode: parse_mode(mode)?,
        invert,
        charset,
    })
}

/// Convert an image file to ASCII art.
#[pyfunction]
#[pyo3(signature = (path, *, mode = "ascii", width = None, height = None, invert = false, charset = None))]
fn convert(
    path: &str,
    mode: &str,
    width: Option<u32>,
    height: Option<u32>,
    invert: bool,
    charset: Option<String>,
) -> PyResult<String> {
    let opts = build_options(mode, width, height, invert, charset)?;
    asciiify_core::convert_image_file(path, &opts)
        .map_err(|e| pyo3::exceptions::PyIOError::new_err(e.to_string()))
}

/// Convert in-memory image bytes to ASCII art.
#[pyfunction]
#[pyo3(signature = (data, *, mode = "ascii", width = None, height = None, invert = false, charset = None))]
fn convert_bytes(
    data: &[u8],
    mode: &str,
    width: Option<u32>,
    height: Option<u32>,
    invert: bool,
    charset: Option<String>,
) -> PyResult<String> {
    let opts = build_options(mode, width, height, invert, charset)?;
    asciiify_core::convert_image_bytes(data, &opts)
        .map_err(|e| pyo3::exceptions::PyIOError::new_err(e.to_string()))
}

/// Reusable converter with preset options.
#[pyclass]
struct Converter {
    opts: ConvertOptions,
}

#[pymethods]
impl Converter {
    #[new]
    #[pyo3(signature = (*, mode = "ascii", width = None, height = None, invert = false, charset = None))]
    fn new(
        mode: &str,
        width: Option<u32>,
        height: Option<u32>,
        invert: bool,
        charset: Option<String>,
    ) -> PyResult<Self> {
        let opts = build_options(mode, width, height, invert, charset)?;
        Ok(Self { opts })
    }

    /// Convert an image file to ASCII art string.
    fn convert(&self, path: &str) -> PyResult<String> {
        asciiify_core::convert_image_file(path, &self.opts)
            .map_err(|e| pyo3::exceptions::PyIOError::new_err(e.to_string()))
    }

    /// Convert in-memory image bytes to ASCII art string.
    fn convert_bytes(&self, data: &[u8]) -> PyResult<String> {
        asciiify_core::convert_image_bytes(data, &self.opts)
            .map_err(|e| pyo3::exceptions::PyIOError::new_err(e.to_string()))
    }
}

/// Iterator over video frames converted to ASCII art strings.
///
/// Usage:
///     for frame in VideoFrames("video.mp4", width=80):
///         print(frame)
#[cfg(feature = "video")]
#[pyclass(unsendable)]
struct VideoFrames {
    frames: FrameIterator,
    opts: ConvertOptions,
}

#[cfg(feature = "video")]
#[pymethods]
impl VideoFrames {
    #[new]
    #[pyo3(signature = (path, *, mode = "ascii", width = None, height = None, invert = false, charset = None))]
    fn new(
        path: &str,
        mode: &str,
        width: Option<u32>,
        height: Option<u32>,
        invert: bool,
        charset: Option<String>,
    ) -> PyResult<Self> {
        let opts = build_options(mode, width, height, invert, charset)?;
        let frames = extract_frames(path)
            .map_err(|e| pyo3::exceptions::PyIOError::new_err(e.to_string()))?;
        Ok(Self { frames, opts })
    }

    /// Frames per second of the video.
    #[getter]
    fn fps(&self) -> f64 {
        self.frames.fps()
    }

    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __next__(&mut self) -> PyResult<Option<String>> {
        match self.frames.next() {
            Some(Ok(img)) => {
                let s = convert_image(&img, &self.opts)
                    .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
                Ok(Some(s))
            }
            Some(Err(e)) => Err(pyo3::exceptions::PyRuntimeError::new_err(e.to_string())),
            None => Ok(None),
        }
    }
}

/// Decode all audio from a video file and play it in a background thread.
/// Returns immediately; audio continues in the background.
#[cfg(feature = "video")]
#[pyfunction]
fn play_audio_async(path: &str) {
    let path = path.to_string();
    std::thread::spawn(move || {
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
    });
}

#[pymodule]
fn _asciiify(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(convert, m)?)?;
    m.add_function(wrap_pyfunction!(convert_bytes, m)?)?;
    m.add_class::<Converter>()?;
    #[cfg(feature = "video")]
    m.add_class::<VideoFrames>()?;
    #[cfg(feature = "video")]
    m.add_function(wrap_pyfunction!(play_audio_async, m)?)?;
    Ok(())
}
