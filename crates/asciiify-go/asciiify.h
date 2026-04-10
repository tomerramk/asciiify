#ifndef ASCIIIFY_H
#define ASCIIIFY_H

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

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

#endif  /* ASCIIIFY_H */
