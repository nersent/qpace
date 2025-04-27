import { existsSync, FSWatcher, Stats } from "fs";
import { mkdir, readdir, readFile, writeFile } from "fs/promises";
import os, { homedir } from "os";
import { basename, dirname, resolve } from "path";

import { input, confirm, select, checkbox } from "@inquirer/prompts";
import axios, { AxiosResponse } from "axios";
import chalk from "chalk";
import { watch } from "chokidar";
import { Command, Option } from "commander";

import { tryParseInt } from "../base/node/number";
import { deepMerge } from "../base/node/object";
import { withoutExt } from "../base/node/path";

import { EXAMPLES } from "./examples";
import { findPythonPath, RemoteDriver } from "./remote_driver";

import { exec } from "~/base/node/exec";
import { exists, readJson, watchPaths, writeJson } from "~/base/node/fs";
import { isLinux, isMacOs, isWindows, which } from "~/base/node/os";
import * as qp from "~/lib";
import { Client } from "~/lib";
import {
  Config as QpcConfig,
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
  VerifyApiKeyRequest,
  VerifyApiKeyResponse,
} from "~/lib/internal";

const CWD_PATH = process.env["BAZED_WORKSPACE_ROOT"] ?? process.cwd();
const QPACE_BG_PREFIX = `${chalk.hex("#000").bgHex("#7fee64").bold("qPACE")} `;
// const QPACE_BG_PREFIX = `${chalk.bgGreen.black.bold("qPACE")} `;
const EXAMPLES_DIR = resolve(__dirname, "examples");

export class CliError extends Error {
  constructor(message: string) {
    super(message);
    this.name = "CliError";
  }
}

const withHref = (href: string): string => {
  return `${chalk.cyanBright(href)}`;
};

interface UserConfig {
  apiKey?: string;
  telemetry?: boolean;
}

const readUserConfig = async (dir?: string): Promise<UserConfig> => {
  dir ??= resolve(homedir(), ".qpace");
  const configPath = resolve(dir, "config.json");
  if (await exists(configPath)) {
    return await readJson(configPath);
  }
  return { telemetry: true };
};

const writeUserConfig = async (
  config: UserConfig,
  dir?: string,
): Promise<void> => {
  dir ??= resolve(homedir(), ".qpace");
  if (!existsSync(dir)) await mkdir(dir, { recursive: true });
  const configPath = resolve(dir, "config.json");
  await writeJson(configPath, config, true);
};

const updateUserConfig = async (
  modifiedConfig: UserConfig,
  dir?: string,
): Promise<UserConfig> => {
  let config = await readUserConfig(dir);
  config = deepMerge(config, modifiedConfig);
  await writeUserConfig(config, dir);
  return config;
};

// eslint-disable-next-line @typescript-eslint/explicit-function-return-type
const initQpc = async ({
  dir,
  configPath,
  prompt,
  ignoreExisting,
}: {
  dir?: string;
  configPath?: string;
  prompt?: boolean;
  ignoreExisting?: boolean;
}) => {
  dir = resolve(process.cwd(), dir ?? "");
  if (configPath != null) configPath = resolve(dir, configPath);
  if (dir != null && configPath != null && dir !== dirname(configPath)) {
    throw new CliError(
      `Config file ${configPath} must be in the same directory as cwd ${dir}`,
    );
  }
  if (configPath == null) {
    configPath = resolve(dir, QPC_CONFIG_FILENAME);
  }
  let config: QpcConfig | undefined;
  if (await exists(configPath)) {
    config = await readJson(configPath);
  }
  if (config == null && !prompt) {
    throw new CliError(
      `Project not found at ${chalk.yellowBright(
        configPath,
      )}\nTo setup a new project, use ${chalk.white.bold(`"qpc init"`)}`,
    );
  } else if (prompt) {
    await mkdir(dir, { recursive: true });
    const creatingNew = config == null;
    if (config != null) {
      if (ignoreExisting) {
        return { config, configPath };
      }
      console.log(`${QPACE_BG_PREFIX}Project already exists at ${configPath}`);
      const overwrite = await confirm({
        message: "Do you want to overwrite existing project?",
        default: true,
      });
      if (!overwrite) {
        return { config, configPath };
      }
    } else {
      console.log(`${QPACE_BG_PREFIX}Creating new project.`);
      config = getInitConfig();
    }
    const usePython = await confirm({
      message: "Do you want to use Python?",
      default: true,
    });
    if (usePython) {
      config.python = {
        ...config.python,
        bindings: true,
      };
      const pythonPackageName = await input({
        message: "Enter python package name",
        default: config.python?.package ?? "qpace_artifact",
        required: false,
        validate: (x) => {
          if (x.trim().length === 0) {
            return "Python package name is required";
          }
          if (!/^[a-zA-Z0-9_.-]+$/.test(x)) {
            return "Invalid python package name";
          }
          return true;
        },
      });
      config.python.package = pythonPackageName;
    }
    const overwriteConfig = await select({
      message: `Do you want to overwrite config?`,
      choices: [
        {
          name: "no",
          value: false,
        },
        {
          name: "yes",
          value: true,
        },
      ],
    });
    if (overwriteConfig) {
      if (usePython) {
        config.python ??= {};
        const installPythonWheel = await select({
          message: `Should python wheel be automatically installed?`,
          default: config.python?.installWheel,
          choices: [
            {
              name: "yes",
              value: true,
            },
            {
              name: "no",
              value: false,
            },
          ],
        });
        config.python.installWheel = installPythonWheel;

        if (installPythonWheel) {
          const testPythonWheel = await select({
            message: `Should installed python wheel be automatically tested?`,
            default: config.python?.testWheel,
            choices: [
              {
                name: "yes",
                value: true,
              },
              {
                name: "no",
                value: false,
              },
            ],
          });
          config.python.testWheel = testPythonWheel;
        }

        const buildDir = await input({
          message: "Enter build directory",
          default: config.buildDir ?? "build",
          required: false,
          validate: (x) => {
            if (x.trim().length === 0) {
              return "Build directory is required";
            }
            return true;
          },
        });
        config.buildDir = buildDir;
      }
    }
    const useExample = await select({
      message: `Do you want to setup an example?`,
      default: creatingNew,
      choices: [
        {
          name: "yes",
          value: true,
        },
        {
          name: "no",
          value: false,
        },
      ],
    });
    if (useExample) {
      const exampleIds = await checkbox({
        message: "Select examples",
        choices: EXAMPLES.map((r) => ({
          value: r.id,
          name: r.name,
        })),
      });
      for (const exampleId of exampleIds) {
        const exampleDir = resolve(EXAMPLES_DIR, exampleId);
        const srcFiles = await readdir(exampleDir).then((files) =>
          files.map((r) => resolve(exampleDir, r)),
        );
        for (const srcFile of srcFiles) {
          const srcPath = resolve(exampleDir, srcFile);
          let destPath = resolve(dir, basename(srcFile));
          if (existsSync(destPath)) {
            const overwrite = await confirm({
              message: `File ${destPath} already exists. Do you want to overwrite it?`,
              default: true,
            });
            if (!overwrite) {
              destPath = resolve(dir, `${Date.now()}_${srcFile}`);
            }
          }
          let content = await readFile(srcPath, "utf8");
          content = content.replaceAll(
            "__QPC_PYTHON_PACKAGE__",
            config.python?.package ?? "",
          );
          await writeFile(destPath, content, "utf8");
        }
      }
    }
    await writeJson(configPath, config, true);
    if (creatingNew) {
      console.log(`${QPACE_BG_PREFIX}Project created at ${configPath}`);
      console.log(
        chalk.yellowBright(
          `\nTo check for errors, run:\n${chalk.blue.bold(`qpc check`)}`,
        ),
      );
      console.log(
        chalk.yellowBright(
          `\nTo build the project, run:\n${chalk.blue.bold(
            `qpc build --target python`,
          )}`,
        ),
      );
      if (usePython) {
        console.log(
          chalk.yellowBright(
            `\nTo use pine from python, you need to import using:\n${chalk.blue.bold(
              `import ${config.python?.package ?? ""} as pine`,
            )}`,
          ),
        );
      }
      console.log(
        `\nSee more at ${withHref(`https://qpace.dev/docs/qpc-project`)}`,
      );
    } else {
      console.log(`${QPACE_BG_PREFIX}Project modified at ${configPath}`);
    }
  }
  config ??= {};
  return { config, configPath };
};

const validateApiKey = async ({
  apiKey,
  verbose,
  client,
}: {
  apiKey: string;
  verbose?: boolean;
  client?: Client;
}): Promise<VerifyApiKeyResponse | undefined> => {
  client ??= new Client({ apiKey });
  await updateClientInfo(client);
  client["init"]();
  try {
    const { data } = await client["http"].get<
      VerifyApiKeyResponse,
      AxiosResponse<VerifyApiKeyResponse>,
      VerifyApiKeyRequest
    >(`/api_keys/verify/${client["config"]["apiKey"]}`);
    if (verbose) {
      console.log(
        `${QPACE_BG_PREFIX}Logged in as ${chalk.yellowBright(data.user?.name)}`,
      );
    }
    return data;
  } catch (e) {
    if (axios.isAxiosError(e) && e.response?.status === 403) {
      if (verbose) {
        throw new CliError("Invalid API key. Try authenticating again.");
      }
      return;
    }
    throw e;
  }
};

const promptApiKey = async (): Promise<string> => {
  console.log(`${QPACE_BG_PREFIX}Logging into qpace.dev`);
  console.log(
    `${QPACE_BG_PREFIX}You can find your API key in your browser here: ${chalk.cyanBright(
      `https://qpace.dev/auth`,
    )}`,
  );
  console.log(
    `${QPACE_BG_PREFIX}Paste an API key here and press enter, or press ctrl+c to quit`,
  );
  const apiKey = await input({
    message: "Enter your API key",
    transformer: (apiKey) => apiKey?.trim(),
    validate: async (apiKey) => {
      if (!apiKey.length) {
        return "API key is required";
      }
      if ((await validateApiKey({ apiKey })) != null) {
        return true;
      }
      return "API key is invalid";
    },
  });
  await validateApiKey({ apiKey, verbose: true });
  await updateUserConfig({ apiKey });
  return apiKey;
};

const loadApiKey = async (): Promise<string> => {
  const userConfig = await readUserConfig();
  const apiKey = process.env[ENV_API_KEY] ?? userConfig.apiKey;
  if (apiKey == null) {
    return await promptApiKey();
  }
  return apiKey;
};

const loadClient = async (): Promise<Client> => {
  const apiKey = await loadApiKey();
  const client = new Client({
    apiKey,
    apiBase: process.env[ENV_REST_ENDPOINT],
    grpcApiBase: process.env[ENV_GRPC_ENDPOINT],
  });
  return client;
};

const promptTelemetry = async (enabled?: boolean): Promise<void> => {
  if (enabled != null) {
    await updateUserConfig({ telemetry: enabled });
  }
  const userConfig = await readUserConfig();
  console.log(
    `${QPACE_BG_PREFIX}Telemetry is ${chalk.white.bold(
      userConfig?.telemetry ? "enabled" : "disabled",
    )}\n`,
  );
  if (userConfig?.telemetry) {
    console.log(
      `Telemetry is completely anonymous and optional.\nThank you for for participating!`,
    );
  } else {
    console.log(
      `You have disabled anonymous telemetry.\nNo data will be collected from your machine.`,
    );
  }
  console.log(`\nLearn more at: ${chalk.cyan(`https://qpace.dev/telemetry`)}`);
};

const updateClientInfo = async (
  client: qp.Client,
  userConfig?: UserConfig,
): Promise<void> => {
  userConfig ??= await readUserConfig();
  if (!userConfig.telemetry) return;
  client["clientInfo"] ??= {};
  client["clientInfo"]["platform"] = os.platform()?.trim();
  client["clientInfo"]["arch"] = os.arch()?.trim();
  client["clientInfo"]["cpus"] = os.cpus().length;
  client["clientInfo"]["osRelease"] = os.release()?.trim();
  client["clientInfo"]["memory"] = os.totalmem();
  client["clientInfo"]["cpu"] = os.cpus()[0].model?.trim();
  await Promise.all(
    [
      ["npm", "npm --version"],
      ["pnpm", "pnpm --version"],
      ["yarn", "yarn --version"],
      ["python", "python --version"],
      ["python3", "python3 --version"],
      ["pip", "pip --version"],
      ["pip3", "pip3 --version"],
      ["rustc", "rustc --version"],
      ["cargo", "cargo --version"],
    ].map(async ([key, command]) => {
      const { stdout } = await exec({
        command,
      });
      client["clientInfo"]![key] = stdout;
    }),
  );
  client["init"]();
};

const handleException = (e: Error): void => {
  if (e instanceof Error && e.name === "ExitPromptError") {
    return;
  }
  if (e instanceof CliError) {
    console.log(chalk.redBright(e.message));
    process.exit(1);
  }
  throw e;
};

const DATA_FORMAT_CHOICES = ["table", "table-mini", "json", "csv"] as const;
type DataFormat = typeof DATA_FORMAT_CHOICES[number];

const ORDER_CHOICES = ["asc", "desc"] as const;
type Order = typeof ORDER_CHOICES[number];

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

const main = async (): Promise<void> => {
  process.on("unhandledRejection", (reason, promise) => {
    handleException(reason as Error);
  });
  process.on("uncaughtException", (reason) => {
    handleException(reason as Error);
  });

  const program = new Command();
  program.option("-v, --version", "Show version").action(async () => {
    console.table({
      qpace: qp.getVersion(),
      qpaceCore: qp.getCoreVersion(),
    });
  });
  program
    .command("login")
    .argument("[api key]")
    .action(async (apiKey: string | undefined) => {
      if (apiKey != null) {
        await validateApiKey({ apiKey, verbose: true });
        await updateUserConfig({ apiKey });
        return;
      }
      await promptApiKey();
    });
  program
    .command("init")
    .option("--cwd <path>", `Project root directory`)
    .action(async (opts: { cwd?: string }) => {
      await initQpc({
        dir: resolve(CWD_PATH, opts.cwd ?? process.cwd()),
        prompt: true,
      });
    });
  program
    .command("symbol")
    .alias("sym")
    .option("--limit <limit>", `Limit`, "50")
    .option(
      "--timeframe, -t <timeframe>",
      `If provided, symbols are expected to have OHLCV available at that timeframe`,
    )
    .addOption(
      new Option("--format, -f <format>", `Data format`)
        .choices(DATA_FORMAT_CHOICES)
        .default("table-mini"),
    )
    .argument("[patterns...]")
    .action(
      async (
        patterns: string[],
        opts: {
          limit: string;
          timeframe?: string;
          format: DataFormat;
        },
      ) => {
        const client = await loadClient();
        const limit = parseInt(opts.limit);
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
        if (opts.format === "table-mini") {
          console.table(items, [
            "ticker_id",
            "base_currency",
            "currency",
            "prefix",
          ]);
        } else if (opts.format === "table") {
          console.table(items);
        } else if (opts.format === "json") {
          console.log(items);
        } else {
          throw new CliError(`Unsupported data format: ${opts.format}`);
        }
      },
    );
  program
    .command("ohlcv")
    .alias("price")
    .option("--timeframe, -t <timeframe>", `Timeframe`, "1D")
    .option("--limit <limit>", `Bars limit`, "30")
    .option("--offset <offset>", "Bar offset", "0")
    .addOption(
      new Option("--order, -o <order>", `Order`)
        .choices(ORDER_CHOICES)
        .default("asc"),
    )
    .addOption(
      new Option("--format, -f <format>", `Data format`)
        .choices(DATA_FORMAT_CHOICES)
        .default("pretty"),
    )
    .argument("<patterns...>")
    .action(
      async (
        patterns: string[],
        opts: {
          timeframe: string;
          offset?: string;
          limit?: string;
          order: Order;
          format: DataFormat;
        },
      ) => {
        const client = await loadClient();
        let items: any[] = [];
        for (const pattern of patterns) {
          const sym = await client.sym({ tickerId: pattern });
          const ohlcv = await client.ohlcv(
            sym,
            qp.Timeframe.fromString(opts.timeframe),
            {
              pb: true,
              order: opts.order,
              limit: tryParseInt(opts.limit),
              offset: tryParseInt(opts.offset),
            },
          );
          const _items = ohlcv.bars.map((r) => ({
            tickerId: sym.tickerId,
            ...r.toJSON(),
          }));
          items.push(..._items);
        }
        if (items.length === 0) {
          throw new CliError("No bars found");
        }
        if (opts.format === "table") {
          console.table(items);
        } else {
          throw new CliError(`Unsupported data format: ${opts.format}`);
        }
      },
    );
  program
    .command("check")
    .option("--cwd <path>", `Project root directory`)
    .option("--config <path>", `Config file`)
    .option("--watch, -w", `Recheck on file changes`, false)
    .action(
      async (opts: { cwd?: string; config?: string; watch?: boolean }) => {
        const client = await loadClient();
        const { config: qpcConfig, configPath: qpcConfigPath } = await initQpc({
          dir: opts.cwd,
          configPath: opts.config,
          prompt: false,
        });
        const driver = new RemoteDriver(
          {
            client: client["compilerClient"],
            qpcConfig,
            rootDir: dirname(qpcConfigPath),
            grpcMetadata: client["createGrpcMetadata"](),
          },
          client["http"],
        );
        if (opts.watch) {
          console.log(`${QPACE_BG_PREFIX}Watching for file changes`);
          watchPaths(
            { included: await driver.collectSrcFiles() },
            async (path) => {
              console.log(chalk.blackBright(`${path}`));
              await driver.check();
            },
          );
        }
        const res = await driver.check();
        if (!opts.watch && !res.ok) {
          process.exitCode = 1;
          return;
        }
      },
    );
  program
    .command("build")
    .addOption(
      new Option("--target <target>", `Target platform`).choices(BUILD_TARGETS),
    )
    .option("--cwd <path>", `Project root directory`)
    .option("--config <path>", `Config file`)
    .option("--emit", `Emits compiled files. Default: "config.emit"`)
    .option("--emit-dir <path>", `Output directory. Default: "config.emitDir"`)
    .option("--dir <path>", `Output directory. Default: "config.buildDir"`)
    .option(
      "--skip-wheel-install",
      `Skips installing python wheel artifact. "config.python.installWheel"`,
    )
    .option(
      "--skip-wheel-test",
      `Skips testing python wheel artifact. "config.python.testWheel"`,
      false,
    )
    .option("--verbose, -v", `Verbose output`)
    .option("--watch, -w", `Rebuild on file changes`, false)
    .action(
      async (opts: {
        cwd?: string;
        config?: string;
        emit?: boolean;
        target?: BuildTarget;
        emitDir?: string;
        dir?: string;
        verbose?: boolean;
        watch?: boolean;
        skipWheelTest?: boolean;
        skipWheelInstall?: boolean;
      }) => {
        const client = await loadClient();
        const { config: qpcConfig, configPath: qpcConfigPath } = await initQpc({
          dir: opts.cwd,
          configPath: opts.config,
          prompt: false,
        });
        qpcConfig.python ??= {};
        qpcConfig.emitDir = opts.emitDir ?? qpcConfig.emitDir;
        qpcConfig.emit = opts.emit ?? qpcConfig.emit;
        qpcConfig.buildDir = opts.dir ?? qpcConfig.buildDir;
        if (opts.skipWheelInstall) {
          qpcConfig.python.installWheel = false;
        }
        if (opts.skipWheelTest) {
          qpcConfig.python.testWheel = false;
        }
        if (opts.target == null && !qpcConfig.emit) {
          // if (qpcConfig.python.bindings) {
          //   opts.target = "python";
          // } else {
          // }
          throw new CliError(`--target must be specified or --emit enabled`);
        }
        const target = tryMapBuildTarget(opts.target as any);
        if (opts.target != null && target == null) {
          throw new CliError(`Unsupported target: ${opts.target}`);
        }

        const driver = new RemoteDriver(
          {
            client: client["compilerClient"],
            qpcConfig,
            rootDir: dirname(qpcConfigPath),
            verbose: opts.verbose,
            grpcMetadata: client["createGrpcMetadata"](),
          },
          client["http"],
        );
        if (opts.watch) {
          console.log(`${QPACE_BG_PREFIX}Watching for file changes`);
          watchPaths(
            { included: await driver.collectSrcFiles() },
            async (path) => {
              console.log(chalk.blackBright(`${path}`));
              await driver.build({ target });
            },
          );
        }
        const res = await driver.build({ target });
        if (!opts.watch && !res.ok) {
          process.exitCode = 1;
          return;
        }
      },
    );
  program
    .command("run")
    .option("--cwd <path>", `Project root directory`)
    .option("--config <path>", `Config file`)
    .option("--verbose, -v", `Verbose output`)
    .option("--plot", `Plot results`, true)
    .option("--timeframe, -t <timeframe>", `Timeframe`, "1D")
    .option("--force-skip-build", `Skip build`, false)
    .option("--start", `Start date`)
    .option("--end", `End date`)
    .argument("<filename>")
    .argument("<symbol>")
    .action(
      async (
        filename: string,
        symbol: string,
        opts: {
          cwd?: string;
          config?: string;
          dir?: string;
          verbose?: boolean;
          plot?: boolean;
          timeframe: string;
          forceSkipBuild?: boolean;
          start?: string;
          end?: string;
        },
      ) => {
        const startDate = opts.start ? new Date(opts.start) : undefined;
        const endDate = opts.end ? new Date(opts.end) : undefined;

        console.log(chalk.yellowBright(`Warning: Experimental feature`));
        if (!filename.endsWith(".pine")) {
          throw new CliError(
            `Can only run ${chalk.white.bold(
              ".pine",
            )} files, got ${chalk.yellowBright(basename(filename))}`,
          );
        }
        const pythonPath = await findPythonPath();
        if (pythonPath == null) {
          throw new CliError(
            `Python not found. Install python and or it to PATH`,
          );
        }
        const pyqpace = await exec({
          command: `${pythonPath} -m pip show qpace`,
        });
        if (pyqpace.exitCode !== 0) {
          throw new CliError(
            `Python package "qpace" not found. Install it using ${chalk.bold.white(
              `"${pythonPath} -m pip install qpace --break-system-packages"`,
            )}`,
          );
        }

        const client = await loadClient();
        const timeframe = qp.Timeframe.fromString(opts.timeframe);
        const sym = await client.sym({
          id: symbol,
          tickerId: symbol,
          timeframe,
        });
        if (sym == null) {
          throw new CliError(
            `Symbol ${symbol} not found. Use ${chalk.white.bold(
              `"qpc sym"`,
            )} to list available symbols`,
          );
        }

        const { config: qpcConfig, configPath: qpcConfigPath } = await initQpc({
          dir: opts.cwd,
          configPath: opts.config,
          prompt: false,
        });
        const target = tryMapBuildTarget("python");
        if (target == null) {
          throw new CliError(`Unsupported target`);
        }

        if (!qpcConfig.python?.bindings) {
          throw new CliError(
            `Python bindings are not enabled. Set ${chalk.bold.white(
              `"python.bindings: true"`,
            )} in ${chalk.bold.white(`"${qpcConfigPath}"`)}`,
          );
        }
        if (!qpcConfig.python?.installWheel) {
          throw new CliError(
            `Set ${chalk.bold.white(
              `"python.installWheel: true"`,
            )} in ${chalk.bold.white(`"${qpcConfigPath}"`)}`,
          );
        }
        const driver = new RemoteDriver(
          {
            client: client["compilerClient"],
            qpcConfig,
            rootDir: dirname(qpcConfigPath),
            verbose: opts.verbose,
            grpcMetadata: client["createGrpcMetadata"](),
          },
          client["http"],
        );
        const filePath = resolve(dirname(qpcConfigPath), filename);
        const srcFiles = await driver.collectSrcFiles();
        if (!srcFiles.some((r) => resolve(driver.rootDir, r) === filePath)) {
          throw new CliError(
            `File ${chalk.yellowBright(
              filePath,
            )} is not included in source files. Include it in ${chalk.bold.white(
              `"${qpcConfigPath}"`,
            )}`,
          );
        }
        console.log(`${QPACE_BG_PREFIX}${chalk.green(filename)}`);
        if (!opts.forceSkipBuild) {
          const res = await driver.build({ target });
          if (!res.ok) {
            process.exitCode = 1;
            return;
          }
          if (res.wheelPath == null) {
            throw new CliError(`Python wheel not found`);
          }
        }
        const pineTarget = `${withoutExt(basename(filePath))}`;

        const py = `
import sys
import qpace as qp
import matplotlib.pyplot as plt
from datetime import datetime
import ${qpcConfig.python?.package ?? ""} as pine
client = qp.Client(api_key="${client["config"]["apiKey"]}")
sym = client.sym(id="${sym.id}")
ohlcv = client.ohlcv(sym, "${opts.timeframe}", pb=True)
ctx = qp.Ctx(sym=sym, ohlcv=ohlcv)

if pine.${pineTarget}.Script.Kind != "strategy":
  raise Exception("Only strategies are supported at the moment")

#script = pine.${pineTarget}.Script(ctx=ctx.fork())
#script.collect()
#bt = script.bt
#print(bt.equity)
bt = qp.Backtest(ctx.fork(), config=qp.BacktestConfig())
for bar_index in bt:
    if bar_index == 100:
        bt.signal(qp.Signal.long())
    if bar_index == 200:
        bt.signal(qp.Signal.close_all())
close: list[float] = ohlcv.close
open_time: list[datetime] = ohlcv.open_time
equity_list: list[float] = bt.equity_list
net_equity_list: list[float] = bt.net_equity_list

ema = qp.ta.ema(ctx.fork(), ohlcv.close, 90)

# @TODO: Fix this
equity_list = equity_list[0:len(close)]
net_equity_list = net_equity_list[0:len(close)]
ema = ema[0:len(close)]

fig, (ax1, ax2) = plt.subplots(2, 1)
ax1.plot(open_time, close, label="Close", color="black")
ax1.plot(open_time, ema, label="EMA", color="orange")
ax2.plot(open_time, equity_list, label="Equity", color="blue")
ax2.plot(open_time, net_equity_list, label="Net Equity", color="red")
ax1.legend()
ax2.legend()
plt.show()
`.trim();
        // console.log(py);
        const pyBash = Buffer.from(py, "utf8").toString("base64");
        // Avoid bash escaping issues
        await exec({
          command: `${pythonPath} -c "import base64; exec(base64.b64decode('${pyBash}').decode('utf-8'))"`,
          io: true,
        });
      },
    );
  program.addCommand(
    new Command("telemetry")
      .action(async () => {
        await promptTelemetry();
      })
      .addCommand(
        new Command("enable").action(async () => {
          await promptTelemetry(true);
        }),
      )
      .addCommand(
        new Command("disable").action(async () => {
          await promptTelemetry(false);
        }),
      ),
  );
  program.parse();
};

main();
