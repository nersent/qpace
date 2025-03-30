import { writeFileSync } from "fs";
import { readFile, writeFile } from "fs/promises";
import { basename, dirname, normalize, relative, resolve } from "path";

import { ClientReadableStream } from "@grpc/grpc-js";
import * as grpc from "@grpc/grpc-js";
import chalk from "chalk";
import { glob } from "glob";
import ora, { Ora } from "ora";

import { Config, FileTag, Target } from "./compiler";
import { CompilerApiClient } from "./proto/compiler_grpc_pb";
import { BuildRequest } from "./proto/compiler_pb";
import * as compilerApi from "./proto/compiler_pb";

import { exec } from "~/base/node/exec";
import { createDir } from "~/base/node/fs";
import { prettifyTime } from "~/base/node/time";

interface DriverContextConfig {
  qpcConfig: Config;
  checkOnly?: boolean;
  target?: Target;
  verbose?: boolean;
  rootDir: string;
  client: CompilerApiClient;
  grpcMetadata?: grpc.Metadata;
}

type CheckpointTime = [number | undefined, number | undefined];

namespace CheckpointTime {
  export const def = (): CheckpointTime => [undefined, undefined];
  export const start = (time: CheckpointTime): void => {
    time[0] = Date.now();
  };
  export const end = (time: CheckpointTime): void => {
    time[1] = Date.now();
  };
  export const diff = (time: CheckpointTime): number | undefined => {
    if (time[0] == null || time[1] == null) return undefined;
    return time[1] - time[0];
  };
}

class DriverContext {
  private stream: ClientReadableStream<compilerApi.StageEvent> | undefined;
  private ora: Ora | undefined;
  private time = CheckpointTime.def();
  private checkTime = CheckpointTime.def();
  private emitTime = CheckpointTime.def();
  private buildTime = CheckpointTime.def();
  private wheelInstallTime = CheckpointTime.def();
  private wheelTestTime = CheckpointTime.def();
  private ok = true;

  constructor(private readonly config: DriverContextConfig) {}

  public async start(srcPaths: string[]): Promise<void> {
    return new Promise<void>(async (_resolve, _reject) => {
      const { checkOnly, verbose, qpcConfig, target, rootDir } = this.config;
      CheckpointTime.start(this.time);

      const req = await this.createBuildRequest(srcPaths);
      this.stream = this.config.client.build(req, this.config.grpcMetadata);
      this.stream.on("error", (err) => {
        this.ok = false;
        this.ora?.fail();
        console.error(chalk.redBright(err));
        _reject(err);
      });
      this.stream.on("data", (e: compilerApi.StageEvent) => this.onStage(e));

      this.ora = ora(`Checking`).start();
      if (this.ok) {
        let e = await this.waitForStage("check_end");
        this.ora.text = `${this.ora.text} (${prettifyTime(
          CheckpointTime.diff(this.checkTime),
        )})`;
        const checkEvent = e.getCheckEnd();
        if (checkEvent == null) {
          throw new Error(`Expected check end event`);
        }
        if (checkEvent.getOk()) {
          this.ora = this.ora.succeed();
        } else {
          this.ora = this.ora.fail();
          this.ok = false;
          this.printMessage(checkEvent.getMessage());
        }
      }
      if (this.ok && qpcConfig.emit) {
        this.ora = ora(`Emitting ${target ?? ""}`.trim()).start();
        const e = await this.waitForStage("emit_end");
        const emitEvent = e.getEmitEnd();
        if (emitEvent == null) {
          throw new Error(`Expected emit end event`);
        }
        this.ora.stop();
        await Promise.all(
          emitEvent.getFilesList().map((r) => this.onReceivedFile(r)),
        );
        if (emitEvent.getOk()) {
          this.ora.succeed(
            `${this.ora.text} (${prettifyTime(
              CheckpointTime.diff(this.emitTime),
            )})`,
          );
        } else {
          this.ok = false;
          this.ora.fail();
          this.printMessage(emitEvent.getMessage());
        }
      }
      if (this.ok && !checkOnly && target != null) {
        this.ora = ora(`Building ${target ?? ""}`.trim()).start();
        const e = await this.waitForStage("build_end");
        const buildEvent = e.getBuildEnd();
        if (buildEvent == null) {
          throw new Error(`Expected build end event`);
        }
        if (buildEvent.getOk()) {
          this.ora.succeed(
            `${this.ora.text} (${prettifyTime(
              CheckpointTime.diff(this.buildTime),
            )})`,
          );
        } else {
          this.ok = false;
          this.ora.fail();
          this.printMessage(buildEvent.getMessage());
        }

        if (
          this.ok &&
          target?.startsWith("python") &&
          qpcConfig.python?.installWheel
        ) {
          const wheelFile = buildEvent.getWheel();
          if (wheelFile == null) {
            throw new Error(`Expected wheel file`);
          }
          const wheelPath = await this.onReceivedFile(wheelFile);
          this.ora = ora(`Installing ${relative(rootDir, wheelPath)}`).start();
          CheckpointTime.start(this.wheelInstallTime);
          const installWheelRes = await exec({
            command: `pip install "${wheelPath}" --force-reinstall`,
            io: verbose,
          });
          CheckpointTime.end(this.wheelInstallTime);
          this.ora.text = `${this.ora.text} (${prettifyTime(
            CheckpointTime.diff(this.wheelInstallTime),
          )})`;
          if (installWheelRes.exitCode === 0) {
            this.ora.succeed();
          } else {
            this.ora.fail();
            this.ok = false;
            process.stdout.write(installWheelRes.stdout);
            process.stderr.write(installWheelRes.stderr);
          }
          if (qpcConfig.python.testWheel) {
            this.ora = ora(`Testing python wheel`).start();
            CheckpointTime.start(this.wheelTestTime);
            const testWheelRes = await exec({
              command: `python -c "import ${qpcConfig.python.package}`,
              io: verbose,
            });
            CheckpointTime.end(this.wheelTestTime);
            this.ora.text = `${this.ora.text} (${prettifyTime(
              CheckpointTime.diff(this.wheelTestTime),
            )})`;
            if (testWheelRes.exitCode === 0) {
              if (testWheelRes.stdout.includes("warnings.warn")) {
                this.ora.warn();
                process.stdout.write(testWheelRes.stdout);
              } else {
                this.ora.succeed();
              }
            } else {
              this.ora.fail();
              this.ok = false;
              process.stdout.write(testWheelRes.stdout);
              process.stderr.write(testWheelRes.stderr);
            }
          }
        }
      }
      CheckpointTime.end(this.time);
      this.logEnd();
    });
  }

  private printMessage(message?: string): void {
    console.log(resolve("xd.txt"));
    if (message?.length) console.log(message);
  }

  private waitForStage(
    stage: "check_end" | "emit_end" | "build_end",
  ): Promise<compilerApi.StageEvent> {
    return new Promise((_resolve) => {
      this.stream?.on("data", (e: compilerApi.StageEvent) => {
        if (stage === "check_end" && e.hasCheckEnd()) {
          return _resolve(e);
        }
        if (stage === "emit_end" && e.hasEmitEnd()) {
          return _resolve(e);
        }
        if (stage === "build_end" && e.hasBuildEnd()) {
          return _resolve(e);
        }
      });
    });
  }

  private onStage(e: compilerApi.StageEvent): void {
    if (e.hasMessage()) {
      console.log(e.getMessage());
      return;
    }
    if (e.hasCheckStart()) {
      CheckpointTime.start(this.checkTime);
      return;
    }
    if (e.hasCheckEnd()) {
      CheckpointTime.end(this.checkTime);
      return;
    }
    if (e.hasEmitStart()) {
      CheckpointTime.start(this.emitTime);
      return;
    }
    if (e.hasEmitEnd()) {
      CheckpointTime.end(this.emitTime);
      return;
    }
    if (e.hasBuildStart()) {
      CheckpointTime.start(this.buildTime);
      return;
    }
    if (e.hasBuildEnd()) {
      CheckpointTime.end(this.buildTime);
      return;
    }
  }

  private async createBuildRequest(srcPaths: string[]): Promise<BuildRequest> {
    const { verbose, target, qpcConfig, rootDir, checkOnly } = this.config;
    if (verbose) {
      console.log(chalk.blackBright(srcPaths.map((r) => `← ${r}`).join("\n")));
    }
    const req = new BuildRequest();
    req.setQpcConfig(JSON.stringify(qpcConfig));
    if (checkOnly) req.setCheckOnly(checkOnly);
    if (target != null) req.setTarget(target);
    const reqFiles: compilerApi.File[] = [];
    reqFiles.push(
      ...(await Promise.all(
        srcPaths.map(async (path) => {
          const data = await readFile(resolve(rootDir, path));
          return new compilerApi.File().setPath(path).setData(data);
        }),
      )),
    );
    req.setFilesList(reqFiles);
    return req;
  }

  private async onReceivedFile(file: compilerApi.File): Promise<string> {
    const { verbose, rootDir } = this.config;
    if (verbose) {
      console.log(chalk.blackBright(`→ ${normalize(file.getPath())}`));
    }
    const path = resolve(rootDir, file.getPath());
    const data = Buffer.from(file.getData_asU8());

    await createDir(dirname(path));
    await writeFile(path, data);

    return path;
  }

  private logEnd(): void {
    if (this.ok) {
      console.log(
        chalk.green(
          `Finished in ${prettifyTime(CheckpointTime.diff(this.time))}`,
        ),
      );
    } else {
      console.log(
        chalk.redBright(
          `Failed in ${prettifyTime(CheckpointTime.diff(this.time))}`,
        ),
      );
      process.exitCode = 1;
    }
  }
}

export interface RemoteDriverOpts {
  qpcConfig: Config;
  client: CompilerApiClient;
  rootDir: string;
  verbose?: boolean;
  grpcMetadata?: grpc.Metadata;
}

export class RemoteDriver {
  public readonly qpcConfig: Config;
  public readonly client: CompilerApiClient;
  public readonly rootDir: string;
  public readonly verbose: boolean;
  private readonly grpcMetadata?: grpc.Metadata;

  constructor(opts: RemoteDriverOpts) {
    this.qpcConfig = opts.qpcConfig;
    this.client = opts.client;
    this.rootDir = opts.rootDir;
    this.verbose = opts.verbose ?? false;
    this.grpcMetadata = opts.grpcMetadata;
  }

  public async collectSrcFiles(): Promise<string[]> {
    const cwd = this.rootDir;
    const qpcConfig = this.qpcConfig;
    const excludes: string[] = [...(qpcConfig.exclude ?? [])];
    const includes: string[] = [...(qpcConfig.include ?? [])];
    const srcPaths: string[] = [];
    srcPaths.push(
      ...(await Promise.all(
        includes.map((r) => glob(r, { cwd, ignore: excludes })),
      ).then((r) => r.flat())),
    );
    return srcPaths.map((r) => basename(r, cwd));
  }

  public async build({ target }: { target?: Target }): Promise<void> {
    const ctx = new DriverContext({
      qpcConfig: this.qpcConfig,
      rootDir: this.rootDir,
      client: this.client,
      target,
      verbose: this.verbose,
      grpcMetadata: this.grpcMetadata,
    });
    return ctx.start(await this.collectSrcFiles());
  }

  public async check(): Promise<void> {
    const ctx = new DriverContext({
      qpcConfig: this.qpcConfig,
      checkOnly: true,
      rootDir: this.rootDir,
      client: this.client,
      verbose: this.verbose,
      grpcMetadata: this.grpcMetadata,
    });
    return ctx.start(await this.collectSrcFiles());
  }
}
