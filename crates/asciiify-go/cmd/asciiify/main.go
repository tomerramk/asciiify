// Command asciiify converts images and videos to ASCII art.
//
// Usage:
//
//	asciiify [flags] <input>
//
// Install globally:
//
//	go install github.com/tomerramk/asciiify/crates/asciiify-go/cmd/asciiify@latest
package main

import (
	"flag"
	"fmt"
	"os"
	"os/signal"
	"path/filepath"
	"strings"
	"syscall"
	"time"

	asciiify "github.com/tomerramk/asciiify"
)

var videoExtensions = map[string]bool{
	".mp4":  true,
	".avi":  true,
	".mov":  true,
	".mkv":  true,
	".webm": true,
	".m4v":  true,
	".flv":  true,
	".wmv":  true,
}

func main() {
	mode := flag.String("mode", "ascii", "Output mode: ascii, half-block, or braille")
	width := flag.Uint("width", 0, "Output width in characters (0 = auto)")
	height := flag.Uint("height", 0, "Output height in characters (0 = auto)")
	invert := flag.Bool("invert", false, "Invert brightness")
	charset := flag.String("charset", "", "Custom ASCII character ramp (ascii mode only)")
	output := flag.String("output", "", "Output to file instead of stdout")
	fps := flag.Float64("fps", 0, "Override playback FPS for video (0 = use video's native FPS)")
	mute := flag.Bool("mute", false, "Disable audio playback")

	// Short aliases
	flag.StringVar(mode, "m", "ascii", "Output mode (shorthand)")
	flag.UintVar(width, "w", 0, "Output width (shorthand)")
	flag.UintVar(height, "H", 0, "Output height (shorthand)")
	flag.StringVar(output, "o", "", "Output file (shorthand)")

	flag.Usage = func() {
		fmt.Fprintf(os.Stderr, "Usage: asciiify [flags] <input>\n\nConvert images and videos to ASCII art\n\nFlags:\n")
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

	ext := strings.ToLower(filepath.Ext(input))
	if videoExtensions[ext] {
		video, err := asciiify.OpenVideo(input, opts)
		if err != nil {
			fmt.Fprintf(os.Stderr, "Error: %v\n", err)
			os.Exit(1)
		}
		defer video.Close()

		playFPS := *fps
		if playFPS <= 0 {
			playFPS = video.FPS()
		}
		if playFPS <= 0 {
			playFPS = 24
		}
		delay := time.Duration(float64(time.Second) / playFPS)

		if *output != "" {
			var frames []string
			for {
				frame, err := video.NextFrame()
				if err != nil {
					break
				}
				frames = append(frames, frame)
			}
			content := strings.Join(frames, "\n---\n")
			if err := os.WriteFile(*output, []byte(content), 0644); err != nil {
				fmt.Fprintf(os.Stderr, "Error writing file: %v\n", err)
				os.Exit(1)
			}
			fmt.Fprintf(os.Stderr, "Written to %s\n", *output)
		} else {
			fmt.Print("\x1b[?1049h\x1b[?25l")
			cleanup := func() {
				fmt.Print("\x1b[?25h\x1b[?1049l")
			}
			defer cleanup()
			sigCh := make(chan os.Signal, 1)
			signal.Notify(sigCh, os.Interrupt, syscall.SIGTERM)
			go func() {
				<-sigCh
				cleanup()
				os.Exit(0)
			}()
			// Start audio in background (mirrors Rust CLI's start_audio)
			if !*mute {
				asciiify.PlayAudioAsync(input)
			}
			start := time.Now()
			frameIndex := 0
			for {
				frame, err := video.NextFrame()
				if err != nil {
					break
				}
				targetTime := time.Duration(float64(time.Second) * float64(frameIndex) / playFPS)
				elapsed := time.Since(start)
				frameIndex++
				if elapsed > targetTime+delay {
					continue
				}
				lines := strings.Split(frame, "\n")
				var sb strings.Builder
				sb.WriteString("\x1b[H")
				for row, line := range lines {
					fmt.Fprintf(&sb, "\x1b[%d;1H%s", row+1, line)
				}
				fmt.Print(sb.String())
				nextFrameTime := time.Duration(float64(time.Second) * float64(frameIndex) / playFPS)
				remaining := nextFrameTime - time.Since(start)
				if remaining > 0 {
					time.Sleep(remaining)
				}
			}
		}
	} else {
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
}