import { basename, resolve } from "path";
import { Driver, Program } from "~/compiler/driver";
import { CompilerClient } from "~/compiler/proto/compiler_grpc_pb";
import { exists } from "~/base/node/fs";
import { readFile, writeFile } from "fs/promises";
import { FileSystem } from "~/compiler/driver";
import {
  BuildConfigEvent,
  BuildRequestEvent,
} from "~/compiler/proto/compiler_pb";
import * as compilerApi from "~/compiler/proto/compiler_pb";
import { glob } from "glob";

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
    const fs = this.program.getFs();
    const paths = await this.collectSrcFiles();
    const cwd = this.program.getRootDir();

    return new Promise<void>(async (_resolve, _reject) => {
      const stream = this.client.build();

      stream.on("data", async (e: compilerApi.BuildResponseEvent) => {
        if (e.hasEnd()) {
          return _resolve();
        }
        if (e.hasLog()) {
          const message = e.getLog()!.getMessage();
          if (message != null) {
            console.log(message);
          }
          return;
        }
        if (e.hasFile()) {
          const file = e.getFile()!.getFile()!;
          const buffer = Buffer.from(file.getData());
          const path = resolve(cwd, file.getPath());
          await fs.write(path, buffer);
          return;
        }
      });

      {
        const e = new BuildRequestEvent().setConfig(
          new BuildConfigEvent()
            .setQpcConfig(JSON.stringify(this.program.getConfig()))
            .setTarget("gowno"),
        );
        stream.write(e);
      }

      await Promise.all(
        paths.map(async (path) => {
          const data = await fs.read(path);
          const e = new BuildRequestEvent().setFile(
            new compilerApi.FileEvent().setFile(
              new compilerApi.File().setPath(path).setData(data),
            ),
          );
          stream.write(e);
        }),
      );
    });
  }

  public async check(): Promise<void> {}
}
