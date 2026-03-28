import * as wasm from "alejandra_front";

let initialized = false;
let initPromise: Promise<void> | null = null;

export const initFormatter = async () => {
  // If already initialized, return immediately
  if (initialized) {
    return;
  }

  // If initialization is in progress, wait for it
  if (initPromise) {
    return initPromise;
  }

  // Start initialization
  initPromise = (async () => {
    try {
      console.log("initFormatter: Starting WASM initialization");
      const result = await wasm.default();
      console.log("initFormatter: WASM loaded successfully", result);
      initialized = true;
    } catch (error) {
      console.error("initFormatter: Failed to initialize WASM formatter", error);
      initPromise = null;
      throw error;
    }
  })();

  return initPromise;
};

export const formatCode = (code: string, filename: string = "file.nix"): string => {
  if (!initialized) {
    console.warn("formatCode: Formatter not initialized, returning input unchanged");
    return code;
  }

  try {
    const formatted = wasm.format(code, filename);
    console.log("formatCode: Formatting successful");
    return formatted;
  } catch (error) {
    console.error("formatCode: Formatting error", error);
    return code;
  }
};
