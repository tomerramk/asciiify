/// Output mode for ASCII art conversion.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OutputMode {
    /// Classic ASCII character ramp (one char per pixel).
    #[default]
    Ascii,
    /// Unicode half-block characters (▀▄█ ), doubling vertical resolution.
    HalfBlock,
    /// Unicode braille patterns (2×4 dot matrix per character), highest resolution.
    Braille,
}

/// Options controlling the ASCII art conversion.
#[derive(Debug, Clone, Default)]
pub struct ConvertOptions {
    /// Output width in characters. `None` = auto-detect from terminal.
    pub width: Option<u32>,
    /// Output height in characters. `None` = auto from aspect ratio.
    pub height: Option<u32>,
    /// Which character mapping mode to use.
    pub mode: OutputMode,
    /// Invert brightness (light chars on dark background becomes dark on light).
    pub invert: bool,
    /// Custom ASCII ramp string (only used in `Ascii` mode).
    /// Characters should go from darkest (space) to brightest.
    pub charset: Option<String>,
}

impl ConvertOptions {
    /// Resolve the effective terminal width and height.
    /// Returns `(cols, rows)` in characters.
    pub fn resolve_dimensions(&self, img_width: u32, img_height: u32) -> (u32, u32) {
        let term_size = crossterm::terminal::size().unwrap_or((80, 24));
        let cols = self.width.unwrap_or(term_size.0 as u32);

        if let Some(h) = self.height {
            return (cols, h);
        }

        // Compute height from aspect ratio.
        // Terminal chars are roughly 2:1 (height:width), so we apply a 0.5 correction.
        let aspect = img_height as f64 / img_width as f64;
        let char_aspect_ratio = 0.5;
        let rows = (cols as f64 * aspect * char_aspect_ratio).round() as u32;
        let rows = rows.max(1);

        (cols, rows)
    }

    /// Get the ASCII ramp string (custom or default).
    pub fn ascii_ramp(&self) -> &str {
        self.charset.as_deref().unwrap_or(" .:-=+*#%@")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_options() {
        let opts = ConvertOptions::default();
        assert_eq!(opts.width, None);
        assert_eq!(opts.height, None);
        assert_eq!(opts.mode, OutputMode::Ascii);
        assert!(!opts.invert);
        assert_eq!(opts.charset, None);
    }

    #[test]
    fn ascii_ramp_default() {
        let opts = ConvertOptions::default();
        assert_eq!(opts.ascii_ramp(), " .:-=+*#%@");
    }

    #[test]
    fn ascii_ramp_custom() {
        let opts = ConvertOptions {
            charset: Some(" .oO@".to_string()),
            ..Default::default()
        };
        assert_eq!(opts.ascii_ramp(), " .oO@");
    }

    #[test]
    fn resolve_dimensions_explicit() {
        let opts = ConvertOptions {
            width: Some(40),
            height: Some(20),
            ..Default::default()
        };
        assert_eq!(opts.resolve_dimensions(800, 600), (40, 20));
    }

    #[test]
    fn resolve_dimensions_auto_height() {
        let opts = ConvertOptions {
            width: Some(80),
            height: None,
            ..Default::default()
        };
        let (cols, rows) = opts.resolve_dimensions(200, 200);
        assert_eq!(cols, 80);
        // Square image at width 80 with 0.5 aspect correction → 40
        assert_eq!(rows, 40);
    }

    #[test]
    fn resolve_dimensions_min_one_row() {
        let opts = ConvertOptions {
            width: Some(80),
            height: None,
            ..Default::default()
        };
        // Very wide image → rows should still be at least 1
        let (_, rows) = opts.resolve_dimensions(10000, 1);
        assert!(rows >= 1);
    }

    #[test]
    fn output_mode_default() {
        assert_eq!(OutputMode::default(), OutputMode::Ascii);
    }
}
