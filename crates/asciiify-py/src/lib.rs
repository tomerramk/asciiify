use pyo3::prelude::*;

use asciiify_core::{ConvertOptions, OutputMode};

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

#[pymodule]
fn asciiify(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(convert, m)?)?;
    m.add_function(wrap_pyfunction!(convert_bytes, m)?)?;
    m.add_class::<Converter>()?;
    Ok(())
}
