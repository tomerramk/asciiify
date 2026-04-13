# asciiify-go

Go package for converting images to ASCII art. Powered by Rust via C FFI.

Part of the [asciiify](https://github.com/tomerramk/asciiify) project.

![Pikachu](/docs/pikachu.png)

## Prerequisites

Build the shared library before using:

```bash
git clone https://github.com/tomerramk/asciiify.git
cd asciiify
cargo build --release -p asciiify-go
```

## Install

```bash
go get github.com/tomerramk/asciiify/crates/asciiify-go
```

## CLI

Install the `asciiify` command:

```bash
go install github.com/tomerramk/asciiify/crates/asciiify-go/cmd/asciiify@latest

# Convert an image
asciiify image.png

# Braille mode, custom width
asciiify -mode braille -w 100 photo.jpg

# Output to file
asciiify -o output.txt image.png

# All options
asciiify -help
```

## Library Usage

```go
package main

import (
	"fmt"
	asciiify "github.com/tomerramk/asciiify/crates/asciiify-go"
)

func main() {
	// Convert image file
	art, err := asciiify.ConvertFile("image.png", nil)
	if err != nil {
		panic(err)
	}
	fmt.Println(art)

	// With options
	art, _ = asciiify.ConvertFile("photo.jpg", &asciiify.Options{
		Mode:  "braille",
		Width: 100,
	})
	fmt.Println(art)

	// Convert from bytes
	data, _ := os.ReadFile("image.png")
	art, _ = asciiify.ConvertBytes(data, &asciiify.Options{
		Mode:  "half-block",
		Width: 80,
	})
	fmt.Println(art)
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
