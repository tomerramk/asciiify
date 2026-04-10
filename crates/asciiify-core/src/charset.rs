/// Map a brightness value (0=black, 255=white) to an ASCII ramp character.
pub fn brightness_to_ascii(brightness: u8, ramp: &str, invert: bool) -> char {
    let b = if invert { 255 - brightness } else { brightness };
    let chars: Vec<char> = ramp.chars().collect();
    let idx = (b as usize * (chars.len() - 1)) / 255;
    chars[idx]
}

/// Half-block threshold encoding.
/// Given two vertically adjacent pixel brightness values (top, bottom),
/// return the appropriate Unicode half-block character.
///
/// - Both dark  → ' ' (space)
/// - Top lit    → '▀' (upper half block)
/// - Bottom lit → '▄' (lower half block)
/// - Both lit   → '█' (full block)
pub fn brightness_to_half_block(top: u8, bottom: u8, threshold: u8, invert: bool) -> char {
    let t = if invert { 255 - top } else { top };
    let b = if invert { 255 - bottom } else { bottom };
    let top_on = t >= threshold;
    let bot_on = b >= threshold;

    match (top_on, bot_on) {
        (false, false) => ' ',
        (true, false) => '\u{2580}',  // ▀
        (false, true) => '\u{2584}',  // ▄
        (true, true) => '\u{2588}',   // █
    }
}

/// Braille dot-to-bit mapping for a 2×4 pixel block.
///
/// Layout:        Bit offsets:
///  (0,0) (1,0)    0x01  0x08
///  (0,1) (1,1)    0x02  0x10
///  (0,2) (1,2)    0x04  0x20
///  (0,3) (1,3)    0x40  0x80
const BRAILLE_MAP: [[u8; 4]; 2] = [
    [0x01, 0x02, 0x04, 0x40], // left column (x=0): dots 1,2,3,7
    [0x08, 0x10, 0x20, 0x80], // right column (x=1): dots 4,5,6,8
];

/// Convert a 2×4 block of brightness values into a Unicode braille character.
///
/// `block[x][y]` where x ∈ 0..2, y ∈ 0..4. Each value is 0–255 brightness.
pub fn block_to_braille(block: &[[u8; 4]; 2], threshold: u8, invert: bool) -> char {
    let mut bits: u8 = 0;
    for x in 0..2 {
        for y in 0..4 {
            let b = if invert { 255 - block[x][y] } else { block[x][y] };
            if b >= threshold {
                bits |= BRAILLE_MAP[x][y];
            }
        }
    }
    char::from_u32(0x2800 + bits as u32).unwrap_or(' ')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ascii_ramp_boundaries() {
        let ramp = " .:-=+*#%@";
        assert_eq!(brightness_to_ascii(0, ramp, false), ' ');
        assert_eq!(brightness_to_ascii(255, ramp, false), '@');
        // Midpoint should be somewhere in the middle
        let mid = brightness_to_ascii(127, ramp, false);
        assert!(ramp.contains(mid));
    }

    #[test]
    fn ascii_ramp_invert() {
        let ramp = " .:-=+*#%@";
        assert_eq!(brightness_to_ascii(0, ramp, true), '@');
        assert_eq!(brightness_to_ascii(255, ramp, true), ' ');
    }

    #[test]
    fn half_block_combos() {
        let th = 128;
        assert_eq!(brightness_to_half_block(0, 0, th, false), ' ');
        assert_eq!(brightness_to_half_block(200, 0, th, false), '\u{2580}');
        assert_eq!(brightness_to_half_block(0, 200, th, false), '\u{2584}');
        assert_eq!(brightness_to_half_block(200, 200, th, false), '\u{2588}');
    }

    #[test]
    fn half_block_invert() {
        let th = 128;
        // Bright pixels become dark when inverted
        assert_eq!(brightness_to_half_block(200, 200, th, true), ' ');
        assert_eq!(brightness_to_half_block(0, 0, th, true), '\u{2588}');
    }

    #[test]
    fn braille_all_black() {
        let block = [[0u8; 4]; 2];
        assert_eq!(block_to_braille(&block, 128, false), '\u{2800}'); // empty braille
    }

    #[test]
    fn braille_all_white() {
        let block = [[255u8; 4]; 2];
        assert_eq!(block_to_braille(&block, 128, false), '\u{28FF}'); // all dots
    }

    #[test]
    fn braille_single_dot() {
        // Only top-left pixel lit → dot 1 → bit 0x01
        let mut block = [[0u8; 4]; 2];
        block[0][0] = 255;
        assert_eq!(block_to_braille(&block, 128, false), '\u{2801}');
    }

    #[test]
    fn braille_checkerboard() {
        // Checkerboard: (0,0),(1,1),(0,2),(1,3) are lit
        let mut block = [[0u8; 4]; 2];
        block[0][0] = 255; // bit 0x01
        block[1][1] = 255; // bit 0x10
        block[0][2] = 255; // bit 0x04
        block[1][3] = 255; // bit 0x80
        let expected = 0x2800 + 0x01 + 0x10 + 0x04 + 0x80;
        assert_eq!(block_to_braille(&block, 128, false), char::from_u32(expected).unwrap());
    }
}
