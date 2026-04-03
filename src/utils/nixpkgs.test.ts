import { describe, expect, it, vi } from "vitest";

vi.mock("~/utils/nixpkgs", () => ({
  getRandomFile: vi.fn(async () => ({
    path: "pkgs/test/default.nix",
    content: "# Mock nix content",
  })),
}));

import { getRandomFile } from "./nixpkgs";

describe("nixpkgs utilities", () => {
  describe("getRandomFile", () => {
    it("returns an object with path and content properties", async () => {
      const result = await getRandomFile();
      expect(result).toHaveProperty("path");
      expect(result).toHaveProperty("content");
      expect(typeof result.path).toBe("string");
      expect(typeof result.content).toBe("string");
    });

    it("returns a valid .nix file path", async () => {
      const result = await getRandomFile();
      expect(result.path).toMatch(/\.nix$/);
      expect(result.path.includes("pkgs/") || result.path.includes("nixos/")).toBe(true);
    });

    it("returns non-empty content", async () => {
      const result = await getRandomFile();
      expect(result.content.length).toBeGreaterThan(0);
    });

    it("returns different paths on multiple calls", async () => {
      (getRandomFile as any).mockResolvedValueOnce({ path: "pkgs/a.nix", content: "a" });
      (getRandomFile as any).mockResolvedValueOnce({ path: "pkgs/b.nix", content: "b" });
      (getRandomFile as any).mockResolvedValueOnce({ path: "pkgs/c.nix", content: "c" });

      const results = new Set();
      for (let i = 0; i < 3; i++) {
        const result = await getRandomFile();
        results.add(result.path);
      }

      expect(results.size).toBe(3);
    });
  });
});
