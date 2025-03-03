import { resolve } from "path";

import axios from "axios";
import chalk from "chalk";

import { ApiClient } from "./api_client";

import {
  Driver as DriverBase,
  DriverBuildOptions,
  Program,
  TargetSpec,
} from "~/pace/compiler/schema/driver";
import * as compilerApi from "~/pace/platform/schema/compiler_api_pb";

export class RemoteDriver extends DriverBase {
  private readonly apiClient: ApiClient;

  constructor(program: Program, deps: { apiClient: ApiClient }) {
    super(program);
    this.apiClient = deps.apiClient;
  }

  private async collectFiles(): Promise<compilerApi.File[]> {
    const files: compilerApi.File[] = [];
    files.push(
      ...(await Promise.all(
        this.program.host.rootNames.map(async (filename) => {
          const buffer = await this.program.host.readFile(filename);
          if (buffer == null) throw new Error(`Could not read ${filename}`);
          return new compilerApi.File().setFilename(filename).setData(buffer);
        }),
      )),
    );
    return files;
  }

  private async prepareGrpcReq<T extends compilerApi.BuildRequest>(
    req: T,
  ): Promise<T> {
    const files = await this.collectFiles();
    const options = JSON.stringify(this.program.options);
    return req.setFilesList(files).setOptions(options) as T;
  }

  public async build(opts?: DriverBuildOptions): Promise<boolean> {
    return new Promise<boolean>(async (resolve, reject) => {
      const targetSpec =
        opts?.targetSpec != null
          ? new compilerApi.TargetSpecifier()
              .setArch(opts.targetSpec.arch)
              .setOs(opts.targetSpec.os)
              .setTarget(opts.targetSpec.target)
          : undefined;
      const stream = this.apiClient.compilerApi.build(
        await this.prepareGrpcReq(
          new compilerApi.BuildRequest()
            .setCheckOnly(opts?.checkOnly ?? false)
            .setTargetSpec(targetSpec),
        ),
      );
      stream.on("data", async (e: compilerApi.BuildResponseEvent) => {
        if (e.getLog() != null) {
          opts?.logger?.log(e.getLog()?.getMessage());
          return;
        }
        if (e.getFile() != null) {
          const file = e.getFile()!.getFile()!;
          const filename = file.getFilename();
          opts?.logger?.log(chalk.blackBright(`-> ${filename}`));
          await this.program.host.writeFile(
            filename,
            Buffer.from(file.getData()),
          );
        }
        if (e.getEnd() != null) {
          return resolve(e.getEnd()!.getOk());
        }
      });
    });
  }
}
