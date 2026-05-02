# asciiify

Node.js/TypeScript native addon for converting images and videos to ASCII art. Powered by Rust via napi-rs.

Part of the [asciiify](https://github.com/tomerramk/asciiify) project.

![Pikachu](/docs/pikachu.png)

## Install

```bash
npm install @tomerramk/asciiify
```

## Building from Source

Requires [Rust](https://rustup.rs/) and Node.js.

```bash
git clone https://github.com/tomerramk/asciiify.git
cd asciiify/crates/asciiify-js

npm install
npm run build
# Compiled addon is asciiify.*.node

# Link globally so the asciiify CLI resolves to this build
npm link
```

## CLI

Install globally to get the `asciiify` command:

```bash
npm install -g @tomerramk/asciiify

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

## Library Usage

```typescript
import {
  convert,
  convertBytes,
  Converter,
  VideoFrames,
} from "@tomerramk/asciiify";

// Convert image file
const art = convert("image.png");
console.log(art);

// With options
const art2 = convert("image.jpg", { mode: "braille", width: 100 });

// Convert from buffer
import { readFileSync } from "fs";
const data = readFileSync("image.png");
const art3 = convertBytes(data, { mode: "half-block", width: 80 });

// Reusable converter
const converter = new Converter({ mode: "ascii", width: 120, invert: true });
const art4 = converter.convert("image.png");

// Video: iterate frames as ASCII strings
const vf = new VideoFrames("video.mp4", { width: 80 });
console.log("FPS:", vf.fps);
let frame;
while ((frame = vf.nextFrame()) !== null) {
  console.log(frame);
}
```

## Video Support

Video support is included by default. FFmpeg is downloaded automatically on first use.

```js
import { VideoFrames } from "@tomerramk/asciiify";

const vf = new VideoFrames("video.mp4", { width: 80 });
console.log("FPS:", vf.fps);
let frame;
while ((frame = vf.nextFrame()) !== null) {
  console.log(frame);
}
```

Build note: when building from source, video is included in the default build:

```bash
npm run build
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

| Mode           | Description                             |
| -------------- | --------------------------------------- |
| `"ascii"`      | Classic character ramp (default)        |
| `"half-block"` | Unicode blocks, 2x vertical resolution  |
| `"braille"`    | Unicode braille, 4x vertical resolution |

## License

MIT
