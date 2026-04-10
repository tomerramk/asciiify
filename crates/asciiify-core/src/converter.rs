use image::DynamicImage;

use crate::charset::{block_to_braille, brightness_to_ascii, brightness_to_half_block};
use crate::error::ConvertError;
use crate::image::{load_image, load_image_from_bytes, prepare_image};
use crate::options::{ConvertOptions, OutputMode};

const DEFAULT_THRESHOLD: u8 = 128;

/// Convert an image file to an ASCII art string.
pub fn convert_image_file(
    path: impl AsRef<std::path::Path>,
    opts: &ConvertOptions,
) -> Result<String, ConvertError> {
    let img = load_image(path)?;
    convert_image(&img, opts)
}

/// Convert raw image bytes to an ASCII art string.
pub fn convert_image_bytes(
    data: &[u8],
    opts: &ConvertOptions,
) -> Result<String, ConvertError> {
    let img = load_image_from_bytes(data)?;
    convert_image(&img, opts)
}

/// Convert a `DynamicImage` to an ASCII art string.
pub fn convert_image(
    img: &DynamicImage,
    opts: &ConvertOptions,
) -> Result<String, ConvertError> {
    let gray = prepare_image(img, opts);
    let (px_w, px_h) = gray.dimensions();

    match opts.mode {
        OutputMode::Ascii => Ok(render_ascii(&gray, px_w, px_h, opts)),
        OutputMode::HalfBlock => Ok(render_half_block(&gray, px_w, px_h, opts)),
        OutputMode::Braille => Ok(render_braille(&gray, px_w, px_h, opts)),
    }
}

fn render_ascii(
    gray: &image::GrayImage,
    width: u32,
    height: u32,
    opts: &ConvertOptions,
) -> String {
    let ramp = opts.ascii_ramp();
    let mut out = String::with_capacity((width as usize + 1) * height as usize);

    for y in 0..height {
        for x in 0..width {
            let luma = gray.get_pixel(x, y).0[0];
            out.push(brightness_to_ascii(luma, ramp, opts.invert));
        }
        if y + 1 < height {
            out.push('\n');
        }
    }
    out
}

fn render_half_block(
    gray: &image::GrayImage,
    width: u32,
    height: u32,
    opts: &ConvertOptions,
) -> String {
    // height is the pixel height (rows * 2). We iterate in pairs of rows.
    let char_rows = height / 2;
    let mut out = String::with_capacity((width as usize + 1) * char_rows as usize);

    for row in 0..char_rows {
        let y_top = row * 2;
        let y_bot = y_top + 1;
        for x in 0..width {
            let top = gray.get_pixel(x, y_top).0[0];
            let bot = if y_bot < height {
                gray.get_pixel(x, y_bot).0[0]
            } else {
                0
            };
            out.push(brightness_to_half_block(top, bot, DEFAULT_THRESHOLD, opts.invert));
        }
        if row + 1 < char_rows {
            out.push('\n');
        }
    }
    out
}

fn render_braille(
    gray: &image::GrayImage,
    width: u32,
    height: u32,
    opts: &ConvertOptions,
) -> String {
    // width = cols*2, height = rows*4. Iterate in 2×4 blocks.
    let char_cols = width / 2;
    let char_rows = height / 4;
    let mut out = String::with_capacity((char_cols as usize + 1) * char_rows as usize);

    for row in 0..char_rows {
        for col in 0..char_cols {
            let bx = col * 2;
            let by = row * 4;
            let mut block = [[0u8; 4]; 2];
            for dx in 0..2u32 {
                for dy in 0..4u32 {
                    let px = bx + dx;
                    let py = by + dy;
                    if px < width && py < height {
                        block[dx as usize][dy as usize] = gray.get_pixel(px, py).0[0];
                    }
                }
            }
            out.push(block_to_braille(&block, DEFAULT_THRESHOLD, opts.invert));
        }
        if row + 1 < char_rows {
            out.push('\n');
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::{GrayImage, Luma};

    fn make_gray_image(width: u32, height: u32, value: u8) -> DynamicImage {
        let img = GrayImage::from_fn(width, height, |_, _| Luma([value]));
        DynamicImage::ImageLuma8(img)
    }

    #[test]
    fn ascii_output_dimensions() {
        let img = make_gray_image(100, 100, 128);
        let opts = ConvertOptions {
            width: Some(10),
            height: Some(5),
            mode: OutputMode::Ascii,
            ..Default::default()
        };
        let result = convert_image(&img, &opts).unwrap();
        let lines: Vec<&str> = result.lines().collect();
        assert_eq!(lines.len(), 5);
        assert!(lines.iter().all(|l| l.chars().count() == 10));
    }

    #[test]
    fn half_block_output_dimensions() {
        let img = make_gray_image(100, 100, 128);
        let opts = ConvertOptions {
            width: Some(10),
            height: Some(5),
            mode: OutputMode::HalfBlock,
            ..Default::default()
        };
        let result = convert_image(&img, &opts).unwrap();
        let lines: Vec<&str> = result.lines().collect();
        assert_eq!(lines.len(), 5);
        assert!(lines.iter().all(|l| l.chars().count() == 10));
    }

    #[test]
    fn braille_output_dimensions() {
        let img = make_gray_image(100, 100, 128);
        let opts = ConvertOptions {
            width: Some(10),
            height: Some(5),
            mode: OutputMode::Braille,
            ..Default::default()
        };
        let result = convert_image(&img, &opts).unwrap();
        let lines: Vec<&str> = result.lines().collect();
        assert_eq!(lines.len(), 5);
        assert!(lines.iter().all(|l| l.chars().count() == 10));
    }

    #[test]
    fn all_black_ascii() {
        let img = make_gray_image(10, 10, 0);
        let opts = ConvertOptions {
            width: Some(10),
            height: Some(5),
            mode: OutputMode::Ascii,
            ..Default::default()
        };
        let result = convert_image(&img, &opts).unwrap();
        // All space characters (darkest in default ramp)
        assert!(result.chars().all(|c| c == ' ' || c == '\n'));
    }

    #[test]
    fn all_white_ascii() {
        let img = make_gray_image(10, 10, 255);
        let opts = ConvertOptions {
            width: Some(10),
            height: Some(5),
            mode: OutputMode::Ascii,
            ..Default::default()
        };
        let result = convert_image(&img, &opts).unwrap();
        // All @ characters (brightest in default ramp)
        assert!(result.chars().all(|c| c == '@' || c == '\n'));
    }

    #[test]
    fn invert_flips_output() {
        let img = make_gray_image(10, 10, 0);
        let normal = convert_image(&img, &ConvertOptions {
            width: Some(10),
            height: Some(5),
            mode: OutputMode::Ascii,
            invert: false,
            ..Default::default()
        }).unwrap();
        let inverted = convert_image(&img, &ConvertOptions {
            width: Some(10),
            height: Some(5),
            mode: OutputMode::Ascii,
            invert: true,
            ..Default::default()
        }).unwrap();
        assert_ne!(normal, inverted);
    }

    #[test]
    fn custom_charset() {
        let img = make_gray_image(10, 10, 255);
        let opts = ConvertOptions {
            width: Some(10),
            height: Some(5),
            mode: OutputMode::Ascii,
            charset: Some(" X".to_string()),
            ..Default::default()
        };
        let result = convert_image(&img, &opts).unwrap();
        assert!(result.chars().all(|c| c == 'X' || c == '\n'));
    }

    #[test]
    fn all_black_half_block() {
        let img = make_gray_image(10, 10, 0);
        let opts = ConvertOptions {
            width: Some(10),
            height: Some(5),
            mode: OutputMode::HalfBlock,
            ..Default::default()
        };
        let result = convert_image(&img, &opts).unwrap();
        assert!(result.chars().all(|c| c == ' ' || c == '\n'));
    }

    #[test]
    fn all_white_half_block() {
        let img = make_gray_image(10, 10, 255);
        let opts = ConvertOptions {
            width: Some(10),
            height: Some(5),
            mode: OutputMode::HalfBlock,
            ..Default::default()
        };
        let result = convert_image(&img, &opts).unwrap();
        assert!(result.chars().all(|c| c == '\u{2588}' || c == '\n'));
    }

    #[test]
    fn all_black_braille() {
        let img = make_gray_image(10, 10, 0);
        let opts = ConvertOptions {
            width: Some(10),
            height: Some(5),
            mode: OutputMode::Braille,
            ..Default::default()
        };
        let result = convert_image(&img, &opts).unwrap();
        assert!(result.chars().all(|c| c == '\u{2800}' || c == '\n'));
    }

    #[test]
    fn all_white_braille() {
        let img = make_gray_image(10, 10, 255);
        let opts = ConvertOptions {
            width: Some(10),
            height: Some(5),
            mode: OutputMode::Braille,
            ..Default::default()
        };
        let result = convert_image(&img, &opts).unwrap();
        assert!(result.chars().all(|c| c == '\u{28FF}' || c == '\n'));
    }

    #[test]
    fn convert_image_bytes_valid_png() {
        // Create a minimal valid PNG in memory
        let img = make_gray_image(4, 4, 128);
        let mut buf = Vec::new();
        img.write_to(
            &mut std::io::Cursor::new(&mut buf),
            image::ImageFormat::Png,
        ).unwrap();

        let opts = ConvertOptions {
            width: Some(4),
            height: Some(2),
            mode: OutputMode::Ascii,
            ..Default::default()
        };
        let result = convert_image_bytes(&buf, &opts).unwrap();
        assert_eq!(result.lines().count(), 2);
    }

    #[test]
    fn convert_image_bytes_invalid_data() {
        let opts = ConvertOptions::default();
        assert!(convert_image_bytes(b"not a real image", &opts).is_err());
    }
}
