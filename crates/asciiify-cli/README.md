# asciiify-cli

Command-line tool for converting images and videos to ASCII art in your terminal.

Part of the [asciiify](https://github.com/tomerramk/asciiify) project.

![Pikachu](/docs/pikachu.png)

## Install

```bash
cargo install asciiify-cli
```

Or download a prebuilt binary from [GitHub Releases](https://github.com/tomerramk/asciiify/releases).

## Usage

```bash
# Convert image to ASCII
asciiify image.png

# Braille mode with custom width
asciiify photo.jpg -m braille -w 120

# Half-block mode
asciiify logo.png -m half-block -w 60 -H 20

# Invert brightness
asciiify image.png --invert

# Custom character ramp
asciiify image.png --charset " .oO@"

# Output to file
asciiify image.png -o output.txt

# Video playback (requires video feature)
asciiify video.mp4 --fps 15
```

## Output Modes

| Mode       | Flag            | Description                             |
| ---------- | --------------- | --------------------------------------- |
| ASCII      | `-m ascii`      | Classic character ramp (default)        |
| Half-block | `-m half-block` | Unicode blocks, 2x vertical resolution  |
| Braille    | `-m braille`    | Unicode braille, 4x vertical resolution |

## Video Support

Build with the `video` feature for video playback (requires FFmpeg installed):

```bash
cargo install asciiify-cli --features video
```

## License

MIT
