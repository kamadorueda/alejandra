export const COMMIT = "3a3fa1b0f20ae56cd52247c7af708acf4d034430";

// FILES are now lazy-loaded from public/nixpkgsFiles.json
// This reduces the initial bundle size significantly (~1.3MB)
let cachedFiles: string[] | null = null;

export const getFiles = async (): Promise<string[]> => {
  if (cachedFiles) {
    return cachedFiles;
  }

  const response = await fetch("/nixpkgsFiles.json");
  if (!response.ok) {
    throw new Error(`Failed to load nixpkgsFiles.json: ${response.statusText}`);
  }

  cachedFiles = await response.json();
  return cachedFiles;
};

// Test utility: reset the cache (only used in tests)
export const _resetCache = (): void => {
  cachedFiles = null;
};
