# asciiify

Python library for converting images to ASCII art. Powered by Rust via PyO3.

Part of the [asciiify](https://github.com/tomerramk/asciiify) project.

## Install

```bash
pip install asciiify
```

## Usage

```python
import asciiify

# Convert image file
print(asciiify.convert("image.png"))

# With options
art = asciiify.convert("photo.jpg", mode="braille", width=100, height=50)

# Convert from bytes
with open("image.png", "rb") as f:
    art = asciiify.convert_bytes(f.read(), mode="half-block", width=80)

# Reusable converter with preset options
converter = asciiify.Converter(mode="ascii", width=120, invert=True)
art = converter.convert("image.png")
art = converter.convert_bytes(data)
```

## Output Modes

| Mode | Description |
|------|-------------|
| `"ascii"` | Classic character ramp (default) |
| `"half-block"` | Unicode blocks, 2x vertical resolution |
| `"braille"` | Unicode braille, 4x vertical resolution |

## API

### `convert(path, *, mode="ascii", width=None, height=None, invert=False, charset=None) -> str`

Convert an image file to ASCII art.

### `convert_bytes(data, *, mode="ascii", width=None, height=None, invert=False, charset=None) -> str`

Convert in-memory image bytes to ASCII art.

### `Converter(*, mode="ascii", width=None, height=None, invert=False, charset=None)`

Reusable converter with preset options. Has `.convert(path)` and `.convert_bytes(data)` methods.

## License

MIT
