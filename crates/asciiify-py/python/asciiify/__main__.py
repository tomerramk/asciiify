"""CLI entry point for asciiify: python -m asciiify or asciiify command."""

import argparse
import os
import sys
import time

from asciiify import convert

VIDEO_EXTENSIONS = {".mp4", ".avi", ".mov", ".mkv", ".webm", ".m4v", ".flv", ".wmv"}


def main():
    parser = argparse.ArgumentParser(
        prog="asciiify",
        description="Convert images and videos to ASCII art",
    )
    parser.add_argument("input", help="Input image or video file path")
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
    parser.add_argument(
        "--fps",
        type=float,
        default=None,
        help="Override playback FPS for video (default: use video's native FPS)",
    )
    parser.add_argument(
        "--mute",
        action="store_true",
        help="Disable audio playback",
    )

    args = parser.parse_args()

    ext = os.path.splitext(args.input)[1].lower()
    is_video = ext in VIDEO_EXTENSIONS

    if is_video:
        try:
            from asciiify import VideoFrames
        except ImportError:
            print("Error: video support not available in this build", file=sys.stderr)
            sys.exit(1)

        try:
            frames = VideoFrames(
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

        fps = args.fps if args.fps else frames.fps
        delay = 1.0 / fps if fps > 0 else 1.0 / 24.0

        if args.output:
            with open(args.output, "w", encoding="utf-8") as f:
                for frame in frames:
                    f.write(frame)
                    f.write("\n---\n")
            print(f"Written to {args.output}", file=sys.stderr)
        else:
            # Enter alternate screen + hide cursor (same as Rust CLI)
            sys.stdout.write("\x1b[?1049h\x1b[?25l")
            sys.stdout.flush()

            def cleanup():
                sys.stdout.write("\x1b[?25h\x1b[?1049l")
                sys.stdout.flush()

            import signal
            import atexit
            atexit.register(cleanup)
            try:
                signal.signal(signal.SIGINT, lambda *_: (cleanup(), sys.exit(0)))
            except (OSError, ValueError):
                pass

            # Start audio in background (mirrors Rust CLI's start_audio)
            if not args.mute:
                try:
                    from asciiify import play_audio_async
                    play_audio_async(args.input)
                except Exception:
                    pass

            start = time.monotonic()
            frame_index = 0
            for frame in frames:
                target_s = frame_index / fps
                elapsed = time.monotonic() - start
                frame_index += 1

                # Skip if more than one frame behind
                if elapsed > target_s + delay:
                    continue

                # Write each line at its exact row — no full-screen clear, no flash
                lines = frame.split("\n")
                out = "\x1b[H"
                for row, line in enumerate(lines):
                    out += f"\x1b[{row + 1};1H{line}"
                sys.stdout.write(out)
                sys.stdout.flush()

                # Sleep until next frame is due
                remaining = (target_s + delay) - (time.monotonic() - start)
                if remaining > 0:
                    time.sleep(remaining)

            cleanup()
    else:
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
