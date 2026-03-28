import { describe, expect, it, beforeEach, afterEach, vi } from "vitest";
import { encodeState, decodeState, getStateFromUrl, setStateInUrl } from "./permalink";

describe("permalink utilities", () => {
  describe("encodeState", () => {
    it("encodes state to a base64 string", () => {
      const state = { code: "hello world" };
      const encoded = encodeState(state);
      expect(typeof encoded).toBe("string");
      expect(encoded).toMatch(/^[A-Za-z0-9+/=]+$/);
    });

    it("handles empty code strings", () => {
      const state = { code: "" };
      const encoded = encodeState(state);
      expect(typeof encoded).toBe("string");
    });

    it("handles special characters in code", () => {
      const state = { code: '{ lib, stdenv }:\nstdenv.mkDerivation { name = "test"; }' };
      const encoded = encodeState(state);
      expect(typeof encoded).toBe("string");
    });

    it("handles unicode characters", () => {
      const state = { code: "# Comment with emoji 🎉" };
      const encoded = encodeState(state);
      expect(typeof encoded).toBe("string");
    });
  });

  describe("decodeState", () => {
    it("decodes a valid encoded state", () => {
      const original = { code: "hello world" };
      const encoded = encodeState(original);
      const decoded = decodeState(encoded);
      expect(decoded).toEqual(original);
    });

    it("returns null for invalid base64", () => {
      const decoded = decodeState("!!!invalid base64!!!!");
      expect(decoded).toBeNull();
    });

    it("returns null for invalid JSON", () => {
      const invalidJson = btoa("not json");
      const decoded = decodeState(invalidJson);
      expect(decoded).toBeNull();
    });

    it("returns null for empty string", () => {
      const decoded = decodeState("");
      expect(decoded).toBeNull();
    });

    it("handles large code strings", () => {
      const largeCode = "x".repeat(10000);
      const state = { code: largeCode };
      const encoded = encodeState(state);
      const decoded = decodeState(encoded);
      expect(decoded).toEqual(state);
    });
  });

  describe("round-trip encoding/decoding", () => {
    it("maintains data integrity through encode-decode cycle", () => {
      const testCases = [
        { code: "" },
        { code: "simple" },
        { code: '{ lib }:\nlib.mkIf true "value"' },
        { code: "# Comment\n# More comments\nx = 42;" },
        { code: 'with import <nixpkgs> {}; mkShell { buildInputs = [ python3 ]; }' },
      ];

      testCases.forEach((state) => {
        const encoded = encodeState(state);
        const decoded = decodeState(encoded);
        expect(decoded).toEqual(state);
      });
    });
  });

  describe("getStateFromUrl", () => {
    beforeEach(() => {
      delete (window as any).location;
      window.location = { hash: "" } as any;
    });

    afterEach(() => {
      window.location = { hash: "" } as any;
    });

    it("returns null when hash is empty", () => {
      window.location = { hash: "" } as any;
      const state = getStateFromUrl();
      expect(state).toBeNull();
    });

    it("returns null when hash is just #", () => {
      window.location = { hash: "#" } as any;
      const state = getStateFromUrl();
      expect(state).toBeNull();
    });

    it("decodes state from URL hash", () => {
      const original = { code: "test code" };
      const encoded = encodeState(original);
      window.location = { hash: `#${encoded}` } as any;
      const state = getStateFromUrl();
      expect(state).toEqual(original);
    });

    it("returns null when hash contains invalid state", () => {
      window.location = { hash: "#invalid!!!!" } as any;
      const state = getStateFromUrl();
      expect(state).toBeNull();
    });
  });

  describe("setStateInUrl", () => {
    let replaceStateSpy: any;

    beforeEach(() => {
      replaceStateSpy = vi.spyOn(window.history, "replaceState");
    });

    afterEach(() => {
      replaceStateSpy.mockRestore();
    });

    it("updates URL hash with encoded state", () => {
      const state = { code: "new code" };
      setStateInUrl(state);
      expect(replaceStateSpy).toHaveBeenCalled();
    });

    it("uses # prefix in URL", () => {
      const state = { code: "test" };
      setStateInUrl(state);
      const callArgs = replaceStateSpy.mock.calls[0];
      const url = callArgs[2] as string;
      expect(url).toMatch(/^#/);
    });

    it("encodes state in URL hash", () => {
      const state = { code: "test code" };
      setStateInUrl(state);
      const callArgs = replaceStateSpy.mock.calls[0];
      const url = callArgs[2] as string;
      const hash = url.slice(1);
      const decoded = decodeState(hash);
      expect(decoded).toEqual(state);
    });

    it("maintains history state as null", () => {
      const state = { code: "test" };
      setStateInUrl(state);
      const callArgs = replaceStateSpy.mock.calls[0];
      expect(callArgs[0]).toBeNull();
      expect(callArgs[1]).toBe("");
    });
  });
});
