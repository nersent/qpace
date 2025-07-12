import chalk from "chalk";
import { mkdir } from "fs/promises";
import { resolve } from "path";
import { exec } from "~/base/node/exec";
import { clearDir } from "~/base/node/fs";
import { BuildTarget } from "~/cli/compiler";

const WORKSPACE_PATH = process.env["BAZED_WORKSPACE_ROOT"] ?? process.cwd();

const main = async (): Promise<void> => {
  const root = resolve(WORKSPACE_PATH, "algoalpha");
  const baseCommand = "pnpm bazed run //cli:main --verbose --";
  await exec({
    command: `${baseCommand} --version`,
    io: true,
  });
  const outDir = resolve(WORKSPACE_PATH, "out/algoalpha");
  console.log(chalk.magentaBright(outDir));
  await clearDir(outDir);
  await mkdir(outDir, { recursive: true });

  const targets: BuildTarget[] = [
    "python-x86_64-windows",
    "python-x86_64-macos",
    "python-x86_64-linux",
    "python-arm64-macos",
    "python-arm64-linux",
    "js",
    //
    // "wasm-unknown-unknown",
    // "node-x86_64-windows",
    // "node-x86_64-linux",
    // "node-x86_64-macos",
    // "node-arm64-linux",
    // "node-arm64-macos",
  ];

  for (const target of targets) {
    await exec({
      command: `${baseCommand} build --target ${target} --out-dir ${outDir} --verbose --cwd ${root}`,
      verbose: true,
      cwd: root,
      throw: true,
      env: {
        ...process.env,
        // DEV: true,
      },
    });
  }

  // python -m twine upload out/algoalpha/*.whl
  // npm publish --access public out/algoalpha/algoalpha-0.0.1.tgz
};

main();
