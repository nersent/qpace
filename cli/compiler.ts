import { Command, Option } from "commander";
import { CompilerApiClient } from "~/compiler/schema_grpc_pb";
import * as grpc from "@grpc/grpc-js";
import * as compilerApi from "~/compiler/schema_pb";
import { promisify } from "util";
import { unwrap } from "~/base/js/assert";
import chalk from "chalk";
import { glob } from "glob";
import ora, { Ora } from "ora";
import {
  Config,
  getInitConfig,
  mergeConfigs,
  QPC_CONFIG_FILENAME,
  Target,
  TARGETS,
  tryGetJsPackageName,
  tryGetPythonPackageName,
} from "~/compiler/config";
import { basename, dirname, resolve } from "path";
import { exists, readJson, writeJson } from "~/base/node/fs";
import { CliError } from "./exceptions";
import { mkdir, readFile, writeFile } from "fs/promises";
import { isLinux, isMacOs, isWindows, which } from "~/base/node/os";
import { mkdirSync } from "fs";
import { downloadFileToPath } from "~/base/node/network";
import { Profile } from "./profile";
import axios, { AxiosInstance } from "axios";
import { getPythonVersion, locatePython } from "~/base/node/python";
import { exec } from "~/base/node/exec";
import { detectNodePackageManager, installNodePackage } from "./utils";

export const BUILD_TARGETS = [
  ...TARGETS,
  "python",
  "node",
  "web",
  "wasm",
  "js",
] as const;
export type BuildTarget = typeof BUILD_TARGETS[number];

export namespace BuildTarget {
  export const tryIntoTarget = (
    buildTarget: BuildTarget,
  ): Target | undefined => {
    if (TARGETS.includes(buildTarget as any)) {
      return buildTarget as Target;
    }
    const arch = process.arch;
    {
      if (buildTarget === "python" && isMacOs() && arch === "x64") {
        return "python-x86_64-macos";
      }
      if (buildTarget === "python" && isMacOs() && arch === "arm64") {
        return "python-arm64-macos";
      }
      if (buildTarget === "python" && isLinux() && arch === "x64") {
        return "python-x86_64-linux";
      }
      if (buildTarget === "python" && isLinux() && arch === "arm64") {
        return "python-arm64-linux";
      }
      if (buildTarget === "python" && isWindows() && arch === "x64") {
        return "python-x86_64-windows";
      }
      //
      if (buildTarget === "node" && isMacOs() && arch === "x64") {
        return "node-x86_64-macos";
      }
      if (buildTarget === "node" && isMacOs() && arch === "arm64") {
        return "node-arm64-macos";
      }
      if (buildTarget === "node" && isLinux() && arch === "x64") {
        return "node-x86_64-linux";
      }
      if (buildTarget === "node" && isLinux() && arch === "arm64") {
        return "node-arm64-linux";
      }
      if (buildTarget === "node" && isWindows() && arch === "x64") {
        return "node-x86_64-windows";
      }
      //
      if (buildTarget === "web" || buildTarget === "wasm") {
        return "wasm-unknown-unknown";
      }
      if (buildTarget === "js") {
        return "js-universal";
      }
    }
    return;
  };
}

export const getCompilerClient = async (): Promise<
  [CompilerApiClient, grpc.Metadata]
> => {
  const profile = await Profile.load();
  const client = await profile.getClient();
  return [
    new CompilerApiClient(
      unwrap(client.config.grpcApiBase),
      unwrap(client.config.grpcCredentials),
      unwrap(client["_grpcOptions"]),
    ),
    client["grpcMetadata"],
  ];
};

export const fetchInfo = async (
  client: CompilerApiClient,
): Promise<{ version: string; buildDate: Date }> => {
  const info = (await promisify(client.info.bind(client))(
    new compilerApi.InfoRequest(),
  )) as compilerApi.InfoResponse;
  return {
    version: info.getVersion(),
    buildDate: unwrap(info.getBuildTime()?.toDate()),
  };
};

export const isValidNpmPackageName = (name: string): boolean => {
  if (name == null) return false;
  name = name.trim();
  if (typeof name !== "string" || name.length === 0 || name.length > 214)
    return false;
  if (/[A-Z]/.test(name)) return false; // no capitals
  if (/^[._]/.test(name)) return false; // no leading . or _
  if (/\s/.test(name)) return false; // no whitespace

  // scoped package?
  if (name.startsWith("@")) {
    const [scope, pkg] = name.split("/");
    if (!scope || !pkg) return false; // need exactly @scope/pkg

    const scopeOk = /^@[a-z0-9][a-z0-9._-]*$/.test(scope);
    const pkgOk = /^[a-z0-9][a-z0-9._-]*$/.test(pkg);
    return scopeOk && pkgOk;
  }

  // unscoped
  return /^[a-z0-9][a-z0-9._-]*$/.test(name);
};

export const isValidPipPackageName = (name: string): boolean => {
  if (name == null) return false;
  name = name.trim();
  if (typeof name !== "string" || name.length === 0 || name.length > 255)
    return false;
  // core rule: first/last char alnum; middle chars alnum, dot, dash, underscore
  return /^[A-Za-z0-9]+([A-Za-z0-9._-]*[A-Za-z0-9])?$/.test(name);
};

export const tryLoadQpcConfig = async (
  path: string,
): Promise<Config | undefined> => {
  let config = getInitConfig();
  if (await exists(path)) {
    config = mergeConfigs(config, await readJson(path));
  }
  return config;
};

export const loadQpcConfig = async (path: string): Promise<Config> => {
  return unwrap(
    await tryLoadQpcConfig(path),
    `QPC config file not found at "${chalk.yellow(path)}"`,
  );
};

const collectSrcPaths = async (
  config: Config,
  cwd: string,
): Promise<string[]> => {
  const excludes: string[] = [...(config.exclude ?? [])];
  const includes: string[] = [...(config.include ?? [])];
  const srcPaths: string[] = [];
  srcPaths.push(
    ...(await Promise.all(
      includes.map((r) => glob(r, { cwd, ignore: excludes })),
    ).then((r) => r.flat())),
  );
  return srcPaths;
};

const readSrcFiles = async (
  relPaths: string[],
  dir: string,
): Promise<compilerApi.File[]> => {
  return Promise.all(
    relPaths.map(async (path) => {
      const file = new compilerApi.File();
      const data = await readFile(resolve(dir, path));
      file.setPath(path);
      file.setData(data);
      return file;
    }),
  );
};

const printOutcoming = (items: string[]): void => {
  if (items.length === 0) return;
  console.log(chalk.blackBright(items.map((r) => `← ${r}`).join("\n")));
};

const printIncoming = (items: string[]): void => {
  if (items.length === 0) return;
  console.log(chalk.blackBright(items.map((r) => `→ ${r}`).join("\n")));
};

const check = async ({
  cwd,
  config: configPath,
  verbose,
}: {
  cwd?: string;
  config?: string;
  verbose?: boolean;
}): Promise<void> => {
  const profile = await Profile.load();
  await profile.ping();
  cwd ??= process.cwd();
  verbose && console.log(chalk.blackBright(`CWD: ${cwd}`));
  const [compilerClient, grpcMetadata] = await getCompilerClient();
  const qpcConfig = await loadQpcConfig(
    configPath ?? resolve(cwd, QPC_CONFIG_FILENAME),
  );
  const srcPaths = await collectSrcPaths(qpcConfig, cwd);
  verbose && printOutcoming(srcPaths);
  const srcFiles = await readSrcFiles(srcPaths, cwd);
  const req = new compilerApi.CheckRequest();
  req.setQpcConfig(JSON.stringify(qpcConfig));
  req.setFilesList(srcFiles);
  let pb = ora(`Checking`).start();
  let ok = true;
  const res = await new Promise<compilerApi.CheckResponse>(
    (resolve, reject) => {
      compilerClient.check(req, grpcMetadata, (err, data) => {
        if (err) return reject(err);
        resolve(data);
      });
    },
  );
  ok = res.getOk();
  const logs = res.getMessage();
  pb.stop();
  if (logs?.length) {
    console.log(logs);
  }
  if (ok) {
    pb.succeed(`Ok`);
  } else {
    pb.fail(`Failed`);
    process.exitCode = 1;
  }
  if (verbose) {
    console.log(`Request ID: ${chalk.magentaBright(res.getRequestId())}`);
  }
};

const writeApiFile = async (
  file: compilerApi.File,
  dir: string,
  axiosInstance?: AxiosInstance,
): Promise<void> => {
  const path = resolve(dir, file.getPath());
  mkdirSync(dirname(path), { recursive: true });
  if (file.hasData()) {
    const data = file.getData_asU8();
    await writeFile(path, data);
  } else if (file.hasUrl()) {
    try {
      await downloadFileToPath(unwrap(file.getUrl()), path, axiosInstance);
    } catch (e) {
      if (axios.isAxiosError(e)) {
        throw new CliError(
          `Could not download file from ${file.getUrl()}\n${e.message}`,
        );
      }
      throw e;
    }
  } else {
    throw new CliError(`File "${file.getPath()}" has no data or URL`);
  }
};

const getApiFileFlags = (file: compilerApi.File) => {
  const flags = file.getFlags();
  return {
    pythonWheel: Boolean(flags & compilerApi.FileFlag.FILE_FLAG_PYTHON_WHEEL),
    npmTar: Boolean(flags & compilerApi.FileFlag.FILE_FLAG_NPM_TAR),
  };
};

const build = async ({
  target: rawTarget,
  cwd,
  config: configPath,
  emit,
  outDir,
  skipInstall,
  skipTest,
  verbose,
}: {
  target?: BuildTarget;
  cwd?: string;
  config?: string;
  emit?: boolean;
  outDir?: string;
  skipInstall?: boolean;
  skipTest?: boolean;
  verbose?: boolean;
}): Promise<void> => {
  if (rawTarget == null && !emit) {
    throw new CliError(`You must specify --target or use --emit option`);
  }

  let target: Target | undefined;
  if (rawTarget != null) {
    target = BuildTarget.tryIntoTarget(rawTarget);
    if (target == null) throw new CliError(`Invalid target: ${rawTarget}`);
  }

  const profile = await Profile.load();
  await profile.ping();
  const client = await profile.getClient();

  cwd ??= process.cwd();
  verbose && console.log(chalk.blackBright(`CWD: ${cwd}`));

  const pythonPath = await locatePython();
  const pythonVersion = await getPythonVersion(pythonPath);

  client["_clientInfo"]["python"] = pythonVersion;

  const [compilerClient, grpcMetadata] = await getCompilerClient();
  const qpcConfig = await loadQpcConfig(
    configPath ?? resolve(cwd, QPC_CONFIG_FILENAME),
  );
  qpcConfig.py ??= {};
  qpcConfig.js ??= {};
  qpcConfig.node ??= {};
  qpcConfig.web ??= {};
  qpcConfig.emit ||= emit;
  qpcConfig.outDir = outDir ?? qpcConfig.outDir;

  if (skipInstall) qpcConfig.install = false;
  if (skipTest) qpcConfig.test = false;

  if (target?.startsWith("python") && qpcConfig.install) {
    if (pythonPath == null)
      throw new CliError(`Python not found. Install python and or it to PATH`);
    {
      const res = await exec({
        command: `${pythonPath} -m pip show qpace`,
      });
      if (res.exitCode !== 0) {
        throw new CliError(
          `Python package "qpace" not found. Install it using ${chalk.yellowBright(
            `"${pythonPath} -m pip install qpace --break-system-packages"`,
          )}`,
        );
      }
    }
  }

  // console.log("XD", qpcConfig);
  // process.exit();

  const srcPaths = await collectSrcPaths(qpcConfig, cwd);
  verbose && printOutcoming(srcPaths);
  const srcFiles = await readSrcFiles(srcPaths, cwd);

  const req = new compilerApi.BuildRequest();
  req.setQpcConfig(JSON.stringify(qpcConfig));
  req.setFilesList(srcFiles);
  if (target != null) req.setTarget(target);

  let pb: Ora | undefined;
  pb = ora(`Checking`).start();
  let ok = true;

  const stream = compilerClient.build(req, grpcMetadata);

  const fail = (msg?: string): void => {
    pb?.fail(msg ?? `Failed`);
    ok = false;
  };

  await new Promise<void>((_resolve, _reject) => {
    stream.on("error", (err) => {
      console.error(chalk.redBright(err));
      fail();
      _reject(err);
    });

    stream.on("data", async (e: compilerApi.BuildEvent) => {
      if (e.hasCheckEnd()) {
        const data = e.getCheckEnd()!;
        const reqId = data.getRequestId();
        if (verbose) {
          console.log(
            chalk.blackBright(`\nRequest ID: ${chalk.magentaBright(reqId)}`),
          );
        }
        ok = data.getOk();
        pb.clear();
        const logs = data.getMessage();
        if (logs?.length) console.log(`${logs}\n`);
        if (!ok) _resolve();
        return;
      }
      if (e.hasBuildStart()) {
        pb.start(`Building ${chalk.blueBright(target)}`);
        return;
      }
      if (e.hasBuildEnd()) {
        const data = e.getBuildEnd()!;
        const files = data.getFilesList();
        const profile = await Profile.load();
        const client = await profile.getClient();
        await Promise.all(
          files.map((f) => writeApiFile(f, cwd, unwrap(client["http"]))),
        );

        if (data.getOk()) {
          if (target?.includes("python")) {
            const pythonWheelFile = files.find(
              (f) => getApiFileFlags(f).pythonWheel,
            );
            if (pythonWheelFile == null) {
              fail(`Python wheel file not produced`);
            }
            if (pythonWheelFile != null && qpcConfig.install) {
              pb.text = `Installing Python wheel`;
              const path = resolve(cwd, pythonWheelFile.getPath());
              {
                const res = await exec({
                  command: `${pythonPath} -m pip install "${path}" --break-system-packages`,
                  io: verbose,
                });
                if (res.exitCode !== 0 || res.stdout.includes("ERROR")) {
                  fail(`Failed to install Python wheel`);
                  process.stdout.write(res.stdout);
                  process.stderr.write(res.stderr);
                }
              }
              if (ok && qpcConfig.test) {
                pb.text = `Testing Python wheel`;
                const res = await exec({
                  command: `${pythonPath} -c "import ${tryGetPythonPackageName(
                    qpcConfig.py,
                  )}"`,
                  io: verbose,
                });
                if (res.exitCode !== 0 || res.stdout.includes("ERROR")) {
                  fail(`Failed Python wheel test`);
                  process.stdout.write(res.stdout);
                  process.stderr.write(res.stderr);
                }
              }
            }
          } else if (
            target?.includes("node") ||
            target?.includes("wasm") ||
            target === "js-universal"
          ) {
            const npmTarFile = files.find((f) => getApiFileFlags(f).npmTar);
            if (npmTarFile == null) {
              fail(`NPM tar file not produced`);
            }
            if (npmTarFile != null && qpcConfig.install) {
              pb.text = `Installing NPM package`;
              const path = resolve(cwd, npmTarFile.getPath());
              const pkgManager = await detectNodePackageManager(cwd);
              const res = await installNodePackage(`"${path}"`, {
                cwd,
                manager: pkgManager,
                verbose,
              });
              if (res.exitCode !== 0) {
                fail(`Failed to install NPM package`);
                process.stdout.write(res.stdout);
                process.stderr.write(res.stderr);
              }
            }
          }
        } else {
          fail();
        }

        if (ok) pb?.succeed(`Ok`);
        verbose && printIncoming(files.map((f) => f.getPath()));
        const logs = data.getMessage();
        if (logs?.length) console.log(logs);

        _resolve();
        return;
      }
    });
  });

  if (ok && verbose) {
    console.log(chalk.greenBright(`\nUse following:`));
    if (target?.includes("python")) {
      console.log(
        chalk.greenBright(
          `import ${tryGetPythonPackageName(qpcConfig.py)} as pine;`,
        ),
      );
    } else if (target?.includes("node")) {
      console.log(
        chalk.greenBright(
          `import * as pine from "${tryGetJsPackageName(qpcConfig.js)}/node";`,
        ),
      );
    } else if (target?.includes("node")) {
      console.log(
        chalk.greenBright(
          `import * as pine from "${tryGetJsPackageName(qpcConfig.js)}/web";`,
        ),
      );
    } else if (target === "js-universal") {
      console.log(
        chalk.greenBright(
          `import * as pine from "${tryGetJsPackageName(
            qpcConfig.js,
          )}/node|web";`,
        ),
      );
    }
    console.log("");
  }

  if (!ok) {
    process.exitCode = 1;
  }

  pb?.stop();
};

export const getCommands = (): Command[] => {
  return [
    new Command("check")
      .description(
        "Analyzes the current project and reports errors, but doesn't build",
      )
      .option("--cwd <path>", `Project root directory`)
      .option("--config <path>", `Path to the QPC config file`)
      .option("--verbose", `Prints verbose output`, false)
      .action(check),
    new Command("build")
      .description(
        "Compiles the project, produces artifacts like Python wheel, NPM package and optionally installs + tests them",
      )
      .addOption(
        new Option("--target <target>", `Target platform`).choices(
          BUILD_TARGETS,
        ),
      )
      .option("--emit", `Emits compiled files. Default: "config.emit"`)
      .option(
        "--out-dir <path>",
        `Directory to emit compiled files and artifacts. Default: "config.outDir"`,
      )
      .option("--cwd <path>", `Project root directory`)
      .option("--config <path>", `Path to the QPC config file`)
      .option(
        "--skip-install",
        `Skips installing artifact. "config.python|node.install"`,
      )
      .option(
        "--skip-test",
        `Skips testing artifact. "config.python|node.test"`,
        false,
      )
      .option("--verbose", `Prints verbose output`, false)
      .action(build),
  ];
};
