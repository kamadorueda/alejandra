import wasmInit, { format } from "alejandra_front";
import { FormatterConfig, DEFAULT_CONFIG } from "~/types/config";

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
      // Initialize WASM - this will auto-load from node_modules
      await wasmInit();

      initialized = true;
    } catch (error) {
      console.error("initFormatter: Failed to initialize WASM formatter", error);
      initPromise = null;
      throw error;
    }
  })();

  return initPromise;
};

export const formatCode = (
  code: string,
  filename: string = "file.nix",
  config: FormatterConfig = DEFAULT_CONFIG
): string => {
  if (!initialized) {
    console.warn("formatCode: Formatter not initialized, returning input unchanged");
    return code;
  }

  const configJson = JSON.stringify(config);
  return format(code, filename, configJson);
};
