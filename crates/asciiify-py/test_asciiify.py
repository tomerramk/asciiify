"""Tests for the asciiify Python bindings."""

import os
import pytest

import asciiify

FIXTURE = os.path.join(
    os.path.dirname(__file__), "..", "..", "tests", "fixtures", "gradient.png"
)


class TestConvert:
    def test_basic(self):
        result = asciiify.convert(FIXTURE, width=40, height=10)
        lines = result.splitlines()
        assert len(lines) == 10
        assert all(len(line) == 40 for line in lines)

    def test_ascii_mode(self):
        result = asciiify.convert(FIXTURE, mode="ascii", width=20, height=5)
        assert len(result.splitlines()) == 5

    def test_half_block_mode(self):
        result = asciiify.convert(FIXTURE, mode="half-block", width=20, height=5)
        lines = result.splitlines()
        assert len(lines) == 5
        assert all(len(line) == 20 for line in lines)

    def test_braille_mode(self):
        result = asciiify.convert(FIXTURE, mode="braille", width=20, height=5)
        lines = result.splitlines()
        assert len(lines) == 5
        assert all(len(line) == 20 for line in lines)

    def test_invert(self):
        normal = asciiify.convert(FIXTURE, width=20, height=5)
        inverted = asciiify.convert(FIXTURE, width=20, height=5, invert=True)
        assert normal != inverted

    def test_custom_charset(self):
        result = asciiify.convert(FIXTURE, width=20, height=5, charset=" .oO@")
        assert all(c in " .oO@\n" for c in result)

    def test_nonexistent_file_raises(self):
        with pytest.raises(OSError):
            asciiify.convert("nonexistent_image_12345.png", width=10, height=5)

    def test_invalid_mode_raises(self):
        with pytest.raises(ValueError, match="unknown mode"):
            asciiify.convert(FIXTURE, mode="invalid", width=10, height=5)


class TestConvertBytes:
    def test_basic(self):
        with open(FIXTURE, "rb") as f:
            data = f.read()
        result = asciiify.convert_bytes(data, width=20, height=5)
        assert len(result.splitlines()) == 5

    def test_all_modes(self):
        with open(FIXTURE, "rb") as f:
            data = f.read()
        for mode in ("ascii", "half-block", "braille"):
            result = asciiify.convert_bytes(data, mode=mode, width=20, height=5)
            lines = result.splitlines()
            assert len(lines) == 5, f"mode={mode} produced {len(lines)} lines"
            assert all(len(l) == 20 for l in lines), f"mode={mode} wrong width"

    def test_invalid_bytes_raises(self):
        with pytest.raises(OSError):
            asciiify.convert_bytes(b"not an image", width=10, height=5)


class TestConverter:
    def test_basic(self):
        converter = asciiify.Converter(width=30, height=8)
        result = converter.convert(FIXTURE)
        lines = result.splitlines()
        assert len(lines) == 8
        assert all(len(line) == 30 for line in lines)

    def test_with_mode(self):
        converter = asciiify.Converter(mode="braille", width=20, height=5)
        result = converter.convert(FIXTURE)
        assert len(result.splitlines()) == 5

    def test_convert_bytes(self):
        converter = asciiify.Converter(width=20, height=5)
        with open(FIXTURE, "rb") as f:
            data = f.read()
        result = converter.convert_bytes(data)
        assert len(result.splitlines()) == 5

    def test_reusable(self):
        converter = asciiify.Converter(width=20, height=5)
        result1 = converter.convert(FIXTURE)
        result2 = converter.convert(FIXTURE)
        assert result1 == result2

    def test_invert(self):
        normal = asciiify.Converter(width=20, height=5)
        inverted = asciiify.Converter(width=20, height=5, invert=True)
        assert normal.convert(FIXTURE) != inverted.convert(FIXTURE)

    def test_invalid_mode_raises(self):
        with pytest.raises(ValueError, match="unknown mode"):
            asciiify.Converter(mode="bad")
