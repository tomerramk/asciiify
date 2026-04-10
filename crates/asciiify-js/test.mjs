import { readFileSync } from "node:fs";
import { join } from "node:path";
import { describe, it } from "node:test";
import assert from "node:assert/strict";

import { convert, convertBytes, Converter } from "./index.js";

const FIXTURE = join(
  import.meta.dirname,
  "..",
  "..",
  "tests",
  "fixtures",
  "gradient.png",
);

describe("convert()", () => {
  it("returns correct dimensions", () => {
    const result = convert(FIXTURE, { width: 40, height: 10 });
    const lines = result.split("\n");
    assert.equal(lines.length, 10);
    assert.ok(lines.every((l) => l.length === 40));
  });

  it("supports ascii mode", () => {
    const result = convert(FIXTURE, { mode: "ascii", width: 20, height: 5 });
    assert.equal(result.split("\n").length, 5);
  });

  it("supports half-block mode", () => {
    const result = convert(FIXTURE, {
      mode: "half-block",
      width: 20,
      height: 5,
    });
    const lines = result.split("\n");
    assert.equal(lines.length, 5);
    assert.ok(lines.every((l) => l.length === 20));
  });

  it("supports braille mode", () => {
    const result = convert(FIXTURE, { mode: "braille", width: 20, height: 5 });
    const lines = result.split("\n");
    assert.equal(lines.length, 5);
    assert.ok(lines.every((l) => l.length === 20));
  });

  it("invert changes output", () => {
    const normal = convert(FIXTURE, { width: 20, height: 5 });
    const inverted = convert(FIXTURE, { width: 20, height: 5, invert: true });
    assert.notEqual(normal, inverted);
  });

  it("custom charset restricts characters", () => {
    const result = convert(FIXTURE, { width: 20, height: 5, charset: " .oO@" });
    const allowed = new Set(" .oO@\n");
    assert.ok([...result].every((c) => allowed.has(c)));
  });

  it("throws on nonexistent file", () => {
    assert.throws(() =>
      convert("nonexistent_12345.png", { width: 10, height: 5 }),
    );
  });

  it("throws on invalid mode", () => {
    assert.throws(() =>
      convert(FIXTURE, { mode: "invalid", width: 10, height: 5 }),
    );
  });
});

describe("convertBytes()", () => {
  const data = readFileSync(FIXTURE);

  it("returns correct dimensions", () => {
    const result = convertBytes(data, { width: 20, height: 5 });
    assert.equal(result.split("\n").length, 5);
  });

  it("works with all modes", () => {
    for (const mode of ["ascii", "half-block", "braille"]) {
      const result = convertBytes(data, { mode, width: 20, height: 5 });
      const lines = result.split("\n");
      assert.equal(lines.length, 5, `mode=${mode}`);
      assert.ok(
        lines.every((l) => l.length === 20),
        `mode=${mode} width`,
      );
    }
  });

  it("throws on invalid bytes", () => {
    assert.throws(() =>
      convertBytes(Buffer.from("not an image"), { width: 10, height: 5 }),
    );
  });
});

describe("Converter", () => {
  it("convert returns correct dimensions", () => {
    const conv = new Converter({ width: 30, height: 8 });
    const result = conv.convert(FIXTURE);
    const lines = result.split("\n");
    assert.equal(lines.length, 8);
    assert.ok(lines.every((l) => l.length === 30));
  });

  it("convertBytes works", () => {
    const conv = new Converter({ width: 20, height: 5 });
    const data = readFileSync(FIXTURE);
    const result = conv.convertBytes(data);
    assert.equal(result.split("\n").length, 5);
  });

  it("is reusable", () => {
    const conv = new Converter({ width: 20, height: 5 });
    const r1 = conv.convert(FIXTURE);
    const r2 = conv.convert(FIXTURE);
    assert.equal(r1, r2);
  });

  it("invert changes output", () => {
    const normal = new Converter({ width: 20, height: 5 });
    const inverted = new Converter({ width: 20, height: 5, invert: true });
    assert.notEqual(normal.convert(FIXTURE), inverted.convert(FIXTURE));
  });

  it("throws on invalid mode", () => {
    assert.throws(() => new Converter({ mode: "bad" }));
  });
});
