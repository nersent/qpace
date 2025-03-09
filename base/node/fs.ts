import { mkdtemp, readFile, stat, writeFile } from "fs/promises";
import { FileHandle, mkdir as _mkdir } from "node:fs/promises";
import { tmpdir } from "os";
import { resolve } from "path";

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

const maybeFmtJson = (data: any, format?: boolean): string => {
  if (format) return JSON.stringify(data, null, 2);
  return JSON.stringify(data);
};

export const writeJson = async <T>(
  path: string,
  data: T,
  formatted = false,
): Promise<void> => {
  await writeFile(path, maybeFmtJson(data, formatted), "utf8");
};

export const tmpDir = async (prefix?: string): Promise<string> => {
  prefix = resolve(tmpdir(), prefix ?? Date.now().toString());
  return await mkdtemp(prefix);
};

export const createDir = async (path: string): Promise<void> => {
  if (!(await exists(path))) {
    await _mkdir(path, { recursive: true });
  }
};
