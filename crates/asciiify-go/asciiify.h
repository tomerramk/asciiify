#ifndef ASCIIIFY_H
#define ASCIIIFY_H

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

/**
 * Opaque handle to a video frame iterator with associated conversion options.
 */
typedef struct AsciiifyVideo AsciiifyVideo;

/**
 * Convert an image file to ASCII art.
 *
 * # Safety
 * - `path` must be a valid null-terminated UTF-8 string.
 * - `mode` must be a valid null-terminated UTF-8 string (or null for default).
 * - `charset` must be a valid null-terminated UTF-8 string (or null for default).
 * - The returned string must be freed with `asciiify_free`.
 */
char *asciiify_convert_file(const char *path,
                            const char *mode,
                            uint32_t width,
                            uint32_t height,
                            bool invert,
                            const char *charset);

/**
 * Convert in-memory image bytes to ASCII art.
 *
 * # Safety
 * - `data` must point to a valid byte buffer of length `data_len`.
 * - `mode` must be a valid null-terminated UTF-8 string (or null for default).
 * - `charset` must be a valid null-terminated UTF-8 string (or null for default).
 * - The returned string must be freed with `asciiify_free`.
 */
char *asciiify_convert_bytes(const uint8_t *data,
                             uintptr_t data_len,
                             const char *mode,
                             uint32_t width,
                             uint32_t height,
                             bool invert,
                             const char *charset);

/**
 * Free a string returned by asciiify functions.
 *
 * # Safety
 * `ptr` must be a pointer returned by `asciiify_convert_file` or `asciiify_convert_bytes`,
 * or null (which is a no-op).
 */
void asciiify_free(char *ptr);

/**
 * Open a video file and return an opaque handle for frame-by-frame conversion.
 *
 * Returns null on failure.
 *
 * # Safety
 * - `path` must be a valid null-terminated UTF-8 string.
 * - `mode` may be null (defaults to "ascii").
 * - `charset` may be null (defaults to built-in ramp).
 * - The returned handle must be freed with `asciiify_video_close`.
 */
struct AsciiifyVideo *asciiify_video_open(const char *path,
                                          const char *mode,
                                          uint32_t width,
                                          uint32_t height,
                                          bool invert,
                                          const char *charset);

/**
 * Get the frames-per-second of the opened video.
 *
 * # Safety
 * `handle` must be a valid pointer returned by `asciiify_video_open`.
 */
double asciiify_video_fps(const struct AsciiifyVideo *handle);

/**
 * Get the next frame as an ASCII art string.
 *
 * Returns null when there are no more frames or on error.
 * The returned string must be freed with `asciiify_free`.
 *
 * # Safety
 * `handle` must be a valid pointer returned by `asciiify_video_open`.
 */
char *asciiify_video_next_frame(struct AsciiifyVideo *handle);

/**
 * Close the video handle and free all associated resources.
 *
 * # Safety
 * `handle` must be a valid pointer returned by `asciiify_video_open`, or null (no-op).
 */
void asciiify_video_close(struct AsciiifyVideo *handle);

/**
 * Decode all audio from a video file and play it in a background thread.
 * Returns immediately; audio continues in the background.
 *
 * # Safety
 * `path` must be a valid null-terminated UTF-8 string.
 */
void asciiify_play_audio_async(const char *path);

#endif  /* ASCIIIFY_H */
