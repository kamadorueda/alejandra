import { FILES, COMMIT } from "./nixpkgsFiles";

export const randomPath = (): string =>
  FILES[Math.floor(Math.random() * FILES.length)];

export const path2url = (path: string): string =>
  `https://raw.githubusercontent.com/nixos/nixpkgs/${COMMIT}/${path}`;

export const get = async (path: string): Promise<string> => {
  const url = path2url(path);

  try {
    const response = await fetch(url);
    return await response.text();
  } catch (error) {
    return `# An error occurred while fetching ${url}\n# ${error}`;
  }
};
