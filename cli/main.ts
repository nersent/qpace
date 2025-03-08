import { Command } from "commander";
import { Driver, Program } from "../compiler/driver";
import {
  Config,
  getDefaultConfig,
  mergeConfigs,
  QPC_CONFIG_FILENAME,
} from "~/compiler/config";
import { resolve } from "path";
import { OsFileSystem, RemoteDriver } from "./remote_driver";
import { exists, readJson } from "~/base/node/fs";
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

const loadProgram = async (rootDir: string): Promise<Program> => {
  const config = await loadConfig(rootDir);
  return {
    getRootDir: () => rootDir,
    getConfig: () => config,
    getFs: () => new OsFileSystem(rootDir),
  };
};

const main = async (): Promise<void> => {
  const rootDir = `C:\\projects\\nersent\\qpace\\xd`;
  const program = await loadProgram(rootDir);
  const client = new Client({});
  const driver: Driver = new RemoteDriver(program, client.compilerClient);
  // console.log(program.getConfig());
  console.log('building')
  await driver.build();
  // const qp = await import("../core/pkg/qpace_core.js");
  // const program = new Command();
  // program.version(`qpace_core = ${qp.getVersion()}`);
  // program.addCommand(new Command('build').action(() => console.log('build')));
  // cliService.build().forEach((r) => program.addCommand(r));
  // program.parse();
};

main();
