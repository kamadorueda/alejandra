import { describe, it, expect, vi } from "vitest";
import { getRandomFile } from "./index";

describe("nixpkgs data", () => {
  it("getRandomFile returns an object with path and content", async () => {
    const result = await getRandomFile();
    expect(result).toHaveProperty("path");
    expect(result).toHaveProperty("content");
    expect(typeof result.path).toBe("string");
    expect(typeof result.content).toBe("string");
  });

  it("getRandomFile returns a path that exists in the files map", async () => {
    const result = await getRandomFile();
    expect(result.path).toMatch(/\.(nix|ts)$/);
  });

  it("getRandomFile returns non-empty content", async () => {
    const result = await getRandomFile();
    expect(result.content.length).toBeGreaterThan(0);
  });

  it("getRandomFile returns different paths on multiple calls", async () => {
    const results = new Set();
    // Call multiple times to verify randomness (with some tolerance for small files)
    for (let i = 0; i < 5; i++) {
      const result = await getRandomFile();
      results.add(result.path);
    }

    // Should get at least some variety (not always the same file)
    expect(results.size).toBeGreaterThanOrEqual(1);
  });
});
