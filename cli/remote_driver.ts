import { basename, resolve } from "path";
import { Driver, Program } from "~/compiler/driver";
import { CompilerClient } from "~/compiler/proto/compiler_grpc_pb";
import { exists } from "~/base/node/fs";
import { readFile, writeFile } from "fs/promises";
import { FileSystem } from "~/compiler/driver";
import { BuildRequest } from "~/compiler/proto/compiler_pb";
import * as compilerApi from "~/compiler/proto/compiler_pb";
import { glob } from "glob";
import chalk from "chalk";

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
    const fs = this.program.getFs();
    const paths = await this.collectSrcFiles();

    for (const path of paths) {
      console.log(chalk.blackBright(`-> ${path}`));
    }

    const req = new BuildRequest();
    req.setQpcConfig(JSON.stringify(this.program.getConfig()));
    req.setTarget("gowno");
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

    const stream = this.client.build(req);

    await new Promise<void>((_resolve, _reject) => {
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
              await fs.write(file.getPath(), Buffer.from(file.getData_asU8()));
            }),
          );
          _resolve();
          return;
        }
      });
    });

    console.log(chalk.greenBright("Done"));
  }

  public async check(): Promise<void> {}
}
