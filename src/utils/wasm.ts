import * as wasm from "alejandra_front";

let initialized = false;

export const initFormatter = async () => {
  if (!initialized) {
    await wasm.default();
    initialized = true;
  }
};

export const formatCode = (code: string, filename: string = "file.nix"): string => {
  return wasm.format(code, filename);
};
