use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::ptr;

use asciiify_core::{ConvertOptions, OutputMode};

#[cfg(feature = "video")]
use asciiify_core::{convert_image, extract_frames, FrameIterator};

#[cfg(feature = "video")]
use rodio::{buffer::SamplesBuffer, OutputStream, Sink};

fn parse_mode(mode: &str) -> OutputMode {
    match mode.to_lowercase().as_str() {
        "half-block" | "halfblock" | "block" => OutputMode::HalfBlock,
        "braille" => OutputMode::Braille,
        _ => OutputMode::Ascii,
    }
}

/// Convert an image file to ASCII art.
///
/// # Safety
/// - `path` must be a valid null-terminated UTF-8 string.
/// - `mode` must be a valid null-terminated UTF-8 string (or null for default).
/// - `charset` must be a valid null-terminated UTF-8 string (or null for default).
/// - The returned string must be freed with `asciiify_free`.
#[no_mangle]
pub unsafe extern "C" fn asciiify_convert_file(
    path: *const c_char,
    mode: *const c_char,
    width: u32,
    height: u32,
    invert: bool,
    charset: *const c_char,
) -> *mut c_char {
    let path = match unsafe { CStr::from_ptr(path) }.to_str() {
        Ok(s) => s,
        Err(_) => return ptr::null_mut(),
    };

    let mode_str = if mode.is_null() {
        "ascii"
    } else {
        unsafe { CStr::from_ptr(mode) }.to_str().unwrap_or("ascii")
    };

    let charset_str = if charset.is_null() {
        None
    } else {
        unsafe { CStr::from_ptr(charset) }
            .to_str()
            .ok()
            .map(String::from)
    };

    let opts = ConvertOptions {
        width: if width == 0 { None } else { Some(width) },
        height: if height == 0 { None } else { Some(height) },
        mode: parse_mode(mode_str),
        invert,
        charset: charset_str,
    };

    match asciiify_core::convert_image_file(path, &opts) {
        Ok(result) => CString::new(result)
            .map(|s| s.into_raw())
            .unwrap_or(ptr::null_mut()),
        Err(_) => ptr::null_mut(),
    }
}

/// Convert in-memory image bytes to ASCII art.
///
/// # Safety
/// - `data` must point to a valid byte buffer of length `data_len`.
/// - `mode` must be a valid null-terminated UTF-8 string (or null for default).
/// - `charset` must be a valid null-terminated UTF-8 string (or null for default).
/// - The returned string must be freed with `asciiify_free`.
#[no_mangle]
pub unsafe extern "C" fn asciiify_convert_bytes(
    data: *const u8,
    data_len: usize,
    mode: *const c_char,
    width: u32,
    height: u32,
    invert: bool,
    charset: *const c_char,
) -> *mut c_char {
    if data.is_null() || data_len == 0 {
        return ptr::null_mut();
    }

    let bytes = unsafe { std::slice::from_raw_parts(data, data_len) };

    let mode_str = if mode.is_null() {
        "ascii"
    } else {
        unsafe { CStr::from_ptr(mode) }.to_str().unwrap_or("ascii")
    };

    let charset_str = if charset.is_null() {
        None
    } else {
        unsafe { CStr::from_ptr(charset) }
            .to_str()
            .ok()
            .map(String::from)
    };

    let opts = ConvertOptions {
        width: if width == 0 { None } else { Some(width) },
        height: if height == 0 { None } else { Some(height) },
        mode: parse_mode(mode_str),
        invert,
        charset: charset_str,
    };

    match asciiify_core::convert_image_bytes(bytes, &opts) {
        Ok(result) => CString::new(result)
            .map(|s| s.into_raw())
            .unwrap_or(ptr::null_mut()),
        Err(_) => ptr::null_mut(),
    }
}

/// Free a string returned by asciiify functions.
///
/// # Safety
/// `ptr` must be a pointer returned by `asciiify_convert_file` or `asciiify_convert_bytes`,
/// or null (which is a no-op).
#[no_mangle]
pub unsafe extern "C" fn asciiify_free(ptr: *mut c_char) {
    if !ptr.is_null() {
        drop(unsafe { CString::from_raw(ptr) });
    }
}

/// Opaque handle to a video frame iterator with associated conversion options.
#[cfg(feature = "video")]
pub struct AsciiifyVideo {
    frames: FrameIterator,
    opts: ConvertOptions,
}

/// Open a video file and return an opaque handle for frame-by-frame conversion.
///
/// Returns null on failure.
///
/// # Safety
/// - `path` must be a valid null-terminated UTF-8 string.
/// - `mode` may be null (defaults to "ascii").
/// - `charset` may be null (defaults to built-in ramp).
/// - The returned handle must be freed with `asciiify_video_close`.
#[cfg(feature = "video")]
#[no_mangle]
pub unsafe extern "C" fn asciiify_video_open(
    path: *const c_char,
    mode: *const c_char,
    width: u32,
    height: u32,
    invert: bool,
    charset: *const c_char,
) -> *mut AsciiifyVideo {
    let path = match unsafe { CStr::from_ptr(path) }.to_str() {
        Ok(s) => s,
        Err(_) => return ptr::null_mut(),
    };

    let mode_str = if mode.is_null() {
        "ascii"
    } else {
        unsafe { CStr::from_ptr(mode) }.to_str().unwrap_or("ascii")
    };

    let charset_str = if charset.is_null() {
        None
    } else {
        unsafe { CStr::from_ptr(charset) }
            .to_str()
            .ok()
            .map(String::from)
    };

    let opts = ConvertOptions {
        width: if width == 0 { None } else { Some(width) },
        height: if height == 0 { None } else { Some(height) },
        mode: parse_mode(mode_str),
        invert,
        charset: charset_str,
    };

    let frames = match extract_frames(path) {
        Ok(f) => f,
        Err(_) => return ptr::null_mut(),
    };

    Box::into_raw(Box::new(AsciiifyVideo { frames, opts }))
}

/// Get the frames-per-second of the opened video.
///
/// # Safety
/// `handle` must be a valid pointer returned by `asciiify_video_open`.
#[cfg(feature = "video")]
#[no_mangle]
pub unsafe extern "C" fn asciiify_video_fps(handle: *const AsciiifyVideo) -> f64 {
    if handle.is_null() {
        return 0.0;
    }
    unsafe { &*handle }.frames.fps()
}

/// Get the next frame as an ASCII art string.
///
/// Returns null when there are no more frames or on error.
/// The returned string must be freed with `asciiify_free`.
///
/// # Safety
/// `handle` must be a valid pointer returned by `asciiify_video_open`.
#[cfg(feature = "video")]
#[no_mangle]
pub unsafe extern "C" fn asciiify_video_next_frame(handle: *mut AsciiifyVideo) -> *mut c_char {
    if handle.is_null() {
        return ptr::null_mut();
    }
    let video = unsafe { &mut *handle };
    match video.frames.next() {
        Some(Ok(img)) => match convert_image(&img, &video.opts) {
            Ok(s) => CString::new(s)
                .map(|c| c.into_raw())
                .unwrap_or(ptr::null_mut()),
            Err(_) => ptr::null_mut(),
        },
        _ => ptr::null_mut(),
    }
}

/// Close the video handle and free all associated resources.
///
/// # Safety
/// `handle` must be a valid pointer returned by `asciiify_video_open`, or null (no-op).
#[cfg(feature = "video")]
#[no_mangle]
pub unsafe extern "C" fn asciiify_video_close(handle: *mut AsciiifyVideo) {
    if !handle.is_null() {
        drop(unsafe { Box::from_raw(handle) });
    }
}

/// Decode all audio from a video file and play it in a background thread.
/// Returns immediately; audio continues in the background.
///
/// # Safety
/// `path` must be a valid null-terminated UTF-8 string.
#[cfg(feature = "video")]
#[no_mangle]
pub unsafe extern "C" fn asciiify_play_audio_async(path: *const c_char) {
    let path_str = match unsafe { CStr::from_ptr(path) }.to_str() {
        Ok(s) => s.to_string(),
        Err(_) => return,
    };
    std::thread::spawn(move || {
        let audio = match asciiify_core::decode_audio(&path_str) {
            Ok(Some(a)) => a,
            _ => return,
        };
        let (_stream, handle) = match OutputStream::try_default() {
            Ok(s) => s,
            Err(_) => return,
        };
        let sink = match Sink::try_new(&handle) {
            Ok(s) => s,
            Err(_) => return,
        };
        sink.append(SamplesBuffer::new(
            audio.channels,
            audio.sample_rate,
            audio.samples,
        ));
        sink.sleep_until_end();
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;

    #[test]
    fn convert_file_nonexistent_returns_null() {
        let path = CString::new("nonexistent_file.png").unwrap();
        let result = unsafe {
            asciiify_convert_file(path.as_ptr(), ptr::null(), 40, 20, false, ptr::null())
        };
        assert!(result.is_null());
    }

    #[test]
    fn convert_bytes_null_data_returns_null() {
        let result = unsafe {
            asciiify_convert_bytes(ptr::null(), 0, ptr::null(), 40, 20, false, ptr::null())
        };
        assert!(result.is_null());
    }

    #[test]
    fn convert_bytes_empty_data_returns_null() {
        let data: &[u8] = &[];
        let result = unsafe {
            asciiify_convert_bytes(data.as_ptr(), 0, ptr::null(), 40, 20, false, ptr::null())
        };
        assert!(result.is_null());
    }

    #[test]
    fn convert_bytes_invalid_data_returns_null() {
        let data = b"not an image";
        let result = unsafe {
            asciiify_convert_bytes(
                data.as_ptr(),
                data.len(),
                ptr::null(),
                40,
                20,
                false,
                ptr::null(),
            )
        };
        assert!(result.is_null());
    }

    #[test]
    fn convert_bytes_valid_png() {
        // Create a minimal valid PNG in memory
        let img = image::DynamicImage::ImageLuma8(image::GrayImage::from_fn(4, 4, |_, _| {
            image::Luma([128u8])
        }));
        let mut buf = Vec::new();
        img.write_to(&mut std::io::Cursor::new(&mut buf), image::ImageFormat::Png)
            .unwrap();

        let mode = CString::new("ascii").unwrap();
        let result = unsafe {
            asciiify_convert_bytes(
                buf.as_ptr(),
                buf.len(),
                mode.as_ptr(),
                4,
                2,
                false,
                ptr::null(),
            )
        };
        assert!(!result.is_null());
        let s = unsafe { CStr::from_ptr(result) }.to_str().unwrap();
        assert_eq!(s.lines().count(), 2);
        unsafe { asciiify_free(result) };
    }

    #[test]
    fn convert_bytes_braille_mode() {
        let img = image::DynamicImage::ImageLuma8(image::GrayImage::from_fn(4, 4, |_, _| {
            image::Luma([255u8])
        }));
        let mut buf = Vec::new();
        img.write_to(&mut std::io::Cursor::new(&mut buf), image::ImageFormat::Png)
            .unwrap();

        let mode = CString::new("braille").unwrap();
        let result = unsafe {
            asciiify_convert_bytes(
                buf.as_ptr(),
                buf.len(),
                mode.as_ptr(),
                4,
                2,
                false,
                ptr::null(),
            )
        };
        assert!(!result.is_null());
        unsafe { asciiify_free(result) };
    }

    #[test]
    fn free_null_is_safe() {
        unsafe { asciiify_free(ptr::null_mut()) };
    }
}
