# asciiify

Node.js/TypeScript native addon for converting images to ASCII art. Powered by Rust via napi-rs.

Part of the [asciiify](https://github.com/tomerramk/asciiify) project.

## Install

```bash
npm install asciiify
```

## Usage

```typescript
import { convert, convertBytes, Converter } from "asciiify";

// Convert image file
const art = convert("image.png");
console.log(art);

// With options
const art2 = convert("photo.jpg", { mode: "braille", width: 100 });

// Convert from buffer
import { readFileSync } from "fs";
const data = readFileSync("image.png");
const art3 = convertBytes(data, { mode: "half-block", width: 80 });

// Reusable converter with preset options
const converter = new Converter({ mode: "ascii", width: 120, invert: true });
const art4 = converter.convert("image.png");
const art5 = converter.convertBytes(data);
```

## Options

```typescript
interface ConvertOptions {
  mode?: "ascii" | "half-block" | "braille"; // default: "ascii"
  width?: number; // output width in characters
  height?: number; // output height in characters
  invert?: boolean; // invert brightness
  charset?: string; // custom ASCII ramp (ascii mode only)
}
```

## Output Modes

| Mode | Description |
|------|-------------|
| `"ascii"` | Classic character ramp (default) |
| `"half-block"` | Unicode blocks, 2x vertical resolution |
| `"braille"` | Unicode braille, 4x vertical resolution |

## License

MIT
