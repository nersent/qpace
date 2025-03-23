import { existsSync, FSWatcher, Stats } from "fs";
import { mkdir } from "fs/promises";
import { homedir } from "os";
import { resolve } from "path";

import { input } from "@inquirer/prompts";
import axios from "axios";
import chalk from "chalk";
import { watch } from "chokidar";
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
} from "~/lib/internal";

export interface UserConfig {
  apiKey?: string;
  telemetry?: boolean;
}

export const tryLoadUserConfig = async (): Promise<UserConfig | undefined> => {
  const dir = resolve(homedir(), ".qpace");
  const configPath = resolve(dir, "config.json");
  if (await exists(configPath)) {
    return await readJson(configPath);
  }
  return;
};

export const saveConfig = async (config: UserConfig): Promise<void> => {
  const dir = resolve(homedir(), ".qpace");
  if (!existsSync(dir)) await mkdir(dir, { recursive: true });
  const configPath = resolve(dir, "config.json");
  await writeJson(configPath, config, true);
};

const QPACE_BG_PREFIX = `${chalk.bgGreen.black.bold("qpace")}: `;

class CliError extends Error {
  constructor(message: string) {
    super(message);
    this.name = "CliError";
  }
}

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

const ORDER_CHOICES = ["asc", "desc"] as const;
type Order = typeof ORDER_CHOICES[number];

class Cli {
  private _client?: qp.Client;
  public userConfig: UserConfig = {};
  private apiKey: string | undefined;

  public async init(): Promise<void> {
    this.userConfig = await this.loadConfig();
    this.apiKey = process.env[ENV_API_KEY];
    this.apiKey ??= this.userConfig.apiKey;
  }

  public get telemetry(): boolean {
    return this.userConfig.telemetry ?? true;
  }

  public setApiKey(apiKey: string, userConfig?: boolean): void {
    this.apiKey = apiKey;
    if (userConfig) {
      this.userConfig.apiKey = apiKey;
    }
  }

  public async loadConfig(): Promise<UserConfig> {
    const userConfig = await tryLoadUserConfig();
    if (userConfig == null) {
      this.userConfig = {};
      await this.saveConfig();
    } else {
      this.userConfig = userConfig;
    }
    return this.userConfig;
  }

  public async saveConfig(): Promise<void> {
    await saveConfig(this.userConfig);
  }

  public tryGetClient(): qp.Client | undefined {
    if (this._client != null) return this._client;
    const apiKey = this.apiKey;
    if (apiKey != null) {
      const apiBase = process.env[ENV_REST_ENDPOINT];
      const grpcApiBase = process.env[ENV_GRPC_ENDPOINT];
      this._client = new qp.Client({
        apiKey,
        apiBase,
        grpcApiBase,
      });
    }
    return this._client;
  }

  public getClient(): qp.Client {
    const client = this.tryGetClient();
    if (client == null) {
      throw new CliError("No API key");
    }
    return client;
  }

  public async maybePromptApiKey(
    apiKey?: string,
    force?: boolean,
  ): Promise<void> {
    apiKey ??= this.userConfig.apiKey;
    if (apiKey == null) {
      force = true;
    }

    let inputApiKey: string | undefined;
    if (apiKey == null || force) {
      console.log(`${QPACE_BG_PREFIX}Logging into qpace.dev`);
      console.log(
        `${QPACE_BG_PREFIX}You can find your API key in your browser here: ${chalk.cyanBright(
          `https://qpace.dev/auth`,
        )}`,
      );
      console.log(
        `${QPACE_BG_PREFIX}Paste an API key here and press enter, or press ctrl+c to quit`,
      );
      inputApiKey = await input({
        message: "Enter your API key",
        validate: (x) => (x.trim().length > 0 ? true : "API key is required"),
      });
      this.setApiKey(inputApiKey, true);
    } else {
      this.setApiKey(apiKey);
    }
    this._client = undefined;

    try {
      const client = this.getClient();
      const team = await client.getMe();
      if (inputApiKey != null) {
        console.log(
          `${QPACE_BG_PREFIX}Logged in as team ${chalk.yellowBright(
            team.team.name,
          )}. Use ${chalk.white.bold(
            `\`qpace login --force\``,
          )} to force relogin.`,
        );
      }
      if (inputApiKey != null) {
        await this.saveConfig();
      }
    } catch (e) {
      if (axios.isAxiosError(e) && e.response?.status === 401) {
        throw new CliError("Invalid API key. Try authenticating again.");
      }
      throw e;
    }
  }

  public async handleTelemetry(enabled?: boolean): Promise<void> {
    if (enabled != null) {
      this.userConfig.telemetry = enabled;
      await this.saveConfig();
    }
    console.log(
      `${QPACE_BG_PREFIX}Telemetry is ${chalk.white.bold(
        this.telemetry ? "enabled" : "disabled",
      )}\n`,
    );
    if (this.telemetry) {
      console.log(
        `qpace telemetry is completely anonymous and optional.\nThank you for for participating!`,
      );
    } else {
      console.log(
        `You have disabled anonymous telemetry.\nNo data will be collected from your machine.`,
      );
    }
    console.log(
      `\nLearn more at: ${chalk.cyan(`https://qpace.dev/telemetry`)}`,
    );
  }
}

const handleException = (e: Error): void => {
  if (e instanceof CliError) {
    console.log(chalk.redBright.bold(e.message));
    process.exit(1);
  }
  throw e;
};

// eslint-disable-next-line @typescript-eslint/explicit-function-return-type
const watchPaths = async (
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

const main = async (): Promise<void> => {
  process.on("unhandledRejection", (reason, promise) => {
    handleException(reason as Error);
  });

  const cli = new Cli();
  await cli.init();
  const program = new Command();
  program.version(`qpace_core = ${qp.getVersion()}`);
  program
    .command("login")
    .argument("[api key]")
    .action(async (apiKey: string | undefined) => {
      await cli.maybePromptApiKey(apiKey, true);
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
    .option("--watch, -w", `Rebuild on file changes`, false)
    .action(
      async (opts: {
        cwd?: string;
        emit?: boolean;
        target?: BuildTarget;
        emitDir?: string;
        dir?: string;
        installWheel?: boolean;
        verbose?: boolean;
        watch?: boolean;
      }) => {
        await cli.maybePromptApiKey();
        const client = cli.getClient();
        const rootDir =
          opts.cwd != null ? resolve(CWD_PATH, opts.cwd) : CWD_PATH;
        const qpcConfig = await loadOrCreateConfig(rootDir);
        qpcConfig.emitDir = opts.emitDir ?? qpcConfig.emitDir;
        qpcConfig.emit = opts.emit ?? qpcConfig.emit;
        qpcConfig.python ??= {};
        qpcConfig.python.installWheel =
          opts.installWheel ?? qpcConfig.python.installWheel;
        qpcConfig.buildDir = opts.dir ?? qpcConfig.buildDir;

        const target = tryMapBuildTarget(opts.target as any);
        if (opts.target != null && target == null) {
          throw new CliError(`Unsupported target: ${opts.target}`);
        }
        if (target == null && !qpcConfig.emit) {
          throw new CliError(`--target must be specified or --emit enabled`);
        }

        const driver = new RemoteDriver({
          client: client.compilerClient,
          qpcConfig,
          rootDir,
          verbose: opts.verbose,
        });
        await driver.build({ target });
        if (opts.watch) {
          watchPaths(
            { included: await driver.collectSrcFiles() },
            async (path) => {
              console.log(chalk.blackBright(`${path}`));
              await driver.build({ target });
            },
          );
        }
      },
    );
  program
    .command("check")
    .option("--config", `Config file`)
    .option("--cwd <path>", `Project root directory`)
    .option("--watch, -w", `Recheck on file changes`, false)
    .action(async (opts: { cwd?: string; watch?: boolean }) => {
      await cli.maybePromptApiKey();
      const client = cli.getClient();
      const rootDir = opts.cwd != null ? resolve(CWD_PATH, opts.cwd) : CWD_PATH;
      const qpcConfig = await loadOrCreateConfig(rootDir);
      const driver = new RemoteDriver({
        client: client.compilerClient,
        qpcConfig,
        rootDir,
      });
      await driver.check();
      if (opts.watch) {
        watchPaths(
          { included: await driver.collectSrcFiles() },
          async (path) => {
            console.log(chalk.blackBright(`${path}`));
            await driver.check();
          },
        );
      }
    });
  program
    .command("symbol")
    .alias("sym")
    .option("--full", `Full info`)
    .option("--limit <limit>", `Limit`, "10")
    .option(
      "--timeframe, -t <timeframe>",
      `Returned symbols have OHLCV available with provided timeframe`,
    )
    .argument("[patterns...]")
    .action(
      async (
        patterns: string[],
        opts: {
          format: DataFormat;
          limit: string;
          full?: boolean;
          timeframe?: string;
        },
      ) => {
        const limit = parseInt(opts.limit);
        await cli.maybePromptApiKey();
        const client = cli.getClient();
        const syms: qp.Sym[] = [];
        const timeframe: qp.Timeframe | undefined = opts.timeframe
          ? qp.Timeframe.fromString(opts.timeframe)
          : undefined;
        if (patterns.length === 0) {
          syms.push(...(await client.syms({ limit, timeframe })));
        } else {
          const _syms = await Promise.all(
            patterns.map(async (r) => [
              ...(await client.syms({ id: r, tickerId: r, limit, timeframe })),
            ]),
          ).then((r) => r.flat());
          syms.push(..._syms);
        }
        if (syms.length === 0) {
          throw new CliError("No symbol found");
        }
        const items = syms.map((sym) => ({
          ...sym.toJSON(),
        }));
        console.table(
          items,
          opts.full
            ? undefined
            : ["tickerId", "baseCurrency", "currency", "prefix"],
        );
      },
    );
  program
    .command("ohlcv")
    .alias("price")
    .option("--timeframe, -t <timeframe>", `Timeframe`, "1D")
    .option("--limit <limit>", `Bars limit`, "30")
    .addOption(
      new Option("--order, -o <order>", `Order`)
        .choices(ORDER_CHOICES)
        .default("desc"),
    )
    .argument("<patterns...>")
    .action(
      async (
        patterns: string[],
        opts: { timeframe?: string; limit?: string; order?: Order },
      ) => {
        await cli.maybePromptApiKey();
        const timeframe =
          opts.timeframe != null
            ? qp.Timeframe.fromString(opts.timeframe)
            : undefined;
        const client = cli.getClient();
        const syms: qp.Sym[] = [];
        let items: any[] = [];
        for (const pattern of patterns) {
          const sym = await client.sym({ tickerId: pattern });
          if (syms.find((r) => r.id === sym.id)) continue;
          const bars = await client.bars({ sym, timeframe });
          const _items = bars.map((r) => ({
            tickerId: sym.tickerId,
            ...r.toJSON(),
          }));
          items.push(..._items);
        }
        if (items.length === 0) {
          throw new CliError("No bars found");
        }
        if (opts.order === "asc") {
          items.sort((a, b) => a.openTime - b.openTime);
        } else {
          items.sort((a, b) => b.openTime - a.openTime);
        }
        if (opts.limit != null) {
          items = items.slice(0, parseInt(opts.limit));
        }
        console.table(items);
      },
    );
  program.addCommand(
    new Command("telemetry")
      .action(() => {
        cli.handleTelemetry();
      })
      .addCommand(
        new Command("enable").action(async () => {
          await cli.handleTelemetry(true);
        }),
      )
      .addCommand(
        new Command("disable").action(async () => {
          await cli.handleTelemetry(false);
        }),
      ),
  );
  program.parse();
};

main();
