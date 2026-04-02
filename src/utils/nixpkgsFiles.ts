import filesData from "../data/nixpkgsFiles.json";

export const COMMIT = "3a3fa1b0f20ae56cd52247c7af708acf4d034430";

export const getFiles = async (): Promise<string[]> => {
  return filesData;
};

// Test utility: reset the cache (only used in tests)
export const _resetCache = (): void => {
  // No-op with direct import, but kept for test compatibility
};

// Test utility: reset the cache (only used in tests)
export const _resetCache = (): void => {
  cachedFiles = null;
};
