import { FSWatcher, Stats } from "fs";
import { mkdtemp, readFile, stat, writeFile } from "fs/promises";
import { FileHandle, mkdir as _mkdir, readdir, unlink } from "node:fs/promises";
import { tmpdir } from "os";
import { resolve } from "path";

import { watch } from "chokidar";

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

// eslint-disable-next-line @typescript-eslint/explicit-function-return-type
export const watchPaths = async (
  {
    included,
    excluded = [],
    delay = 500,
  }: { included: string[]; excluded?: string[]; delay?: number },
  onChange: (filename: string) => Promise<void>,
) => {
  let watcher: FSWatcher | undefined;
  let runTimeout: NodeJS.Timeout | undefined;

  watcher?.close();
  watcher = watch(included, {
    ignored: excluded,
    followSymlinks: true,
    ignoreInitial: true,
  });
  watcher.on("change", (filename: string, stats: Stats) => {
    clearTimeout(runTimeout);
    runTimeout = setTimeout(() => {
      onChange(filename);
    }, delay);
  });

  return { cancel: (): void => watcher?.close() };
};

export const clearDir = async (path: string): Promise<void> => {
  if (!(await exists(path))) {
    return;
  }

  const files = await readdir(path);

  await Promise.all(
    files.map(async (file) => await deleteFile(resolve(path, file))),
  );
};

export const deleteFile = async (path: string): Promise<void> => {
  try {
    await unlink(path);
  } catch (err) {
    if ((err as any)?.code !== "ENOENT") {
      throw err;
    }
  }
};
