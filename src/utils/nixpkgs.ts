import { COMMIT, getFiles } from "./nixpkgsFiles";

export const randomPath = async (): Promise<string> => {
  const files = await getFiles();
  return files[Math.floor(Math.random() * files.length)];
};

export const path2url = (path: string): string =>
  `https://raw.githubusercontent.com/nixos/nixpkgs/${COMMIT}/${path}`;

export const get = async (path: string): Promise<string> => {
  const url = path2url(path);

  try {
    const response = await fetch(url);
    if (!response.ok) {
      throw new Error(`HTTP ${response.status}: ${response.statusText}`);
    }
    return await response.text();
  } catch (error) {
    return `# An error occurred while fetching ${url}\n# ${error}`;
  }
};
