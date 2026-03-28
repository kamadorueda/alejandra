import { describe, expect, it, vi, beforeEach, afterEach } from "vitest";
import { getFiles, COMMIT, _resetCache } from "./nixpkgsFiles";

describe("nixpkgsFiles utilities", () => {
  beforeEach(() => {
    _resetCache(); // Reset the cached files between tests
    global.fetch = vi.fn();
  });

  afterEach(() => {
    vi.restoreAllMocks();
  });

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
    it("fetches and returns files list from public/nixpkgsFiles.json", async () => {
      const mockFiles = ["pkgs/a.nix", "pkgs/b.nix", "pkgs/c.nix"];
      (global.fetch as any).mockResolvedValueOnce({
        ok: true,
        json: async () => mockFiles,
      });

      const result = await getFiles();
      expect(result).toEqual(mockFiles);
      expect(global.fetch).toHaveBeenCalledWith("/nixpkgsFiles.json");
    });

    it("returns an array of strings", async () => {
      const mockFiles = ["file1.nix", "file2.nix"];
      (global.fetch as any).mockResolvedValueOnce({
        ok: true,
        json: async () => mockFiles,
      });

      const result = await getFiles();
      expect(Array.isArray(result)).toBe(true);
      expect(result.every((item) => typeof item === "string")).toBe(true);
    });

    it("caches the result on subsequent calls", async () => {
      const mockFiles = ["file1.nix", "file2.nix"];
      (global.fetch as any).mockResolvedValueOnce({
        ok: true,
        json: async () => mockFiles,
      });

      const result1 = await getFiles();
      const result2 = await getFiles();

      expect(result1).toBe(result2); // Same reference due to caching
      expect(global.fetch).toHaveBeenCalledTimes(1);
    });

    it("does not make additional fetch requests when cache is populated", async () => {
      const mockFiles = ["file1.nix"];
      (global.fetch as any).mockResolvedValueOnce({
        ok: true,
        json: async () => mockFiles,
      });

      await getFiles();
      await getFiles();
      await getFiles();

      expect(global.fetch).toHaveBeenCalledTimes(1);
    });

    it("throws error when fetch fails", async () => {
      (global.fetch as any).mockRejectedValueOnce(new Error("Network error"));

      await expect(getFiles()).rejects.toThrow("Network error");
    });

    it("throws error when response is not ok", async () => {
      (global.fetch as any).mockResolvedValueOnce({
        ok: false,
        statusText: "Not Found",
      });

      await expect(getFiles()).rejects.toThrow(
        "Failed to load nixpkgsFiles.json: Not Found"
      );
    });

    it("handles empty files array", async () => {
      (global.fetch as any).mockResolvedValueOnce({
        ok: true,
        json: async () => [],
      });

      const result = await getFiles();
      expect(result).toEqual([]);
    });

    it("handles large files array", async () => {
      const mockFiles = Array.from({ length: 10000 }, (_, i) => `file${i}.nix`);
      (global.fetch as any).mockResolvedValueOnce({
        ok: true,
        json: async () => mockFiles,
      });

      const result = await getFiles();
      expect(result.length).toBe(10000);
      expect(global.fetch).toHaveBeenCalledTimes(1);
    });

    it("preserves file paths with special characters", async () => {
      const mockFiles = [
        "pkgs/test-package/file+name.nix",
        "pkgs/test/path with spaces/file.nix",
      ];
      (global.fetch as any).mockResolvedValueOnce({
        ok: true,
        json: async () => mockFiles,
      });

      const result = await getFiles();
      expect(result).toEqual(mockFiles);
    });

    it("handles various HTTP error codes when response not ok", async () => {
      const errorCodes = [400, 403, 404, 500, 502];

      for (const code of errorCodes) {
        (global.fetch as any).mockResolvedValueOnce({
          ok: false,
          status: code,
          statusText: `Error ${code}`,
        });

        try {
          await getFiles();
        } catch (error) {
          expect((error as Error).message).toContain("Failed to load");
        }
      }
    });
  });
});
