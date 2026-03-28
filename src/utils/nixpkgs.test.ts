import { describe, expect, it, vi, beforeEach, afterEach } from "vitest";
import { path2url, get, randomPath } from "./nixpkgs";
import { COMMIT, _resetCache } from "./nixpkgsFiles";

describe("nixpkgs utilities", () => {
  describe("path2url", () => {
    it("converts a package path to GitHub raw URL", () => {
      const path = "pkgs/development/compilers/rust/default.nix";
      const url = path2url(path);
      expect(url).toBe(
        `https://raw.githubusercontent.com/nixos/nixpkgs/${COMMIT}/${path}`
      );
    });

    it("handles paths with special characters", () => {
      const path = "pkgs/test-package/file+name.nix";
      const url = path2url(path);
      expect(url).toContain(path);
      expect(url).toContain("raw.githubusercontent.com");
    });

    it("handles paths without leading slash", () => {
      const path = "pkgs/top-level/all-packages.nix";
      const url = path2url(path);
      expect(url).toContain(path);
    });

    it("constructs correct format for any path", () => {
      const path = "any/path/here";
      const url = path2url(path);
      expect(url).toMatch(/^https:\/\/raw\.githubusercontent\.com\/nixos\/nixpkgs\/[a-f0-9]{40}\/any\/path\/here$/);
    });

    it("includes the correct commit hash", () => {
      const url = path2url("test.nix");
      expect(url).toContain(COMMIT);
    });
  });

  describe("get", () => {
    beforeEach(() => {
      global.fetch = vi.fn();
    });

    afterEach(() => {
      vi.restoreAllMocks();
    });

    it("fetches file content successfully", async () => {
      const mockContent = "{ lib, stdenv }:\nstdenv.mkDerivation { }";
      (global.fetch as any).mockResolvedValueOnce({
        ok: true,
        text: async () => mockContent,
      });

      const result = await get("test.nix");
      expect(result).toBe(mockContent);
    });

    it("calls fetch with correct URL", async () => {
      (global.fetch as any).mockResolvedValueOnce({
        ok: true,
        text: async () => "content",
      });

      await get("pkgs/test.nix");
      expect(global.fetch).toHaveBeenCalledWith(
        expect.stringContaining("raw.githubusercontent.com")
      );
    });

    it("returns error message on HTTP error", async () => {
      (global.fetch as any).mockResolvedValueOnce({
        ok: false,
        status: 404,
        statusText: "Not Found",
      });

      const result = await get("missing.nix");
      expect(result).toContain("An error occurred");
      expect(result).toContain("HTTP 404");
    });

    it("returns error message on fetch failure", async () => {
      (global.fetch as any).mockRejectedValueOnce(new Error("Network error"));

      const result = await get("test.nix");
      expect(result).toContain("An error occurred");
      expect(result).toContain("Error");
    });

    it("handles empty file content", async () => {
      (global.fetch as any).mockResolvedValueOnce({
        ok: true,
        text: async () => "",
      });

      const result = await get("empty.nix");
      expect(result).toBe("");
    });

    it("handles large file content", async () => {
      const largeContent = "x".repeat(100000);
      (global.fetch as any).mockResolvedValueOnce({
        ok: true,
        text: async () => largeContent,
      });

      const result = await get("large.nix");
      expect(result).toBe(largeContent);
    });

    it("handles various HTTP error codes", async () => {
      const errorCodes = [400, 403, 404, 500, 502, 503];

      for (const code of errorCodes) {
        (global.fetch as any).mockResolvedValueOnce({
          ok: false,
          status: code,
          statusText: "Error",
        });

        const result = await get("test.nix");
        expect(result).toContain(`HTTP ${code}`);
      }
    });

    it("preserves newlines and special characters", async () => {
      const content = "line1\nline2\r\nline3\ttab\x00null";
      (global.fetch as any).mockResolvedValueOnce({
        ok: true,
        text: async () => content,
      });

      const result = await get("special.nix");
      expect(result).toBe(content);
    });
  });

  describe("randomPath", () => {
    beforeEach(() => {
      _resetCache(); // Reset getFiles cache between tests
      global.fetch = vi.fn();
    });

    afterEach(() => {
      vi.restoreAllMocks();
    });

    it("returns a path from the files list", async () => {
      const files = ["pkgs/a/file1.nix", "pkgs/b/file2.nix", "pkgs/c/file3.nix"];
      (global.fetch as any).mockResolvedValueOnce({
        ok: true,
        json: async () => files,
      });

      const result = await randomPath();
      expect(files).toContain(result);
    });

    it("returns different paths on multiple calls", async () => {
      const files = Array.from({ length: 100 }, (_, i) => `pkgs/file${i}.nix`);
      (global.fetch as any).mockResolvedValue({
        ok: true,
        json: async () => files,
      });

      const results = new Set();
      for (let i = 0; i < 10; i++) {
        const path = await randomPath();
        results.add(path);
      }

      // With 100 files and 10 calls, probability of getting all the same is extremely low
      expect(results.size).toBeGreaterThan(1);
    });

    it("handles single file in list", async () => {
      const files = ["pkgs/only/file.nix"];
      (global.fetch as any).mockResolvedValueOnce({
        ok: true,
        json: async () => files,
      });

      const result = await randomPath();
      expect(result).toBe("pkgs/only/file.nix");
    });

    it("throws error when fetch fails", async () => {
      (global.fetch as any).mockRejectedValueOnce(new Error("Network error"));

      await expect(randomPath()).rejects.toThrow();
    });

    it("throws error when response is not ok", async () => {
      (global.fetch as any).mockResolvedValueOnce({
        ok: false,
        statusText: "Error",
      });

      await expect(randomPath()).rejects.toThrow();
    });
  });
});
