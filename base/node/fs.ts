import { readFile, stat } from "fs/promises";

export const exists = async (path: string): Promise<boolean> => {
  try {
    await stat(path);
  } catch (err) {
    return false;
  }
  return true;
};

export const readJson = async <T = any>(path: string): Promise<T> => {
  try {
    return await readFile(path, "utf8").then((r) => JSON.parse(r));
  } catch (e) {
    console.trace(`Error reading json file: ${path}`);
    throw e;
  }
};
