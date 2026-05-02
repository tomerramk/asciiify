# asciiify-cli

Command-line tool for converting images and videos to ASCII art in your terminal.

Part of the [asciiify](https://github.com/tomerramk/asciiify) project.

![Pikachu](/docs/pikachu.png)

## Install

```bash
cargo install asciiify-cli
```

Or download a prebuilt binary from [GitHub Releases](https://github.com/tomerramk/asciiify/releases).

## Building from Source

Requires [Rust](https://rustup.rs/).

```bash
git clone https://github.com/tomerramk/asciiify.git
cd asciiify

# Build the release binary
cargo build --release -p asciiify-cli
# Binary is at target/release/asciiify

# Or install directly into ~/.cargo/bin
cargo install --path crates/asciiify-cli
```

## Usage

```bash
# Basic usage - convert image to ASCII
asciiify path/to/image.png

# Specify output mode
asciiify image.jpg -m braille

# Set dimensions
asciiify image.png -w 80 -H 25

# Invert brightness
asciiify image.png --invert

# Custom ASCII ramp (ascii mode only)
asciiify image.png --charset " .oO@"

# Output to file
asciiify image.png -o output.txt

# Video playback
asciiify video.mp4 --fps 30

# Help
asciiify --help
```

## Output Modes

| Mode       | Flag            | Description                             |
| ---------- | --------------- | --------------------------------------- |
| ASCII      | `-m ascii`      | Classic character ramp (default)        |
| Half-block | `-m half-block` | Unicode blocks, 2x vertical resolution  |
| Braille    | `-m braille`    | Unicode braille, 4x vertical resolution |

## Video Support

Video playback is included by default. FFmpeg is downloaded automatically the first time
a video file is played.

```bash
# Play video in the terminal
asciiify video.mp4 --fps 30

# Override frame rate
asciiify video.mp4 --fps 15

# Save all frames to a file (one frame per page)
asciiify video.mp4 --fps 30 -o frames.txt
```

## License

MIT
