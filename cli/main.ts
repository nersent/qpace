import { resolve } from "path";

import chalk from "chalk";
import { Command } from "commander";

import { Driver, Program } from "../compiler/driver";

import { loadQp } from "./qp";
import { OsFileSystem, RemoteDriver } from "./remote_driver";

import { exists, readJson, writeJson } from "~/base/node/fs";
import {
  Config,
  getDefaultConfig,
  mergeConfigs,
  QPC_CONFIG_FILENAME,
} from "~/compiler/config";
import { Client } from "~/lib/client";

// import { ApiClient } from "./api_client";
// import { CliService } from "./cli_service";
// import { CompilerCli } from "./compiler_cli";
// import { RemoteDriver } from "./remote_driver";

// const apiClient = new ApiClient({});
// const cliService = new CliService({
//   compilerModule: new CompilerCli(
//     (program) => new RemoteDriver(program, { apiClient }),
//   ),
// });

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

const createProgram = async (
  rootDir: string,
  config: Config,
): Promise<Program> => {
  return {
    getRootDir: () => rootDir,
    getConfig: () => config,
    getFs: () => new OsFileSystem(rootDir),
  };
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
    .option("--target <target>", `Target platform`)
    .option("--emit-dir <path>", `Output directory`)
    .option("--dir <path>", `Output directory`)
    .option("--config", `Config file`)
    .option("--install-wheel", `Installs generated python wheel artifact`)
    .option("--cwd <path>", `Project root directory`)
    .action(async (options) => {
      const cwd =
        options.cwd != null ? resolve(CWD_PATH, options.cwd) : CWD_PATH;
      const config = await loadOrCreateConfig(cwd);
      config.emitDir = options.emitDir ?? config.emitDir;
      config.emit = options.emit ?? config.emit;
      config.python ??= {};
      config.python.installWheel =
        options.installWheel ?? config.python.installWheel;
      config.buildDir = options.dir ?? config.buildDir;

      const program = await createProgram(cwd, config);
      const client = new Client({});
      const driver: Driver = new RemoteDriver(program, client.compilerClient);
      await driver.build();
    });
  program
    .command("check")
    .option("--config", `Config file`)
    .option("--cwd <path>", `Project root directory`)
    .action(async (options) => {
      const cwd =
        options.cwd != null ? resolve(CWD_PATH, options.cwd) : CWD_PATH;
      const config = await loadOrCreateConfig(cwd);
      const program = await createProgram(cwd, config);
      const client = new Client({});
      const driver: Driver = new RemoteDriver(program, client.compilerClient);
      await driver.check();
    });
  program.parse();
};

main();
