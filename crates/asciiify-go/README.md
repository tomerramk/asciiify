# asciiify-go

Go package for converting images and videos to ASCII art. Powered by Rust via C FFI.

Part of the [asciiify](https://github.com/tomerramk/asciiify) project.

![Pikachu](/docs/pikachu.png)

## Install

```bash
go get github.com/tomerramk/asciiify/crates/asciiify-go
```

## Building from Source

Requires [Rust](https://rustup.rs/) and Go. The Go package links against a shared library built from the Rust crate.

```bash
git clone https://github.com/tomerramk/asciiify.git
cd asciiify

# Build the shared library
cargo build --release -p asciiify-go
```

Then set CGO flags to point at the built library before running `go build` or `go install`:

```bash
# Linux / macOS
export CGO_LDFLAGS="-L$(pwd)/target/release -lasciiify_ffi"
export LD_LIBRARY_PATH="$(pwd)/target/release"

# Windows (PowerShell)
$env:CGO_LDFLAGS = "-L$((Get-Location).Path)\target\release -lasciiify_ffi"

# Build and install the CLI
go install ./crates/asciiify-go/cmd/asciiify/
```

Video support is included by default.

## CLI

Install the `asciiify` command:

```bash
go install github.com/tomerramk/asciiify/crates/asciiify-go/cmd/asciiify@latest

# Convert an image
asciiify image.png

# Braille mode, custom width
asciiify -mode braille -w 100 image.jpg

# Output to file
asciiify -o output.txt image.png

# Play a video
asciiify video.mp4

# Play a video at a specific FPS
asciiify -fps 30 video.mp4

# All options
asciiify -help
```

## Library Usage

```go
package main

import (
	"fmt"
	"os"

	asciiify "github.com/tomerramk/asciiify/crates/asciiify-go"
)

func main() {
	// Convert image file
	art, err := asciiify.ConvertFile("image.png", nil)
	if err != nil {
		fmt.Fprintln(os.Stderr, err)
		os.Exit(1)
	}
	fmt.Println(art)

	// With options
	art2, _ := asciiify.ConvertFile("image.jpg", &asciiify.Options{
		Mode:  "braille",
		Width: 100,
	})
	fmt.Println(art2)

	// Convert from bytes
	data, _ := os.ReadFile("image.png")
	art3, _ := asciiify.ConvertBytes(data, &asciiify.Options{
		Mode:  "half-block",
		Width: 80,
	})
	fmt.Println(art3)

	// Video: iterate frames as ASCII strings
	v, err := asciiify.OpenVideo("video.mp4", &asciiify.Options{Width: 80})
	if err != nil {
		fmt.Fprintln(os.Stderr, err)
		os.Exit(1)
	}
	defer v.Close()
	fmt.Printf("FPS: %.2f\n", v.FPS())
	for {
		frame, err := v.NextFrame()
		if err != nil {
			break
		}
		fmt.Println(frame)
	}
}
```

## Video Support

Video support is included by default. FFmpeg is downloaded automatically on first use —
no system installation required.

```go
v, err := asciiify.OpenVideo("video.mp4", &asciiify.Options{Width: 80})
if err != nil { panic(err) }
defer v.Close()
fmt.Printf("FPS: %.2f\n", v.FPS())
for {
    frame, err := v.NextFrame()
    if err != nil { break }
    fmt.Println(frame)
}
```

## Options

| Field     | Type     | Default    | Description                               |
| --------- | -------- | ---------- | ----------------------------------------- |
| `Mode`    | `string` | `"ascii"`  | `"ascii"`, `"half-block"`, or `"braille"` |
| `Width`   | `uint32` | `0` (auto) | Output width in characters                |
| `Height`  | `uint32` | `0` (auto) | Output height in characters               |
| `Invert`  | `bool`   | `false`    | Invert brightness                         |
| `Charset` | `string` | `""`       | Custom ASCII ramp (ascii mode only)       |

## License

MIT
