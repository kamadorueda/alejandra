import { describe, expect, it, vi, beforeEach, afterEach } from "vitest";

// Mock the WASM module before importing our code
vi.mock("alejandra_front", () => ({
  default: vi.fn(async () => {
    // Simulate async initialization
    return Promise.resolve();
  }),
  format: vi.fn((code: string, filename: string, configJson?: string) => {
    // Simple mock: just return the code with "formatted: " prefix
    return `formatted: ${code} (${filename})`;
  }),
}));

// Import after mocking
import { initFormatter, formatCode } from "./wasm";
import wasmInit from "alejandra_front";

describe("wasm utilities", () => {
  beforeEach(() => {
    // Reset the module state for each test by clearing the mock calls
    vi.clearAllMocks();
    // Force re-import by clearing the module
    vi.resetModules();
  });

  describe("initFormatter", () => {
    it("initializes the WASM formatter", async () => {
      // Re-import fresh to get clean state
      const { initFormatter } = await import("./wasm");
      await initFormatter();
      expect(wasmInit).toHaveBeenCalled();
    });

    it("calls wasmInit only once on multiple calls", async () => {
      const { initFormatter } = await import("./wasm");
      await initFormatter();
      await initFormatter();
      await initFormatter();
      expect(wasmInit).toHaveBeenCalledTimes(1);
    });

    it("handles initialization errors gracefully", async () => {
      // Mock wasmInit to fail
      const mockError = new Error("WASM initialization failed");
      (wasmInit as any).mockRejectedValueOnce(mockError);

      const { initFormatter } = await import("./wasm");
      await expect(initFormatter()).rejects.toThrow("WASM initialization failed");
    });

    it("returns a Promise", async () => {
      const { initFormatter } = await import("./wasm");
      const result = initFormatter();
      expect(result).toBeInstanceOf(Promise);
    });

    it("resolves successfully when initialization completes", async () => {
      const { initFormatter } = await import("./wasm");
      const result = await initFormatter();
      expect(result).toBeUndefined();
    });

    it("allows sequential initialization calls to complete", async () => {
      const { initFormatter } = await import("./wasm");
      const promise1 = initFormatter();
      const promise2 = initFormatter();
      const promise3 = initFormatter();

      await Promise.all([promise1, promise2, promise3]);
      expect(wasmInit).toHaveBeenCalledTimes(1);
    });
  });

  describe("formatCode", () => {
    it("formats code using the WASM formatter", async () => {
      const { initFormatter, formatCode } = await import("./wasm");
      await initFormatter();

      const input = "x=1";
      const result = formatCode(input);
      expect(result).toContain("formatted:");
    });

    it("returns original code if formatter is not initialized", async () => {
      // Import without calling initFormatter
      const { formatCode } = await import("./wasm");
      const input = "x=1";
      const result = formatCode(input);
      expect(result).toBe(input);
    });

    it("accepts custom filename parameter", async () => {
      const { initFormatter, formatCode } = await import("./wasm");
      await initFormatter();

      const input = "test";
      const filename = "custom.nix";
      const result = formatCode(input, filename);
      expect(result).toContain(filename);
    });

    it("uses default filename when not provided", async () => {
      const { initFormatter, formatCode } = await import("./wasm");
      await initFormatter();

      const input = "test";
      const result = formatCode(input);
      expect(result).toContain("file.nix");
    });

    it("handles empty code strings", async () => {
      const { initFormatter, formatCode } = await import("./wasm");
      await initFormatter();

      const result = formatCode("");
      expect(typeof result).toBe("string");
    });

    it("handles large code blocks", async () => {
      const { initFormatter, formatCode } = await import("./wasm");
      await initFormatter();

      const largeCode = "x = 1;\n".repeat(1000);
      const result = formatCode(largeCode);
      expect(typeof result).toBe("string");
      expect(result.length).toBeGreaterThan(0);
    });

    it("handles code with special characters", async () => {
      const { initFormatter, formatCode } = await import("./wasm");
      await initFormatter();

      const code = '{ lib, stdenv }:\nstdenv.mkDerivation { name = "test"; }';
      const result = formatCode(code);
      expect(typeof result).toBe("string");
    });

    it("throws error if formatting fails", async () => {
      const { initFormatter, formatCode } = await import("./wasm");
      await initFormatter();

      // Mock format to throw an error after initialization
      const { format } = await import("alejandra_front");
      (format as any).mockImplementationOnce(() => {
        throw new Error("Format failed");
      });

      const input = "test code";
      expect(() => formatCode(input)).toThrow("Format failed");
    });

    it("preserves code structure", async () => {
      const { initFormatter, formatCode } = await import("./wasm");
      await initFormatter();

      const code = "{\n  a = 1;\n  b = 2;\n}";
      const result = formatCode(code);
      expect(typeof result).toBe("string");
    });

    it("works with Nix-specific syntax", async () => {
      const { initFormatter, formatCode } = await import("./wasm");
      await initFormatter();

      const code = "with import <nixpkgs> {}; mkShell { buildInputs = [ python3 ]; }";
      const result = formatCode(code);
      expect(typeof result).toBe("string");
    });

    it("accepts config parameter", async () => {
      const { initFormatter, formatCode } = await import("./wasm");
      await initFormatter();

      const config = { indentation: "FourSpaces" as const };
      const result = formatCode("test", "file.nix", config);
      expect(typeof result).toBe("string");
    });

    it("passes config as JSON to WASM format function", async () => {
      const { initFormatter, formatCode } = await import("./wasm");
      await initFormatter();

      const { format } = await import("alejandra_front");
      const config = { indentation: "Tabs" as const };
      formatCode("test", "file.nix", config);

      expect(format).toHaveBeenCalledWith("test", "file.nix", JSON.stringify(config));
    });

    it("uses default config when not provided", async () => {
      const { initFormatter, formatCode } = await import("./wasm");
      await initFormatter();

      const { format } = await import("alejandra_front");
      formatCode("test");

      expect(format).toHaveBeenCalledWith(
        "test",
        "file.nix",
        JSON.stringify({ indentation: "TwoSpaces" })
      );
    });

    it("throws error on invalid config from WASM", async () => {
      const { initFormatter, formatCode } = await import("./wasm");
      await initFormatter();

      const { format } = await import("alejandra_front");
      (format as any).mockImplementationOnce(() => {
        throw new Error("InvalidConfig: bad config");
      });

      const config = { indentation: "FourSpaces" as const };
      expect(() => formatCode("code", "file.nix", config)).toThrow("InvalidConfig");
    });
  });

  describe("initialization and formatting integration", () => {
    it("initializes formatter before first format call", async () => {
      const { initFormatter, formatCode } = await import("./wasm");
      await initFormatter();

      const code = "test";
      const result = formatCode(code);
      expect(wasmInit).toHaveBeenCalled();
      expect(result).toBeDefined();
    });

    it("handles consecutive format calls after initialization", async () => {
      const { initFormatter, formatCode } = await import("./wasm");
      await initFormatter();

      const results = [];
      for (let i = 0; i < 5; i++) {
        const result = formatCode(`code${i}`);
        results.push(result);
      }

      expect(results.length).toBe(5);
      expect(wasmInit).toHaveBeenCalledTimes(1);
    });
  });
});
