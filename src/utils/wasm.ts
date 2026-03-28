// TEMPORARILY STUBBED - WASM integration disabled for UI testing
// Real WASM will be re-enabled after verifying frontend functionality

let initialized = false;

export const initFormatter = async () => {
  if (!initialized) {
    console.log("Formatter initialized (stub mode - no WASM)");
    initialized = true;
  }
};

export const formatCode = (code: string, _filename: string = "file.nix"): string => {
  // Stub: return input unchanged for now
  console.log("Format called (stub mode - returning input unchanged)");
  return code;
};
