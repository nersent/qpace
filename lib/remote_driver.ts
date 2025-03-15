import { readFile, writeFile } from "fs/promises";
import { basename, dirname, normalize, resolve } from "path";

import { ClientReadableStream } from "@grpc/grpc-js";
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

export interface RemoteDriverOpts {
  config: Config;
  client: CompilerApiClient;
  rootDir: string;
  verbose?: boolean;
}

export class RemoteDriver {
  public readonly config: Config;
  public readonly client: CompilerApiClient;
  public readonly rootDir: string;
  public readonly verbose: boolean;

  constructor(opts: RemoteDriverOpts) {
    this.config = opts.config;
    this.client = opts.client;
    this.rootDir = opts.rootDir;
    this.verbose = opts.verbose ?? false;
  }

  public async collectSrcFiles(): Promise<string[]> {
    const cwd = this.rootDir;
    const config = this.config;
    const excludes: string[] = [...(config.exclude ?? [])];
    const includes: string[] = [...(config.include ?? [])];
    const srcPaths: string[] = [];
    srcPaths.push(
      ...(await Promise.all(
        includes.map((r) => glob(r, { cwd, ignore: excludes })),
      ).then((r) => r.flat())),
    );
    return srcPaths.map((r) => basename(r, cwd));
  }

  public async build({ target }: { target?: Target }): Promise<void> {
    const shouldReceivePythonWheel =
      target?.startsWith("python") && this.config.python?.installWheel;

    const req = await this.createBuildRequest();
    if (target != null) {
      req.setTarget(target);
    }

    const stream = this.client.build(req);

    const startTime = Date.now();
    let spinner: Ora | undefined;
    let receivedPythonWheel = false;

    const res = await new Promise<compilerApi.BuildResponse>((_resolve) => {
      stream.on("data", async (e: compilerApi.BuildResponseEvent) => {
        if (e.hasMessage()) {
          const message = e.getMessage();
          if (message.length) {
            console.log(message);
          }
          return;
        }
        if (e.hasStart()) {
          spinner = ora(`Building`).start();
          return;
        }
        if (e.hasEnd()) {
          spinner = spinner?.succeed()?.stop();
          return;
        }
        if (e.hasResponse()) {
          const res = e.getResponse()!;
          const files = res.getFilesList();
          await Promise.all(files.map((r) => this.onReceivedFile(r)));
          const pythonWheelFile = files.find((r) =>
            r.getTagsList().includes(FileTag.QPC_PYTHON_WHEEL),
          );
          if (pythonWheelFile != null) {
            spinner = ora(`Installing python wheel`).start();
            receivedPythonWheel = true;
            const wheelPath = resolve(this.rootDir, pythonWheelFile.getPath());
            const execRes = await exec({
              command: `pip install "${wheelPath}" --force-reinstall`,
              io: this.verbose,
            });
            if (execRes.exitCode === 0) {
              spinner = spinner?.succeed()?.stop();
            } else {
              spinner = spinner?.fail()?.stop();
              res.setStatus(compilerApi.BuildStatus.ERROR);
              if (!this.verbose) {
                process.stdout.write(execRes.stdout);
                process.stderr.write(execRes.stderr);
              }
            }
          }
          _resolve(res);
          return;
        }
      });
    });

    let status = res.getStatus();
    if (shouldReceivePythonWheel && !receivedPythonWheel) {
      status = compilerApi.BuildStatus.ERROR;
      console.log(chalk.redBright(`Failed to receive python wheel`));
    }

    const endTime = Date.now();
    this.logEnd({ status, startTime, endTime });

    // if (this.config.python?.bindings) {
    //   console.log(chalk.blueBright(`Import from python using:`));
    //   console.log(
    //     chalk.blackBright(`import ${this.config.python.package} as pine`),
    //   );
    // }
  }

  public async check(): Promise<void> {
    const req = await this.createBuildRequest();
    req.setCheckOnly(true);
    const stream = this.client.build(req);

    const startTime = Date.now();
    const res = await this.resolveBuild(stream);
    const endTime = Date.now();

    this.logEnd({ status: res.getStatus(), startTime, endTime });
  }

  private logEnd({
    status,
    startTime,
    endTime,
  }: {
    status: compilerApi.BuildStatus;
    startTime: number;
    endTime: number;
  }): void {
    if (status === compilerApi.BuildStatus.ERROR) {
      console.log(
        chalk.redBright(`Failed in ${prettifyTime(endTime - startTime)}`),
      );
      process.exitCode = 1;
    } else if (status === compilerApi.BuildStatus.OK) {
      console.log(
        chalk.green(`Finished in ${prettifyTime(endTime - startTime)}`),
      );
    }
  }

  private async createBuildRequest(): Promise<BuildRequest> {
    const paths = await this.collectSrcFiles();
    console.log(chalk.blackBright(paths.map((r) => `← ${r}`).join("\n")));
    const req = new BuildRequest();
    req.setQpcConfig(JSON.stringify(this.config));
    const reqFiles: compilerApi.File[] = [];
    reqFiles.push(
      ...(await Promise.all(
        paths.map(async (path) => {
          const data = await readFile(resolve(this.rootDir, path));
          return new compilerApi.File().setPath(path).setData(data);
        }),
      )),
    );
    req.setFilesList(reqFiles);
    return req;
  }

  private resolveBuild(
    stream: ClientReadableStream<compilerApi.BuildResponseEvent>,
  ): Promise<compilerApi.BuildResponse> {
    return new Promise<compilerApi.BuildResponse>((_resolve) => {
      stream.on("data", async (e: compilerApi.BuildResponseEvent) => {
        if (e.hasResponse()) {
          _resolve(e.getResponse()!);
          return;
        }
      });
    });
  }

  private async onReceivedFile(file: compilerApi.File): Promise<void> {
    console.log(chalk.blackBright(`→ ${normalize(file.getPath())}`));

    const path = resolve(this.rootDir, file.getPath());
    const data = Buffer.from(file.getData_asU8());

    await createDir(dirname(path));
    await writeFile(path, data);
  }
}
