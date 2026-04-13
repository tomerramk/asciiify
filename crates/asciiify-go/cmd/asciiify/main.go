// Command asciiify converts images to ASCII art.
//
// Usage:
//
//	asciiify [flags] <input>
//
// Install globally:
//
//	go install github.com/tomerramk/asciiify/cmd/asciiify@latest
package main

import (
	"flag"
	"fmt"
	"os"

	asciiify "github.com/tomerramk/asciiify"
)

func main() {
	mode := flag.String("mode", "ascii", "Output mode: ascii, half-block, or braille")
	width := flag.Uint("width", 0, "Output width in characters (0 = auto)")
	height := flag.Uint("height", 0, "Output height in characters (0 = auto)")
	invert := flag.Bool("invert", false, "Invert brightness")
	charset := flag.String("charset", "", "Custom ASCII character ramp (ascii mode only)")
	output := flag.String("output", "", "Output to file instead of stdout")

	// Short aliases
	flag.StringVar(mode, "m", "ascii", "Output mode (shorthand)")
	flag.UintVar(width, "w", 0, "Output width (shorthand)")
	flag.UintVar(height, "H", 0, "Output height (shorthand)")
	flag.StringVar(output, "o", "", "Output file (shorthand)")

	flag.Usage = func() {
		fmt.Fprintf(os.Stderr, "Usage: asciiify [flags] <input>\n\nConvert images to ASCII art\n\nFlags:\n")
		flag.PrintDefaults()
	}
	flag.Parse()

	if flag.NArg() != 1 {
		flag.Usage()
		os.Exit(1)
	}

	input := flag.Arg(0)

	opts := &asciiify.Options{
		Mode:    *mode,
		Width:   uint32(*width),
		Height:  uint32(*height),
		Invert:  *invert,
		Charset: *charset,
	}

	art, err := asciiify.ConvertFile(input, opts)
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error: %v\n", err)
		os.Exit(1)
	}

	if *output != "" {
		if err := os.WriteFile(*output, []byte(art), 0644); err != nil {
			fmt.Fprintf(os.Stderr, "Error writing file: %v\n", err)
			os.Exit(1)
		}
		fmt.Fprintf(os.Stderr, "Written to %s\n", *output)
	} else {
		fmt.Println(art)
	}
}
