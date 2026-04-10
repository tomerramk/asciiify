# asciiify-core

Core Rust library for converting images to ASCII art.

Part of the [asciiify](https://github.com/tomerramk/asciiify) project.

## Features

- **Three output modes**: classic ASCII characters, Unicode half-blocks, and braille patterns
- **Image support**: PNG, JPEG, GIF, BMP, WebP, TIFF, QOI
- **Video support** (optional `video` feature): frame extraction via FFmpeg
- Zero runtime overhead — pure Rust

## Usage

```rust
use asciiify_core::{convert_image_file, ConvertOptions, OutputMode};

let opts = ConvertOptions {
    width: Some(80),
    mode: OutputMode::Braille,
    ..Default::default()
};

let art = convert_image_file("image.png", &opts)?;
println!("{art}");
```

### From bytes

```rust
use asciiify_core::{convert_image_bytes, ConvertOptions};

let bytes = std::fs::read("image.png")?;
let art = convert_image_bytes(&bytes, &ConvertOptions::default())?;
```

## Output Modes

| Mode | Characters | Resolution |
|------|-----------|------------|
| `Ascii` | `.:-=+*#%@` | 1 char per pixel |
| `HalfBlock` | `▀▄█` | 2x vertical |
| `Braille` | Unicode braille | 4x vertical |

## License

MIT
