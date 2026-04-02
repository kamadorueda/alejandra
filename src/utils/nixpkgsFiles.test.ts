import { describe, expect, it } from "vitest";
import { getFiles, COMMIT } from "./nixpkgsFiles";

describe("nixpkgsFiles utilities", () => {
  describe("COMMIT", () => {
    it("is a valid 40-character commit hash", () => {
      expect(COMMIT).toMatch(/^[a-f0-9]{40}$/);
    });

    it("is a non-empty string", () => {
      expect(COMMIT).toBeDefined();
      expect(typeof COMMIT).toBe("string");
      expect(COMMIT.length).toBe(40);
    });
  });

  describe("getFiles", () => {
    it("returns an array of strings", async () => {
      const result = await getFiles();
      expect(Array.isArray(result)).toBe(true);
      expect(result.length).toBeGreaterThan(0);
      expect(result.every((item) => typeof item === "string")).toBe(true);
    });

    it("returns files that end with .nix", async () => {
      const result = await getFiles();
      expect(result.every((file) => file.endsWith(".nix"))).toBe(true);
    });

    it("contains valid Nix file paths", async () => {
      const result = await getFiles();
      expect(result.length).toBeGreaterThan(100); // Should have 200+ files
      expect(result.some((f) => f.includes("pkgs/") || f.includes("nixos/"))).toBe(true);
    });
  });
});
