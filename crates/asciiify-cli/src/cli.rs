use clap::Parser;
use std::path::Path;

use asciiify_core::OutputMode;

#[derive(Parser)]
#[command(name = "asciiify", about = "Convert images and video to ASCII art", version)]
pub struct Cli {
    /// Input image or video file path
    pub input: String,

    /// Output mode: ascii, half-block, or braille
    #[arg(short, long, default_value = "ascii")]
    pub mode: String,

    /// Output width in characters (default: terminal width)
    #[arg(short, long)]
    pub width: Option<u32>,

    /// Output height in characters (default: auto from aspect ratio)
    #[arg(short = 'H', long)]
    pub height: Option<u32>,

    /// Invert brightness
    #[arg(long)]
    pub invert: bool,

    /// Custom ASCII character ramp (ascii mode only)
    #[arg(long)]
    pub charset: Option<String>,

    /// Target FPS for video playback
    #[arg(long, default_value = "15")]
    pub fps: f64,

    /// Disable audio playback
    #[arg(long)]
    pub mute: bool,

    /// Output to file instead of stdout
    #[arg(short, long)]
    pub output: Option<String>,
}

pub fn parse_mode(s: &str) -> OutputMode {
    match s.to_lowercase().as_str() {
        "half-block" | "halfblock" | "block" => OutputMode::HalfBlock,
        "braille" => OutputMode::Braille,
        _ => OutputMode::Ascii,
    }
}

const VIDEO_EXTENSIONS: &[&str] = &[
    "mp4", "mkv", "avi", "mov", "webm", "flv", "wmv", "m4v", "mpg", "mpeg",
];

pub fn is_video(path: &str) -> bool {
    Path::new(path)
        .extension()
        .and_then(|e| e.to_str())
        .map(|e| VIDEO_EXTENSIONS.contains(&e.to_lowercase().as_str()))
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_mode_ascii_variants() {
        assert_eq!(parse_mode("ascii"), OutputMode::Ascii);
        assert_eq!(parse_mode("ASCII"), OutputMode::Ascii);
        assert_eq!(parse_mode("unknown"), OutputMode::Ascii);
        assert_eq!(parse_mode(""), OutputMode::Ascii);
    }

    #[test]
    fn parse_mode_half_block_variants() {
        assert_eq!(parse_mode("half-block"), OutputMode::HalfBlock);
        assert_eq!(parse_mode("halfblock"), OutputMode::HalfBlock);
        assert_eq!(parse_mode("block"), OutputMode::HalfBlock);
        assert_eq!(parse_mode("HALF-BLOCK"), OutputMode::HalfBlock);
    }

    #[test]
    fn parse_mode_braille() {
        assert_eq!(parse_mode("braille"), OutputMode::Braille);
        assert_eq!(parse_mode("BRAILLE"), OutputMode::Braille);
    }

    #[test]
    fn is_video_common_formats() {
        assert!(is_video("clip.mp4"));
        assert!(is_video("clip.mkv"));
        assert!(is_video("clip.avi"));
        assert!(is_video("clip.mov"));
        assert!(is_video("clip.webm"));
        assert!(is_video("clip.flv"));
        assert!(is_video("clip.wmv"));
        assert!(is_video("clip.m4v"));
        assert!(is_video("clip.mpg"));
        assert!(is_video("clip.mpeg"));
    }

    #[test]
    fn is_video_case_insensitive() {
        assert!(is_video("clip.MP4"));
        assert!(is_video("clip.Mkv"));
    }

    #[test]
    fn is_video_rejects_images() {
        assert!(!is_video("image.png"));
        assert!(!is_video("image.jpg"));
        assert!(!is_video("image.gif"));
        assert!(!is_video("image.bmp"));
    }

    #[test]
    fn is_video_no_extension() {
        assert!(!is_video("noextension"));
        assert!(!is_video(""));
    }

    #[test]
    fn is_video_with_path() {
        assert!(is_video("/some/path/to/video.mp4"));
        assert!(is_video("C:\\Users\\video.avi"));
        assert!(!is_video("/some/path/to/image.png"));
    }
}
