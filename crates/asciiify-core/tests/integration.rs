//! Integration tests using real image fixtures.

use asciiify_core::{convert_image_file, convert_image_bytes, ConvertOptions, OutputMode};

const FIXTURE: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../tests/fixtures/gradient.png");

#[test]
fn convert_fixture_ascii() {
    let opts = ConvertOptions {
        width: Some(40),
        height: Some(10),
        mode: OutputMode::Ascii,
        ..Default::default()
    };
    let result = convert_image_file(FIXTURE, &opts).unwrap();
    let lines: Vec<&str> = result.lines().collect();
    assert_eq!(lines.len(), 10);
    assert!(lines.iter().all(|l| l.chars().count() == 40));
}

#[test]
fn convert_fixture_half_block() {
    let opts = ConvertOptions {
        width: Some(40),
        height: Some(10),
        mode: OutputMode::HalfBlock,
        ..Default::default()
    };
    let result = convert_image_file(FIXTURE, &opts).unwrap();
    let lines: Vec<&str> = result.lines().collect();
    assert_eq!(lines.len(), 10);
    assert!(lines.iter().all(|l| l.chars().count() == 40));
}

#[test]
fn convert_fixture_braille() {
    let opts = ConvertOptions {
        width: Some(40),
        height: Some(10),
        mode: OutputMode::Braille,
        ..Default::default()
    };
    let result = convert_image_file(FIXTURE, &opts).unwrap();
    let lines: Vec<&str> = result.lines().collect();
    assert_eq!(lines.len(), 10);
    assert!(lines.iter().all(|l| l.chars().count() == 40));
}

#[test]
fn convert_fixture_inverted_differs() {
    let base_opts = ConvertOptions {
        width: Some(40),
        height: Some(10),
        mode: OutputMode::Ascii,
        ..Default::default()
    };
    let inv_opts = ConvertOptions {
        invert: true,
        ..base_opts.clone()
    };
    let normal = convert_image_file(FIXTURE, &base_opts).unwrap();
    let inverted = convert_image_file(FIXTURE, &inv_opts).unwrap();
    assert_ne!(normal, inverted);
}

#[test]
fn convert_fixture_from_bytes() {
    let data = std::fs::read(FIXTURE).unwrap();
    let opts = ConvertOptions {
        width: Some(20),
        height: Some(5),
        mode: OutputMode::Ascii,
        ..Default::default()
    };
    let result = convert_image_bytes(&data, &opts).unwrap();
    assert_eq!(result.lines().count(), 5);
}

#[test]
fn convert_fixture_custom_charset() {
    let opts = ConvertOptions {
        width: Some(20),
        height: Some(5),
        mode: OutputMode::Ascii,
        charset: Some(" .oO@".to_string()),
        ..Default::default()
    };
    let result = convert_image_file(FIXTURE, &opts).unwrap();
    // All chars should be from our custom ramp
    assert!(result.chars().all(|c| " .oO@\n".contains(c)));
}

#[test]
fn convert_fixture_all_modes_produce_output() {
    for mode in [OutputMode::Ascii, OutputMode::HalfBlock, OutputMode::Braille] {
        let opts = ConvertOptions {
            width: Some(30),
            height: Some(8),
            mode,
            ..Default::default()
        };
        let result = convert_image_file(FIXTURE, &opts).unwrap();
        assert!(!result.is_empty(), "mode {:?} produced empty output", mode);
        assert_eq!(result.lines().count(), 8, "mode {:?} wrong line count", mode);
    }
}
