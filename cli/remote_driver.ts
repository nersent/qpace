import { readFile, writeFile } from "fs/promises";
import { basename, resolve } from "path";

import { ClientReadableStream } from "@grpc/grpc-js";
import chalk from "chalk";
import { glob } from "glob";

import { exec } from "../base/node/exec";
import { prettifyTime } from "../base/node/time";
import { Config } from "../compiler/config";

import { exists } from "~/base/node/fs";
import { Driver, Program, FileSystem } from "~/compiler/driver";
import { CompilerClient } from "~/compiler/proto/compiler_grpc_pb";
import { BuildRequest } from "~/compiler/proto/compiler_pb";
import * as compilerApi from "~/compiler/proto/compiler_pb";

export class OsFileSystem implements FileSystem {
  constructor(public readonly rootDir: string) {}

  public resolve(path: string): string {
    return resolve(this.rootDir, path);
  }

  public async exists(path: string): Promise<boolean> {
    path = this.resolve(path);
    return exists(path);
  }

  public read(path: string): Promise<Buffer>;
  public read(path: string, encoding: "utf8"): Promise<string>;
  public read(path: string, encoding?: "utf8"): Promise<Buffer | string> {
    path = this.resolve(path);
    return readFile(path, encoding);
  }

  public write(
    path: string,
    data: Buffer | string,
    encoding?: "utf8",
  ): Promise<void> {
    path = this.resolve(path);
    return writeFile(path, data, encoding);
  }
}

export class RemoteDriver implements Driver {
  constructor(
    public readonly program: Program,
    public readonly client: CompilerClient,
  ) {}

  public async collectSrcFiles(): Promise<string[]> {
    const cwd = this.program.getRootDir();
    const config = this.program.getConfig();
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

  public async build(): Promise<void> {
    const cwd = this.program.getRootDir();
    console.log(chalk.blackBright(`Building ${cwd}`));

    const req = await this.createBuildRequest();
    const stream = this.client.build(req);

    const startTime = Date.now();
    const res = await this.resolveBuild(stream);
    const endTime = Date.now();

    this.logEnd({ status: res.getStatus(), startTime, endTime });
  }

  public async check(): Promise<void> {
    const cwd = this.program.getRootDir();
    console.log(chalk.blackBright(`Checking ${cwd}`));

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
    const config = this.program.getConfig();
    const fs = this.program.getFs();
    const paths = await this.collectSrcFiles();
    console.log(chalk.blackBright(paths.map((r) => `→ ${r}`).join("\n")));
    const req = new BuildRequest();
    req.setQpcConfig(JSON.stringify(config));
    const reqFiles: compilerApi.File[] = [];
    reqFiles.push(
      ...(await Promise.all(
        paths.map(async (path) => {
          const data = await fs.read(path);
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
    const fs = this.program.getFs();
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
            res.getFilesList().map(async (file) => {
              const path = file.getPath();
              await fs.write(path, Buffer.from(file.getData_asU8()));
              console.log(chalk.blackBright(`← ${path}`));
            }),
          );
          _resolve(res);
          return;
        }
      });
    });
  }

  private async installPipWheel(path: string): Promise<void> {
    await exec({
      command: `pip install "${path}" --force-reinstall`,
      io: true,
    });
  }
}
