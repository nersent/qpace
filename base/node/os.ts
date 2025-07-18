import os from "os";

import _which from "which";

export const isMacOs = (): boolean => {
  return process.platform === "darwin";
};

export const isWindows = (): boolean => {
  return process.platform === "win32";
};

export const isLinux = (): boolean => {
  return process.platform === "linux";
};

export const workerCpuCount = (): number => {
  const cpus = os.cpus();
  if (isWindows()) {
    return Math.max(1, cpus.length - 2);
  }
  return cpus.length;
};

export const which = async (cmd: string): Promise<string | undefined> => {
  const path = await _which(cmd, { nothrow: true });
  return path as any;
};
