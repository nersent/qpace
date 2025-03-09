import { readFile, writeFile } from "fs/promises";
import { basename, dirname, normalize, resolve } from "path";

import { ClientReadableStream } from "@grpc/grpc-js";
import chalk from "chalk";
import { glob } from "glob";

import { exec } from "../base/node/exec";
import { prettifyTime } from "../base/node/time";

import { createDir } from "~/base/node/fs";
import { Config, FileTag, Target } from "~/compiler/common";
import { CompilerClient } from "~/compiler/proto/compiler_grpc_pb";
import { BuildRequest } from "~/compiler/proto/compiler_pb";
import * as compilerApi from "~/compiler/proto/compiler_pb";

export interface RemoteDriverOpts {
  config: Config;
  client: CompilerClient;
  rootDir: string;
}

export class RemoteDriver {
  public readonly config: Config;
  public readonly client: CompilerClient;
  public readonly rootDir: string;
  private emittedPythonWheel = false;

  constructor(opts: RemoteDriverOpts) {
    this.config = opts.config;
    this.client = opts.client;
    this.rootDir = opts.rootDir;
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
    const req = await this.createBuildRequest();
    if (target != null) {
      req.setTarget(target);
    }
    const stream = this.client.build(req);

    const startTime = Date.now();
    const res = await this.resolveBuild(stream);
    const endTime = Date.now();

    this.logEnd({ status: res.getStatus(), startTime, endTime });
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
    } else if (status === compilerApi.BuildStatus.OK) {
      console.log(
        chalk.green(`Finished in ${prettifyTime(endTime - startTime)}`),
      );
    }
  }

  private async createBuildRequest(): Promise<BuildRequest> {
    const paths = await this.collectSrcFiles();
    console.log(chalk.blackBright(paths.map((r) => `→ ${r}`).join("\n")));
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
        if (e.hasMessage()) {
          const message = e.getMessage();
          if (message.length) {
            console.log(message);
          }
        }
        if (e.hasResponse()) {
          const res = e.getResponse()!;
          await Promise.all(
            res.getFilesList().map((r) => this.onReceivedFile(r)),
          );
          _resolve(res);
          return;
        }
      });
    });
  }

  private async onReceivedFile(file: compilerApi.File): Promise<void> {
    console.log(chalk.blackBright(`← ${normalize(file.getPath())}`));

    const path = resolve(this.rootDir, file.getPath());
    const tags = file.getTagsList();
    const data = Buffer.from(file.getData_asU8());

    await createDir(dirname(path));
    await writeFile(path, data);

    if (tags.includes(FileTag.QPC_PYTHON_WHEEL)) {
      this.emittedPythonWheel = true;
      await this.installPipWheel(path);
      return;
    }
  }

  private async installPipWheel(path: string): Promise<void> {
    console.log(`Installing python wheel`);
    await exec({
      command: `pip install "${path}" --force-reinstall`,
      io: true,
    });
  }
}
