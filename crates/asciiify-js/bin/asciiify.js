#!/usr/bin/env node

"use strict";

const fs = require("fs");
const path = require("path");
const { convert, VideoFrames, playAudioAsync } = require("../index.js");

const VIDEO_EXTENSIONS = new Set([
  ".mp4",
  ".avi",
  ".mov",
  ".mkv",
  ".webm",
  ".m4v",
  ".flv",
  ".wmv",
]);

function sleepMs(ms) {
  Atomics.wait(new Int32Array(new SharedArrayBuffer(4)), 0, 0, ms);
}

function printUsage() {
  console.error(
    `Usage: asciiify [options] <input>

Convert images and videos to ASCII art

Options:
  -m, --mode <mode>        Output mode: ascii, half-block, braille (default: ascii)
  -w, --width <number>     Output width in characters
  -H, --height <number>    Output height in characters
  --invert                 Invert brightness
  --charset <chars>        Custom ASCII character ramp (ascii mode only)
  --fps <number>           Override playback FPS for video
  --mute                   Disable audio playback
  -o, --output <file>      Output to file instead of stdout
  -h, --help               Show this help message`,
  );
}

function parseArgs(argv) {
  const args = { input: null, options: {}, output: null, fps: null };
  let i = 0;

  while (i < argv.length) {
    const arg = argv[i];

    switch (arg) {
      case "-h":
      case "--help":
        printUsage();
        process.exit(0);
        break;
      case "-m":
      case "--mode":
        args.options.mode = argv[++i];
        break;
      case "-w":
      case "--width":
        args.options.width = parseInt(argv[++i], 10);
        break;
      case "-H":
      case "--height":
        args.options.height = parseInt(argv[++i], 10);
        break;
      case "--invert":
        args.options.invert = true;
        break;
      case "--charset":
        args.options.charset = argv[++i];
        break;
      case "--fps":
        args.fps = parseFloat(argv[++i]);
        break;
      case "--mute":
        args.mute = true;
        break;
      case "-o":
      case "--output":
        args.output = argv[++i];
        break;
      default:
        if (arg.startsWith("-")) {
          console.error(`Unknown option: ${arg}`);
          printUsage();
          process.exit(1);
        }
        args.input = arg;
        break;
    }
    i++;
  }

  return args;
}

const args = parseArgs(process.argv.slice(2));

if (!args.input) {
  printUsage();
  process.exit(1);
}

const inputPath = path.resolve(args.input);
const ext = path.extname(inputPath).toLowerCase();
const isVideo = VIDEO_EXTENSIONS.has(ext);

try {
  if (isVideo) {
    const frames = new VideoFrames(inputPath, args.options);
    const fps = args.fps || frames.fps || 24;
    const delayMs = 1000 / fps;

    if (args.output) {
      const out = [];
      let frame;
      while ((frame = frames.nextFrame()) !== null) {
        out.push(frame);
      }
      fs.writeFileSync(args.output, out.join("\n---\n"), "utf-8");
      console.error(`Written to ${args.output}`);
    } else {
      // Enter alternate screen + hide cursor (same as Rust CLI's EnterAlternateScreen)
      process.stdout.write("\x1B[?1049h\x1B[?25l");

      let cleanedUp = false;
      const cleanup = () => {
        if (cleanedUp) return;
        cleanedUp = true;
        process.stdout.write("\x1B[?25h\x1B[?1049l");
      };
      process.on("exit", cleanup);
      process.on("SIGINT", () => {
        cleanup();
        process.exit(0);
      });

      // Start audio in background (mirrors Rust CLI's start_audio)
      if (!args.mute) {
        try {
          playAudioAsync(inputPath);
        } catch (_) {}
      }

      const startTime = Date.now();
      let frameIndex = 0;
      let frame;
      while ((frame = frames.nextFrame()) !== null) {
        const targetTimeMs = (frameIndex / fps) * 1000;
        const elapsed = Date.now() - startTime;
        frameIndex++;

        // Skip frame if we're behind by more than one frame period
        if (elapsed > targetTimeMs + delayMs) continue;

        // Write each line at its exact row — no full-screen clear, no flash
        const lines = frame.split("\n");
        let out = "\x1B[H";
        for (let row = 0; row < lines.length; row++) {
          out += `\x1B[${row + 1};1H${lines[row]}`;
        }
        process.stdout.write(out);

        // Sleep until the next frame is due
        const remaining = targetTimeMs + delayMs - (Date.now() - startTime);
        if (remaining > 0) sleepMs(remaining);
      }

      cleanup();
    }
  } else {
    const art = convert(inputPath, args.options);

    if (args.output) {
      fs.writeFileSync(args.output, art, "utf-8");
      console.error(`Written to ${args.output}`);
    } else {
      console.log(art);
    }
  }
} catch (err) {
  console.error(`Error: ${err.message}`);
  process.exit(1);
}
