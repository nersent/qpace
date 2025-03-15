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
import { validateSymQuery } from "~/lib/internal";

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

const DATA_FORMAT_CHOICES = ["json", "csv", "table"] as const;
type DataFormat = typeof DATA_FORMAT_CHOICES[number];

const getClient = async (): Promise<qp.Client> => {
  return new qp.Client({
    apiKey: "sk_b6fc26f0-d900-4fb0-8fc1-d83abdf1837f",
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
      const client = await getClient();

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

      if (target != null) {
        console.log(chalk.black(`Building ${target}`));
      }

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
