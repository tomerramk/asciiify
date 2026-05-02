# asciiify

Python library and CLI for converting images and videos to ASCII art. Powered by Rust via PyO3.

Part of the [asciiify](https://github.com/tomerramk/asciiify) project.

![Pikachu](/docs/pikachu.png)

## Install

```bash
pip install asciiify
```

## Building from Source

Requires [Rust](https://rustup.rs/) and [maturin](https://www.maturin.rs/).

```bash
git clone https://github.com/tomerramk/asciiify.git
cd asciiify/crates/asciiify-py

pip install maturin

# Build and install into the active Python environment
maturin develop --release

# Or build a wheel and install it
maturin build --release
pip install ../../target/wheels/asciiify-*.whl
```

## CLI

Installing the package also provides the `asciiify` command:

```bash
# Convert an image
asciiify image.png

# Braille mode, custom width
asciiify image.jpg -m braille -w 100

# Output to file
asciiify image.png -o output.txt

# Play a video
asciiify video.mp4

# Play a video at a specific FPS
asciiify video.mp4 --fps 30

# All options
asciiify --help
```

Or via `python -m`:

```bash
python -m asciiify image.png -m half-block -w 80
```

## Library Usage

```python
import asciiify

# Convert image file
art = asciiify.convert("image.png")
print(art)

# With options
art = asciiify.convert("image.jpg", mode="braille", width=100, height=50)

# Convert from bytes
with open("image.png", "rb") as f:
    data = f.read()
art = asciiify.convert_bytes(data, mode="half-block", width=80)

# Reusable converter
converter = asciiify.Converter(mode="ascii", width=120, invert=True)
art = converter.convert("image.png")

# Video: iterate frames as ASCII strings
from asciiify import VideoFrames

frames = VideoFrames("video.mp4", width=80)
print(f"FPS: {frames.fps}")
for frame in frames:
    print(frame)
```

## Video Support

Video support is included by default. FFmpeg is downloaded automatically on first use.

```python
from asciiify import VideoFrames

frames = VideoFrames("video.mp4", width=80)
print(f"FPS: {frames.fps}")
for frame in frames:
    print(frame)
```

Build note: when building from source, video is included in the default build:

```bash
maturin develop
```

## Output Modes

| Mode           | Description                             |
| -------------- | --------------------------------------- |
| `"ascii"`      | Classic character ramp (default)        |
| `"half-block"` | Unicode blocks, 2x vertical resolution  |
| `"braille"`    | Unicode braille, 4x vertical resolution |

## API

### `convert(path, *, mode="ascii", width=None, height=None, invert=False, charset=None) -> str`

Convert an image file to ASCII art.

### `convert_bytes(data, *, mode="ascii", width=None, height=None, invert=False, charset=None) -> str`

Convert in-memory image bytes to ASCII art.

### `Converter(*, mode="ascii", width=None, height=None, invert=False, charset=None)`

Reusable converter with preset options. Has `.convert(path)` and `.convert_bytes(data)` methods.

## License

MIT
