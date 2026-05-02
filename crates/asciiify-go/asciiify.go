// Package asciiify converts images to ASCII art using the asciiify Rust core library.
//
// Before using this package, build the shared library:
//
//	cd crates/asciiify-go && cargo build --release
//
// Then set CGO flags to point at the built library, for example:
//
//	export CGO_LDFLAGS="-L../../target/release -lasciiify_ffi"
//	export LD_LIBRARY_PATH="../../target/release"  # Linux
package asciiify

/*
#cgo LDFLAGS: -L${SRCDIR}/../../target/release -lasciiify_ffi
#include "asciiify.h"
#include <stdlib.h>
*/
import "C"

import (
	"errors"
	"unsafe"
)

// Options controls the ASCII art conversion.
type Options struct {
	// Mode: "ascii" (default), "half-block", or "braille"
	Mode string
	// Width in characters (0 = auto-detect from terminal)
	Width uint32
	// Height in characters (0 = auto from aspect ratio)
	Height uint32
	// Invert brightness
	Invert bool
	// Custom ASCII character ramp (ascii mode only)
	Charset string
}

// ConvertFile converts an image file at the given path to ASCII art.
func ConvertFile(path string, opts *Options) (string, error) {
	if opts == nil {
		opts = &Options{}
	}

	cPath := C.CString(path)
	defer C.free(unsafe.Pointer(cPath))

	cMode := C.CString(opts.Mode)
	defer C.free(unsafe.Pointer(cMode))

	var cCharset *C.char
	if opts.Charset != "" {
		cCharset = C.CString(opts.Charset)
		defer C.free(unsafe.Pointer(cCharset))
	}

	result := C.asciiify_convert_file(
		cPath,
		cMode,
		C.uint32_t(opts.Width),
		C.uint32_t(opts.Height),
		C.bool(opts.Invert),
		cCharset,
	)
	if result == nil {
		return "", errors.New("asciiify: conversion failed")
	}
	defer C.asciiify_free(result)

	return C.GoString(result), nil
}

// ConvertBytes converts in-memory image bytes to ASCII art.
func ConvertBytes(data []byte, opts *Options) (string, error) {
	if opts == nil {
		opts = &Options{}
	}
	if len(data) == 0 {
		return "", errors.New("asciiify: empty input data")
	}

	cMode := C.CString(opts.Mode)
	defer C.free(unsafe.Pointer(cMode))

	var cCharset *C.char
	if opts.Charset != "" {
		cCharset = C.CString(opts.Charset)
		defer C.free(unsafe.Pointer(cCharset))
	}

	result := C.asciiify_convert_bytes(
		(*C.uchar)(unsafe.Pointer(&data[0])),
		C.size_t(len(data)),
		cMode,
		C.uint32_t(opts.Width),
		C.uint32_t(opts.Height),
		C.bool(opts.Invert),
		cCharset,
	)
	if result == nil {
		return "", errors.New("asciiify: conversion failed")
	}
	defer C.asciiify_free(result)

	return C.GoString(result), nil
}

// VideoFrames provides frame-by-frame ASCII art conversion of a video file.
// The handle must be closed after use.
type VideoFrames struct {
	handle *C.struct_AsciiifyVideo
}

// OpenVideo opens a video file for frame-by-frame ASCII conversion.
// Close the returned VideoFrames when done.
func OpenVideo(path string, opts *Options) (*VideoFrames, error) {
	if opts == nil {
		opts = &Options{}
	}

	cPath := C.CString(path)
	defer C.free(unsafe.Pointer(cPath))

	cMode := C.CString(opts.Mode)
	defer C.free(unsafe.Pointer(cMode))

	var cCharset *C.char
	if opts.Charset != "" {
		cCharset = C.CString(opts.Charset)
		defer C.free(unsafe.Pointer(cCharset))
	}

	handle := C.asciiify_video_open(
		cPath,
		cMode,
		C.uint32_t(opts.Width),
		C.uint32_t(opts.Height),
		C.bool(opts.Invert),
		cCharset,
	)
	if handle == nil {
		return nil, errors.New("asciiify: failed to open video")
	}

	return &VideoFrames{handle: handle}, nil
}

// FPS returns the frames per second of the video.
func (v *VideoFrames) FPS() float64 {
	if v.handle == nil {
		return 0
	}
	return float64(C.asciiify_video_fps(v.handle))
}

// NextFrame returns the next frame as an ASCII art string.
// Returns ("", io.EOF) when there are no more frames.
func (v *VideoFrames) NextFrame() (string, error) {
	if v.handle == nil {
		return "", errors.New("asciiify: video handle is closed")
	}
	result := C.asciiify_video_next_frame(v.handle)
	if result == nil {
		return "", errors.New("asciiify: no more frames")
	}
	defer C.asciiify_free(result)
	return C.GoString(result), nil
}

// Close releases all resources associated with the video.
func (v *VideoFrames) Close() {
	if v.handle != nil {
		C.asciiify_video_close(v.handle)
		v.handle = nil
	}
}

// PlayAudioAsync decodes all audio from a video file and plays it in a
// background goroutine. Returns immediately.
func PlayAudioAsync(path string) {
	cPath := C.CString(path)
	defer C.free(unsafe.Pointer(cPath))
	C.asciiify_play_audio_async(cPath)
}
