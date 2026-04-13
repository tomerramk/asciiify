#!/usr/bin/env node

"use strict";

const fs = require("fs");
const path = require("path");
const { convert } = require("../index.js");

function printUsage() {
  console.error(
    `Usage: asciiify [options] <input>

Convert images to ASCII art

Options:
  -m, --mode <mode>        Output mode: ascii, half-block, braille (default: ascii)
  -w, --width <number>     Output width in characters
  -H, --height <number>    Output height in characters
  --invert                 Invert brightness
  --charset <chars>        Custom ASCII character ramp (ascii mode only)
  -o, --output <file>      Output to file instead of stdout
  -h, --help               Show this help message`
  );
}

function parseArgs(argv) {
  const args = { input: null, options: {} };
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

try {
  const art = convert(inputPath, args.options);

  if (args.output) {
    fs.writeFileSync(args.output, art, "utf-8");
    console.error(`Written to ${args.output}`);
  } else {
    console.log(art);
  }
} catch (err) {
  console.error(`Error: ${err.message}`);
  process.exit(1);
}
