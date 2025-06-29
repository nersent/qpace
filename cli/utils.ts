import { readdir } from "node:fs/promises";
import { NodePackageManager } from "~/compiler/config";
import { exec, ExecResult } from "~/base/node/exec";

export const detectNodePackageManager = async (
  cwd: string,
): Promise<NodePackageManager | undefined> => {
  const files = await readdir(cwd);
  if (files.includes("pnpm-lock.yaml")) {
    return "pnpm";
  }
  if (files.includes("yarn.lock")) {
    return "yarn";
  }
  if (files.includes("bun.lockb")) {
    return "bun";
  }
  if (files.includes("package-lock.json")) {
    return "npm";
  }
  return;
};

export const installNodePackage = async (
  name: string,
  {
    cwd,
    manager = "npm",
    verbose = false,
    dev,
  }: {
    cwd?: string;
    manager?: NodePackageManager;
    verbose?: boolean;
    dev?: boolean;
  },
): Promise<ExecResult> => {
  const _items = name;
  if (manager === "pnpm") {
    return await exec({
      command: `pnpm install ${_items} ${dev ? "--save-dev" : ""}`,
      cwd,
      verbose,
    });
  }
  if (manager === "yarn") {
    return await exec({
      command: `yarn add ${_items} ${dev ? "--dev" : ""}`,
      cwd,
      verbose,
    });
  }
  if (manager === "bun") {
    return await exec({
      command: `bun add ${_items} ${dev ? "--dev" : ""}`,
      cwd,
      verbose,
    });
  }
  return await exec({
    command: `npm install ${_items} ${dev ? "--save-dev" : ""}`,
    cwd,
    verbose,
  });
};
