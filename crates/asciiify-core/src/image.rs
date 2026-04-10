use std::path::Path;

use image::{imageops::FilterType, DynamicImage, GrayImage};

use crate::error::ConvertError;
use crate::options::{ConvertOptions, OutputMode};

/// Load an image from a file path.
pub fn load_image(path: impl AsRef<Path>) -> Result<DynamicImage, ConvertError> {
    let path = path.as_ref();
    image::open(path).map_err(|e| ConvertError::ImageLoad {
        path: path.display().to_string(),
        source: e,
    })
}

/// Load an image from an in-memory buffer.
pub fn load_image_from_bytes(data: &[u8]) -> Result<DynamicImage, ConvertError> {
    image::load_from_memory(data).map_err(|e| ConvertError::ImageDecode { source: e })
}

/// Resize and convert the image to grayscale, returning a `GrayImage` sized
/// for the chosen output mode.
///
/// The pixel dimensions of the returned image depend on the mode:
/// - Ascii: `(cols, rows)` — 1 pixel per character
/// - HalfBlock: `(cols, rows * 2)` — 2 vertical pixels per character
/// - Braille: `(cols * 2, rows * 4)` — 2×4 pixels per character
pub fn prepare_image(img: &DynamicImage, opts: &ConvertOptions) -> GrayImage {
    let (cols, rows) = opts.resolve_dimensions(img.width(), img.height());

    let (px_w, px_h) = match opts.mode {
        OutputMode::Ascii => (cols, rows),
        OutputMode::HalfBlock => (cols, rows * 2),
        OutputMode::Braille => (cols * 2, rows * 4),
    };

    let resized = img.resize_exact(px_w, px_h, FilterType::Triangle);
    resized.to_luma8()
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::{GrayImage, Luma};

    fn make_test_image(w: u32, h: u32) -> DynamicImage {
        DynamicImage::ImageLuma8(GrayImage::from_fn(w, h, |x, _| Luma([(x % 256) as u8])))
    }

    #[test]
    fn load_image_nonexistent_file() {
        let result = load_image("nonexistent_image_12345.png");
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("nonexistent_image_12345.png"));
    }

    #[test]
    fn load_image_from_invalid_bytes() {
        let result = load_image_from_bytes(b"not an image");
        assert!(result.is_err());
    }

    #[test]
    fn prepare_image_ascii_dimensions() {
        let img = make_test_image(200, 100);
        let opts = ConvertOptions {
            width: Some(40),
            height: Some(20),
            mode: OutputMode::Ascii,
            ..Default::default()
        };
        let gray = prepare_image(&img, &opts);
        assert_eq!(gray.dimensions(), (40, 20));
    }

    #[test]
    fn prepare_image_half_block_dimensions() {
        let img = make_test_image(200, 100);
        let opts = ConvertOptions {
            width: Some(40),
            height: Some(20),
            mode: OutputMode::HalfBlock,
            ..Default::default()
        };
        let gray = prepare_image(&img, &opts);
        // HalfBlock: cols × (rows * 2)
        assert_eq!(gray.dimensions(), (40, 40));
    }

    #[test]
    fn prepare_image_braille_dimensions() {
        let img = make_test_image(200, 100);
        let opts = ConvertOptions {
            width: Some(40),
            height: Some(20),
            mode: OutputMode::Braille,
            ..Default::default()
        };
        let gray = prepare_image(&img, &opts);
        // Braille: (cols * 2) × (rows * 4)
        assert_eq!(gray.dimensions(), (80, 80));
    }

    #[test]
    fn prepare_image_outputs_grayscale() {
        let img = make_test_image(100, 100);
        let opts = ConvertOptions {
            width: Some(10),
            height: Some(10),
            ..Default::default()
        };
        let gray = prepare_image(&img, &opts);
        // Should have single-channel pixels
        assert!(gray.pixels().count() > 0);
    }
}
