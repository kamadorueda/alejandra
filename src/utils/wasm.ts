import * as wasm from "alejandra_front";

let initialized = false;

export const initFormatter = async () => {
  if (!initialized) {
    try {
      await wasm.default();
      console.log("Formatter initialized with WASM");
      initialized = true;
    } catch (error) {
      console.error("Failed to initialize WASM formatter:", error);
      throw error;
    }
  }
};

export const formatCode = (code: string, filename: string = "file.nix"): string => {
  if (!initialized) {
    console.warn("Formatter not initialized, returning input unchanged");
    return code;
  }

  try {
    return wasm.format(code, filename);
  } catch (error) {
    console.error("Formatting error:", error);
    return code;
  }
};
