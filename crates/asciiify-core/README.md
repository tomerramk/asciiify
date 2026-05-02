# asciiify-core

Core Rust library for converting images and videos to ASCII art.

Part of the [asciiify](https://github.com/tomerramk/asciiify) project.

![Pikachu](/docs/pikachu.png)

## Features

- **Three output modes**: classic ASCII characters, Unicode half-blocks, and braille patterns
- **Image support**: PNG, JPEG, GIF, BMP, WebP, TIFF, QOI
- **Video support** (optional `video` feature): frame extraction via FFmpeg.
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

Build note: the `video` feature is opt-in for library users. Enable it in your
`Cargo.toml`:

```toml
asciiify-core = { version = "0.2.0", features = ["video"] }
```

Or when building from source:

```bash
cargo build --features video
```

### From bytes

```rust
use asciiify_core::{convert_image_bytes, ConvertOptions};

let bytes = std::fs::read("image.png")?;
let art = convert_image_bytes(&bytes, &ConvertOptions::default())?;
```

### Video (requires `video` feature)

```rust
use asciiify_core::{extract_frames, convert_image, ConvertOptions};

let opts = ConvertOptions::default();
let mut frames = extract_frames("video.mp4")?;
println!("FPS: {}", frames.fps());
for result in &mut frames {
    let img = result?;
    let art = convert_image(&img, &opts)?;
    println!("{art}");
}
```

## Output Modes

| Mode        | Characters      | Resolution       |
| ----------- | --------------- | ---------------- |
| `Ascii`     | `.:-=+*#%@`     | 1 char per pixel |
| `HalfBlock` | `▀▄█`           | 2x vertical      |
| `Braille`   | Unicode braille | 4x vertical      |

## Building from Source

Requires [Rust](https://rustup.rs/).

```bash
git clone https://github.com/tomerramk/asciiify.git
cd asciiify

# Build (without video)
cargo build -p asciiify-core

# Build with video support
cargo build -p asciiify-core --features video

# Run tests
cargo test -p asciiify-core
```

To use as a local path dependency in another crate:

```toml
[dependencies]
asciiify-core = { path = "/path/to/asciiify/crates/asciiify-core" }
```

## License

MIT
