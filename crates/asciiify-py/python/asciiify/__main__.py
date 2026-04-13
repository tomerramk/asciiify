"""CLI entry point for asciiify: python -m asciiify or asciiify command."""

import argparse
import sys

from asciiify import convert


def main():
    parser = argparse.ArgumentParser(
        prog="asciiify",
        description="Convert images to ASCII art",
    )
    parser.add_argument("input", help="Input image file path")
    parser.add_argument(
        "-m", "--mode",
        default="ascii",
        choices=["ascii", "half-block", "braille"],
        help="Output mode (default: ascii)",
    )
    parser.add_argument(
        "-w", "--width",
        type=int,
        default=None,
        help="Output width in characters",
    )
    parser.add_argument(
        "-H", "--height",
        type=int,
        default=None,
        help="Output height in characters",
    )
    parser.add_argument(
        "--invert",
        action="store_true",
        help="Invert brightness",
    )
    parser.add_argument(
        "--charset",
        default=None,
        help="Custom ASCII character ramp (ascii mode only)",
    )
    parser.add_argument(
        "-o", "--output",
        default=None,
        help="Output to file instead of stdout",
    )

    args = parser.parse_args()

    try:
        art = convert(
            args.input,
            mode=args.mode,
            width=args.width,
            height=args.height,
            invert=args.invert,
            charset=args.charset,
        )
    except OSError as e:
        print(f"Error: {e}", file=sys.stderr)
        sys.exit(1)

    if args.output:
        with open(args.output, "w", encoding="utf-8") as f:
            f.write(art)
        print(f"Written to {args.output}", file=sys.stderr)
    else:
        print(art)


if __name__ == "__main__":
    main()
