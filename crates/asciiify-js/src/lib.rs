use napi::bindgen_prelude::*;
use napi_derive::napi;

use asciiify_core::{ConvertOptions, OutputMode};

fn parse_mode(mode: &str) -> Result<OutputMode> {
    match mode.to_lowercase().as_str() {
        "ascii" => Ok(OutputMode::Ascii),
        "half-block" | "halfblock" | "block" => Ok(OutputMode::HalfBlock),
        "braille" => Ok(OutputMode::Braille),
        _ => Err(Error::from_reason(format!(
            "unknown mode '{mode}', expected: ascii, half-block, braille"
        ))),
    }
}

fn build_options(opts: Option<ConvertOptionsJs>) -> Result<ConvertOptions> {
    let opts = opts.unwrap_or_default();
    Ok(ConvertOptions {
        width: opts.width,
        height: opts.height,
        mode: parse_mode(opts.mode.as_deref().unwrap_or("ascii"))?,
        invert: opts.invert.unwrap_or(false),
        charset: opts.charset,
    })
}

/// Options for ASCII art conversion.
#[napi(object)]
#[derive(Default)]
pub struct ConvertOptionsJs {
    /// Output mode: "ascii", "half-block", or "braille"
    pub mode: Option<String>,
    /// Output width in characters
    pub width: Option<u32>,
    /// Output height in characters
    pub height: Option<u32>,
    /// Invert brightness
    pub invert: Option<bool>,
    /// Custom ASCII character ramp (ascii mode only)
    pub charset: Option<String>,
}

/// Convert an image file to ASCII art.
#[napi]
pub fn convert(path: String, options: Option<ConvertOptionsJs>) -> Result<String> {
    let opts = build_options(options)?;
    asciiify_core::convert_image_file(&path, &opts)
        .map_err(|e| Error::from_reason(e.to_string()))
}

/// Convert in-memory image bytes to ASCII art.
#[napi]
pub fn convert_bytes(data: Buffer, options: Option<ConvertOptionsJs>) -> Result<String> {
    let opts = build_options(options)?;
    asciiify_core::convert_image_bytes(&data, &opts)
        .map_err(|e| Error::from_reason(e.to_string()))
}

/// Reusable converter with preset options.
#[napi]
pub struct Converter {
    opts: ConvertOptions,
}

#[napi]
impl Converter {
    #[napi(constructor)]
    pub fn new(options: Option<ConvertOptionsJs>) -> Result<Self> {
        let opts = build_options(options)?;
        Ok(Self { opts })
    }

    /// Convert an image file to ASCII art string.
    #[napi]
    pub fn convert(&self, path: String) -> Result<String> {
        asciiify_core::convert_image_file(&path, &self.opts)
            .map_err(|e| Error::from_reason(e.to_string()))
    }

    /// Convert in-memory image bytes to ASCII art string.
    #[napi]
    pub fn convert_bytes(&self, data: Buffer) -> Result<String> {
        asciiify_core::convert_image_bytes(&data, &self.opts)
            .map_err(|e| Error::from_reason(e.to_string()))
    }
}
