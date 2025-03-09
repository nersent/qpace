import { resolve } from "path";

import chalk from "chalk";
import { Command, Option } from "commander";

import { loadQp } from "./qp";
import { RemoteDriver } from "./remote_driver";

import { exists, readJson, writeJson } from "~/base/node/fs";
import { isLinux, isMacOs, isWindows } from "~/base/node/os";
import {
  Config,
  getDefaultConfig,
  mergeConfigs,
  QPC_CONFIG_FILENAME,
  Target,
  TARGETS,
} from "~/compiler/common";
import { Client } from "~/lib/client";

const CWD_PATH = process.env["BAZED_WORKSPACE_ROOT"] ?? process.cwd();

const loadConfig = async (dir: string): Promise<Config> => {
  const configPath = resolve(dir, QPC_CONFIG_FILENAME);
  if (!(await exists(configPath))) {
    throw new Error(
      `Config file not found at ${configPath}. Try running 'qpace init'`,
    );
  }
  let config = await readJson(configPath);
  config = mergeConfigs(getDefaultConfig(), config);
  return config;
};

const loadOrCreateConfig = async (dir: string): Promise<Config> => {
  const configPath = resolve(dir, QPC_CONFIG_FILENAME);
  if (!(await exists(configPath))) {
    const config = getDefaultConfig();
    console.log(
      chalk.yellowBright(
        `Warning: Config file not found at ${configPath}. Creating a default config file.`,
      ),
    );
    await writeJson(configPath, config, true);
    return config;
  }
  return await readJson(configPath);
};

const BUILD_TARGETS = [...TARGETS, "python"] as const;

const tryMapBuildTarget = (
  buildTarget: typeof BUILD_TARGETS[number],
): Target | undefined => {
  if (TARGETS.includes(buildTarget as any)) {
    return buildTarget as Target;
  }
  const arch = process.arch;
  if (buildTarget === "python" && isMacOs() && arch === "arm64") {
    return "python-arm64-macos";
  }
  if (buildTarget === "python" && isMacOs() && arch === "x64") {
    return "python-x86_64-macos";
  }
  if (buildTarget === "python" && isWindows() && arch === "x64") {
    return "python-x86_64-windows";
  }
  if (buildTarget === "python" && isLinux() && arch === "x64") {
    return "python-x86_64-linux";
  }
  return;
};

const main = async (): Promise<void> => {
  const program = new Command();
  const qp = await loadQp();
  program.version(`qpace_core = ${qp.getVersion()}`);
  program
    .command("auth")
    .argument("<token>")
    .action((token) => {
      console.log("auth", token);
    });
  program.command("init").action(() => console.log("init"));
  program
    .command("build")
    .option("--emit", `Emits compiled files`)
    .addOption(
      new Option("--target <target>", `Target platform`).choices(BUILD_TARGETS),
    )
    .option("--emit-dir <path>", `Output directory`)
    .option("--dir <path>", `Output directory`)
    .option("--config", `Config file`)
    .option("--install-wheel", `Installs generated python wheel artifact`)
    .option("--cwd <path>", `Project root directory`)
    .action(async (options) => {
      const rootDir =
        options.cwd != null ? resolve(CWD_PATH, options.cwd) : CWD_PATH;
      const config = await loadOrCreateConfig(rootDir);
      config.emitDir = options.emitDir ?? config.emitDir;
      config.emit = options.emit ?? config.emit;
      config.python ??= {};
      config.python.installWheel =
        options.installWheel ?? config.python.installWheel;
      config.buildDir = options.dir ?? config.buildDir;

      const target = tryMapBuildTarget(options.target);
      if (options.target != null && target == null) {
        throw new Error(`Unsupported target: ${options.target}`);
      }

      if (target == null && !config.emit) {
        throw new Error(`--target must be specified or --emit enabled`);
      }

      const client = new Client({});
      const driver = new RemoteDriver({
        client: client.compilerClient,
        config,
        rootDir,
      });
      await driver.build({ target });
    });
  program
    .command("check")
    .option("--config", `Config file`)
    .option("--cwd <path>", `Project root directory`)
    .action(async (options) => {
      const rootDir =
        options.cwd != null ? resolve(CWD_PATH, options.cwd) : CWD_PATH;
      const config = await loadOrCreateConfig(rootDir);
      const client = new Client({});
      const driver = new RemoteDriver({
        client: client.compilerClient,
        config,
        rootDir,
      });
      await driver.check();
    });
  program.parse();
};

main();
