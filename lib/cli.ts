import { resolve } from "path";

import chalk from "chalk";
import { Command, Option } from "commander";

import { RemoteDriver } from "./remote_driver";

import { exists, readJson, writeJson } from "~/base/node/fs";
import { isLinux, isMacOs, isWindows } from "~/base/node/os";
import * as qp from "~/lib";
import {
  Config,
  getDefaultConfig,
  getInitConfig,
  mergeConfigs,
  QPC_CONFIG_FILENAME,
  Target,
  TARGETS,
} from "~/lib/compiler";
import {
  ENV_API_KEY,
  ENV_GRPC_ENDPOINT,
  ENV_REST_ENDPOINT,
  validateSymQuery,
} from "~/lib/internal";

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
    const config = getInitConfig();
    console.log(
      chalk.yellowBright(
        `Warning: Config file not found at ${configPath}. Creating a default config file.`,
      ),
    );
    await writeJson(configPath, config, true);
    return config;
  }
  return await loadConfig(dir);
};

const BUILD_TARGETS = [...TARGETS, "python"] as const;
type BuildTarget = typeof BUILD_TARGETS[number];

const tryMapBuildTarget = (buildTarget: BuildTarget): Target | undefined => {
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

const DATA_FORMAT_CHOICES = ["json", "csv", "table"] as const;
type DataFormat = typeof DATA_FORMAT_CHOICES[number];

const getClient = async (): Promise<qp.Client> => {
  const apiKey = process.env[ENV_API_KEY];
  if (apiKey == null) {
    throw new Error(`API key not found in environment variable ${ENV_API_KEY}`);
  }
  const restEndpoint = process.env[ENV_REST_ENDPOINT];
  const grpcEndpoint = process.env[ENV_GRPC_ENDPOINT];
  return new qp.Client({
    apiKey,
    restEndpoint,
    grpcEndpoint,
  });
};

const main = async (): Promise<void> => {
  const program = new Command();
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
    .addOption(
      new Option("--target <target>", `Target platform`).choices(BUILD_TARGETS),
    )
    .option("--config <path>", `Config file`)
    .option("--emit", `Emits compiled files. Default: "config.emit"`)
    .option("--emit-dir <path>", `Output directory. Default: "config.emitDir"`)
    .option("--dir <path>", `Output directory. Default: "config.buildDir"`)
    .option(
      "--install-wheel",
      `Installs generated python wheel artifact. Default: "config.python.installWheel"`,
    )
    .option("--cwd <path>", `Project root directory`)
    .option("--verbose, -v", `Verbose output`)
    .action(
      async (opts: {
        cwd?: string;
        emit?: boolean;
        target?: BuildTarget;
        emitDir?: string;
        dir?: string;
        installWheel?: boolean;
        verbose?: boolean;
      }) => {
        const client = await getClient();

        const rootDir =
          opts.cwd != null ? resolve(CWD_PATH, opts.cwd) : CWD_PATH;
        const config = await loadOrCreateConfig(rootDir);
        config.emitDir = opts.emitDir ?? config.emitDir;
        config.emit = opts.emit ?? config.emit;
        config.python ??= {};
        config.python.installWheel =
          opts.installWheel ?? config.python.installWheel;
        config.buildDir = opts.dir ?? config.buildDir;

        const target = tryMapBuildTarget(opts.target as any);
        if (opts.target != null && target == null) {
          throw new Error(`Unsupported target: ${opts.target}`);
        }
        if (target == null && !config.emit) {
          throw new Error(`--target must be specified or --emit enabled`);
        }
        const driver = new RemoteDriver({
          client: client.compilerClient,
          config,
          rootDir,
          verbose: opts.verbose,
        });
        await driver.build({ target });
      },
    );
  program
    .command("check")
    .option("--config", `Config file`)
    .option("--cwd <path>", `Project root directory`)
    .action(async (options) => {
      const client = await getClient();

      const rootDir =
        options.cwd != null ? resolve(CWD_PATH, options.cwd) : CWD_PATH;
      const config = await loadOrCreateConfig(rootDir);
      const driver = new RemoteDriver({
        client: client.compilerClient,
        config,
        rootDir,
      });
      await driver.check();
    });
  program
    .command("symbol")
    .alias("sym")
    .option("--list")
    .option("--full", `Show full symbol info`)
    .option("--id <id>", `Symbol id pattern`)
    .option("--ticker <ticker>", `Symbol ticker id pattern`)
    .action(
      async (opts: {
        format: DataFormat;
        full?: boolean;
        id?: string;
        ticker?: string;
        list?: boolean;
      }) => {
        const client = await getClient();
        const syms: qp.Sym[] = [];
        const symQuery: qp.SymQuery = { id: opts.id, tickerId: opts.ticker };
        if (opts.list) {
          syms.push(...(await client.syms(symQuery)));
        } else {
          validateSymQuery(symQuery);
          syms.push(await client.sym(symQuery));
        }
        if (syms.length === 0) {
          console.log(chalk.yellowBright("No symbol found"));
          return;
        }
        const items = syms.map((sym) => ({
          ...sym.toJSON(),
        }));
        console.table(
          items,
          opts.full
            ? undefined
            : ["tickerId", "baseCurrency", "currency", "minTick", "minQty"],
        );
      },
    );
  program
    .command("ohlcv")
    .alias("price")
    .option("--id <id>", `Symbol id pattern`)
    .option("--ticker <ticker>", `Symbol ticker id pattern`)
    .option("--timeframe <timeframe>", `Timeframe`, "1D")
    .option("--time", `Show time`, true)
    .option("--limit <limit>", `Limit`)
    .option("--asc", `Ascending time order`, true)
    .option("--desc", `Descending order order`, false)
    .action(
      async (opts: {
        id?: string;
        ticker?: string;
        timeframe?: string;
        time?: boolean;
        limit?: string;
        asc?: boolean;
        desc?: boolean;
      }) => {
        const timeframe =
          opts.timeframe != null
            ? qp.Timeframe.fromString(opts.timeframe)
            : undefined;
        let order: "asc" | "desc" = "asc";
        if (opts.asc === opts.desc) {
          throw new Error("Either --asc or --desc must be specified");
        }
        if (opts.desc) order = "desc";
        if (opts.asc) order = "asc";
        const client = await getClient();
        const symQuery: qp.SymQuery = { id: opts.id, tickerId: opts.ticker };
        validateSymQuery(symQuery);
        const sym = await client.sym(symQuery);
        let bars = await client.bars({ sym, timeframe });
        if (bars.length === 0) {
          console.log(chalk.yellowBright("No bars found"));
          return;
        }
        if (order === "desc") {
          bars.reverse();
        }
        if (opts.limit != null) {
          bars = bars.slice(0, parseInt(opts.limit));
        }
        console.table(bars.map((r) => r.toJSON()));
      },
    );
  program.parse();
};

main();
