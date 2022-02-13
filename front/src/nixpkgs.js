import { COMMIT, FILES } from "./nixpkgsFiles";

export const randomPath = () => FILES[Math.floor(Math.random() * FILES.length)];

export const path2url = (path) =>
  `https://raw.githubusercontent.com/nixos/nixpkgs/${COMMIT}/${path}`;

export const get = async (path) => {
  const url = path2url(path);

  try {
    const response = await fetch(url);

    return await response.text();
  } catch (error) {
    return `# An error ocurred while fetching ${url}\n# ${error}`;
  }
};
